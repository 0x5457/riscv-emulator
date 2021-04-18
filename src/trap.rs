#[allow(dead_code)]
use crate::RegT;

/// Trap Cause
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Trap {
    Interrupt(Interrupt),
    Exception(Exception),
}

impl From<Exception> for Trap {
    fn from(e: Exception) -> Self {
        Trap::Exception(e)
    }
}

impl From<Interrupt> for Trap {
    fn from(i: Interrupt) -> Self {
        Trap::Interrupt(i)
    }
}

#[allow(dead_code)]
/// Interrupt
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    UserSoft,
    SupervisorSoft,
    MachineSoft,
    UserTimer,
    SupervisorTimer,
    MachineTimer,
    UserExternal,
    SupervisorExternal,
    MachineExternal,
    Unknown,
}

/// Exception
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Exception {
    InstructionMisaligned,
    InstructionFault,
    IllegalInstruction,
    Breakpoint,
    LoadMisaligned,
    LoadFault,
    StoreMisaligned,
    StoreFault,
    UserEnvCall,
    SupervisorEnvCall,
    MachineEnvCall,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
    Unknown,
}

impl Interrupt {
    pub fn code(&self) -> RegT {
        match self {
            Interrupt::UserSoft => 0,
            Interrupt::SupervisorSoft => 1,
            Interrupt::MachineSoft => 3,
            Interrupt::UserTimer => 4,
            Interrupt::SupervisorTimer => 5,
            Interrupt::MachineTimer => 7,
            Interrupt::UserExternal => 8,
            Interrupt::SupervisorExternal => 9,
            Interrupt::MachineExternal => 11,
            Interrupt::Unknown => 9999,
        }
    }
}

impl Exception {
    pub fn code(&self) -> RegT {
        match self {
            Exception::InstructionMisaligned => 0,
            Exception::InstructionFault => 1,
            Exception::IllegalInstruction => 2,
            Exception::Breakpoint => 3,
            Exception::LoadMisaligned => 4,
            Exception::LoadFault => 5,
            Exception::StoreMisaligned => 6,
            Exception::StoreFault => 7,
            Exception::UserEnvCall => 8,
            Exception::SupervisorEnvCall => 9,
            Exception::MachineEnvCall => 11,
            Exception::InstructionPageFault => 12,
            Exception::LoadPageFault => 13,
            Exception::StorePageFault => 15,
            Exception::Unknown => 8888,
        }
    }

    pub fn is_fatal(&self) -> bool {
        match self {
            Exception::InstructionFault
            | Exception::IllegalInstruction
            | Exception::InstructionMisaligned
            | Exception::LoadFault
            | Exception::StorePageFault
            | Exception::StoreMisaligned => true,
            _ => false,
        }
    }
}
