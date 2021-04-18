/// 乘除指令
use crate::{cpu::Cpu, trap::Exception, Executable, Format, Insn, RegT, SRegT, INSN_SLICE};
use proc_macros::Instruction;

use super::sext;

def_insn!(
  #[derive(Instruction)]
  #[format(R)]
  #[match_code(0x2000033)]
  #[mask(0xfe00707f)]
  ,Mul);

impl Executable for Mul {
    // x[rd] = x[rs1] × x[rs2]
    // 乘(Multiply). R-type, RV32M and RV64M.
    // 把寄存器 x[rs2]乘到寄存器 x[rs1]上，乘积写入 x[rd]。忽略算术溢出。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let rs2 = cpu.state.xs.reg(self.rs2() as u8);
        cpu.state
            .xs
            .set_reg(self.rd() as u8, rs1.wrapping_mul(rs2) & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(R)]
  #[match_code(0x2001033)]
  #[mask(0xfe00707f)]
  ,Mulh);

impl Executable for Mulh {
    // x[rd] = (x[rs1] 𝑠 ×𝑠 x[rs2]) ≫𝑠 XLEN
    //  高位乘(Multiply High). R-type, RV32M and RV64M.
    // 把寄存器 x[rs2]乘到寄存器 x[rs1]上，都视为 2 的补码，将乘积的高位写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8) as SRegT as i128;
        let rs2 = cpu.state.xs.reg(self.rs2() as u8) as SRegT as i128;

        let value = rs1.wrapping_mul(rs2).wrapping_shr(cpu.xlen.len() as u32);
        cpu.state
            .xs
            .set_reg(self.rd() as u8, (value as RegT) & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(R)]
  #[match_code(0x2002033)]
  #[mask(0xfe00707f)]
  ,Mulhsu);

impl Executable for Mulhsu {
    // x[rd] = (x[rs1] 𝑠 ×𝑢 x[rs2]) ≫𝑠 XLEN
    // 高位有符号-无符号乘(Multiply High Signed-Unsigned). R-type, RV32M and RV64M.
    // 把寄存器 x[rs2]乘到寄存器 x[rs1]上，x[rs1]为 2 的补码，x[rs2]为无符号数，将乘积的高位写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8) as SRegT as i128 as u128;
        let rs2 = cpu.state.xs.reg(self.rs2() as u8) as u128;
        let value = rs1.wrapping_mul(rs2).wrapping_shr(cpu.xlen.len() as u32);
        cpu.state
            .xs
            .set_reg(self.rd() as u8, (value as RegT) & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(R)]
  #[match_code(0x2003033)]
  #[mask(0xfe00707f)]
  ,Mulhu);

impl Executable for Mulhu {
    // x[rd] = (x[rs1] 𝑢 ×𝑢 x[rs2]) ≫𝑢 XLEN
    // 高位无符号乘(Multiply High Unsigned). R-type, RV32M and RV64M.
    // 把寄存器 x[rs2]乘到寄存器 x[rs1]上，x[rs1]、x[rs2]均为无符号数，将乘积的高位写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8) as u128;
        let rs2 = cpu.state.xs.reg(self.rs2() as u8) as u128;
        let value = rs1.wrapping_mul(rs2).wrapping_shr(cpu.xlen.len() as u32);
        cpu.state
            .xs
            .set_reg(self.rd() as u8, (value as RegT) & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(R)]
  #[match_code(0x2004033)]
  #[mask(0xfe00707f)]
  ,Div);

impl Executable for Div {
    // x[rd] = x[rs1] ÷s x[rs2]
    // 除法(Divide). R-type, RV32M and RV64M.
    // 用寄存器 x[rs1]的值除以寄存器 x[rs2]的值，向零舍入，将这些数视为二进制补码，把商写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8) as SRegT;
        let rs2 = cpu.state.xs.reg(self.rs2() as u8) as SRegT;

        let value = if rs2 == 0 {
            (-1 as SRegT) as RegT
        } else {
            rs1.wrapping_div(rs2) as RegT
        };
        cpu.state
            .xs
            .set_reg(self.rd() as u8, value & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(R)]
  #[match_code(0x2005033)]
  #[mask(0xfe00707f)]
  ,Divu);

impl Executable for Divu {
    // x[rd] = x[rs1] ÷u x[rs2]
    // 无符号除法(Divide, Unsigned). R-type, RV32M and RV64M.
    // 用寄存器 x[rs1]的值除以寄存器 x[rs2]的值，向零舍入，将这些数视为无符号数，把商写入x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let rs2 = cpu.state.xs.reg(self.rs2() as u8);

        let value = if rs2 == 0 {
            (-1 as SRegT) as RegT
        } else {
            rs1.wrapping_div(rs2)
        };
        cpu.state
            .xs
            .set_reg(self.rd() as u8, value & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(R)]
  #[match_code(0x2006033)]
  #[mask(0xfe00707f)]
  ,Rem);

impl Executable for Rem {
    // x[rd] = x[rs1] %𝑠 x[rs2]
    // 求余数(Remainder). R-type, RV32M and RV64M.
    // x[rs1]除以 x[rs2]，向 0 舍入，都视为 2 的补码，余数写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8) as SRegT;
        let rs2 = cpu.state.xs.reg(self.rs2() as u8) as SRegT;
        let value = if rs2 == 0 {
            rs1 as RegT
        } else {
            (rs1.wrapping_rem(rs2) as RegT) & cpu.xlen.mask()
        };

        cpu.state.xs.set_reg(self.rd() as u8, value);
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(R)]
  #[match_code(0x2007033)]
  #[mask(0xfe00707f)]
  ,Remu);

impl Executable for Remu {
    // x[rd] = x[rs1] %𝑢 x[rs2]
    // 求无符号数的余数(Remainder, Unsigned). R-type, RV32M and RV64M.
    // x[rs1]除以 x[rs2]，向 0 舍入，都视为无符号数，余数写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let rs2 = cpu.state.xs.reg(self.rs2() as u8);
        let value = if rs2 == 0 {
            rs1 as RegT
        } else {
            (rs1.wrapping_rem(rs2) as RegT) & cpu.xlen.mask()
        };
        cpu.state.xs.set_reg(self.rd() as u8, value);
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(R)]
    #[match_code(0x200703b)]
    #[mask(0xfe00707f)]
    ,Remuw);

impl Executable for Remuw {
    // x[rd] = sext(x[rs1][31: 0] %𝑢 x[rs2][31: 0])
    // 求无符号数的余数字(Remainder Word, Unsigned). R-type, RV64M only.
    // x[rs1]的低 32 位除以 x[rs2]的低 32 位，向 0 舍入，都视为无符号数，将余数的有符号扩展
    // 写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8) & 0xffff_ffff;
        let rs2 = cpu.state.xs.reg(self.rs2() as u8) & 0xffff_ffff;
        let value = if rs2 == 0 {
            sext(rs1, 32)
        } else {
            sext(rs1.wrapping_rem(rs2) as RegT, 32)
        };
        cpu.state
            .xs
            .set_reg(self.rd() as u8, value & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(R)]
    #[match_code(0x200503b)]
    #[mask(0xfe00707f)]
    ,Divuw);

impl Executable for Divuw {
    // x[rd] = sext(x[rs1][31:0] ÷u x[rs2][31:0])
    // 无符号字除法(Divide Word, Unsigned). R-type, RV64M.
    // 用寄存器 x[rs1]的低 32 位除以寄存器 x[rs2]的低 32 位，向零舍入，将这些数视为无符号数，
    // 把经符号位扩展的 32 位商写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8) & 0xffff_ffff;
        let rs2 = cpu.state.xs.reg(self.rs2() as u8) & 0xffff_ffff;

        let value = if rs2 == 0 {
            (-1 as SRegT) as RegT
        } else {
            rs1.wrapping_div(rs2)
        };
        cpu.state
            .xs
            .set_reg(self.rd() as u8, sext(value, 32) & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}
