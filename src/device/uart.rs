use std::{
    io::{Read, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex,
    },
    thread,
};

use crate::trap::Exception;

use super::{Data, Device, UART_BASE, UART_SIZE};

/// The interrupt request of UART.
pub const UART_IRQ: u64 = 10;
/// Receive holding register (for input bytes).
const UART_RHR: u64 = UART_BASE + 0;
/// Transmit holding register (for output bytes).
const UART_THR: u64 = UART_BASE + 0;
/// Line control register.
const _UART_LCR: u64 = UART_BASE + 3;
/// Line status register.
/// LSR BIT 0:
///     0 = no data in receive holding register or FIFO.
///     1 = data has been receive and saved in the receive holding register or FIFO.
/// LSR BIT 5:
///     0 = transmit holding register is full. 16550 will not accept any data for transmission.
///     1 = transmitter hold register (or FIFO) is empty. CPU can load the next character.
const UART_LSR: u64 = UART_BASE + 5;

/// The receiver (RX) bit.
const UART_LSR_RX: u8 = 1;
/// The transmitter (TX) bit.
const UART_LSR_TX: u8 = 1 << 5;

pub struct Uart {
    /// Pair of an array for UART buffer and a conditional variable.
    uart: Arc<(Mutex<[u8; UART_SIZE as usize]>, Condvar)>,
    /// Bit if an interrupt happens.
    interrupting: Arc<AtomicBool>,
}

impl Device for Uart {
    fn read<T>(&self, addr: u64) -> Result<T, Exception>
    where
        T: Data,
        [(); <T as Data>::SIZE]: Sized,
    {
        if T::SIZE != 1 {
            return Err(Exception::LoadFault);
        }
        let (uart, cvar) = &*self.uart;
        let mut uart = uart.lock().expect("failed to get an UART object");

        Ok(match addr {
            UART_RHR => {
                cvar.notify_one();
                uart[(UART_LSR - UART_BASE) as usize] &= !UART_LSR_RX;
                T::from_u8(uart[(UART_RHR - UART_BASE) as usize])
            }
            _ => T::from_u8(uart[(addr - UART_BASE) as usize]),
        })
    }

    fn write<T>(&mut self, addr: u64, value: T) -> Result<(), Exception>
    where
        T: Data,
        [(); <T as Data>::SIZE]: Sized,
    {
        if T::SIZE != 1 {
            return Err(Exception::StoreFault);
        }
        let (uart, _cvar) = &*self.uart;
        let mut uart = uart.lock().expect("failed to get an UART object");
        Ok(match addr {
            UART_THR => {
                print!("{}", value.to_u8() as char);
                std::io::stdout().flush().expect("failed to flush stdout");
            }
            _ => {
                uart[(addr - UART_BASE) as usize] = value.to_u8();
            }
        })
    }
}

impl Uart {
    pub fn new() -> Self {
        let uart = Arc::new((Mutex::new([0; UART_SIZE as usize]), Condvar::new()));
        let interrupting = Arc::new(AtomicBool::new(false));

        {
            let (uart, _cvar) = &*uart;
            let mut uart = uart.lock().expect("failed to get an UART object");
            // Transmitter hold register is empty.
            uart[(UART_LSR - UART_BASE) as usize] |= UART_LSR_TX;
        }
        let mut byte = [0; 1];
        let cloned_uart = uart.clone();
        let cloned_interrupting = interrupting.clone();
        thread::spawn(move || loop {
            match std::io::stdin().read(&mut byte) {
                Ok(_) => {
                    let (uart, cvar) = &*cloned_uart;
                    let mut uart = uart.lock().expect("failed to get an UART object");
                    // Wait for the thread to start up.
                    while (uart[(UART_LSR - UART_BASE) as usize] & UART_LSR_RX) == 1 {
                        uart = cvar.wait(uart).expect("the mutex is poisoned");
                    }
                    uart[0] = byte[0];
                    cloned_interrupting.store(true, Ordering::Release);
                    // Data has been receive.
                    uart[(UART_LSR - UART_BASE) as usize] |= UART_LSR_RX;
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        });
        Self {
            uart: uart,
            interrupting: interrupting,
        }
    }

    /// Return true if an interrupt is pending. Clear the interrupting flag by swapping a value.
    pub fn is_interrupting(&self) -> bool {
        self.interrupting.swap(false, Ordering::Acquire)
    }
}
