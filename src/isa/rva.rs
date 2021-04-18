/// 原子指令
use crate::{cpu::Cpu, trap::Exception, Executable, Format, Insn, RegT, INSN_SLICE};
use proc_macros::Instruction;

use super::sext;

def_insn!(
  #[derive(Instruction)]
  #[format(R)]
  #[match_code(0x1000202f)]
  #[mask(0xf9f0707f)]
  ,LrW);

impl Executable for LrW {
    // x[rd] = LoadReserved32(M[x[rs1]])
    // 加载保留字(Load-Reserved Word). R-type, RV32A and RV64A.
    // 从内存中地址为 x[rs1]中加载四个字节，符号位扩展后写入 x[rd]，并对这个内存字注册保留。
    fn exec(&self, _cpu: &mut Cpu) -> Result<(), Exception> {
        todo!()
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(R)]
  #[match_code(0x1800202f)]
  #[mask(0xf800707f)]
  ,ScW);

impl Executable for ScW {
    // x[rd] = StoreConditonal32(M[x[rs1], x[rs2])
    // 条件存入字(Store-Conditional Word). R-type, RV32A and RV64A.
    // 内存地址 x[rs1]上存在加载保留，将 x[rs2]寄存器中的 4 字节数存入该地址。
    // 如果存入成功，向寄存器 x[rd]中存入 0，否则存入一个非 0 的错误码。
    fn exec(&self, _cpu: &mut Cpu) -> Result<(), Exception> {
        todo!()
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(R)]
  #[match_code(0x800202f)]
  #[mask(0xf800707f)]
  ,AmoswapW);

impl Executable for AmoswapW {
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let addr = cpu.state.xs.reg(self.rs1() as u8);
        let src = cpu.state.xs.reg(self.rs2() as u8);
        if addr % 4 != 0 {
            return Err(Exception::LoadMisaligned);
        }
        let value = cpu.mmu.load::<u32>(&cpu.state, addr)? as RegT;
        let value = sext(value, 32);
        cpu.mmu.store::<u32>(&cpu.state, addr, src as u32)?;
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
  #[match_code(0x202f)]
  #[mask(0xf800707f)]
  ,AmoaddW);

impl Executable for AmoaddW {
    // x[rd] = AMO32(M[x[rs1]] + x[rs2])
    // 原子加字(Atomic Memory Operation: Add Word). R-type, RV32A and RV64A.
    // 进行如下的原子操作：将内存中地址为 x[rs1]中的字记为 t，把这个字变为 t+x[rs2]，把 x[rd]设为符号位扩展的 t。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let addr = cpu.state.xs.reg(self.rs1() as u8);
        let src = cpu.state.xs.reg(self.rs2() as u8) as u32;
        let value = cpu.mmu.load::<u32>(&cpu.state, addr)?;
        cpu.mmu
            .store::<u32>(&cpu.state, addr, (src.wrapping_add(value)) as u32)?;
        let value = sext(value as RegT, 32);
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
  #[match_code(0x2000202f)]
  #[mask(0xf800707f)]
  ,AmoxorW);

impl Executable for AmoxorW {
    // x[rd] = AMO32(M[x[rs1]] ^ x[rs2])
    // 原子字异或 (Atomic Memory Operation: XOR Word). R-type, RV32A and RV64A.
    // 进行如下的原子操作：将内存中地址为 x[rs1]中的字记为 t，把这个字变为 t 和 x[rs2]按位异
    // 或的结果，把 x[rd]设为符号位扩展的 t。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let addr = cpu.state.xs.reg(self.rs1() as u8);
        let src = cpu.state.xs.reg(self.rs2() as u8) as u32;
        let value = cpu.mmu.load::<u32>(&cpu.state, addr)?;
        cpu.mmu.store::<u32>(&cpu.state, addr, src ^ value)?;
        let value = sext(value as RegT, 32);
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
  #[match_code(0x6000202f)]
  #[mask(0xf800707f)]
  ,AmoandW);

impl Executable for AmoandW {
    // x[rd] = AMO32(M[x[rs1]] & x[rs2])
    // 原子字与 (Atomic Memory Operation: AND Word). R-type, RV32A and RV64A.
    // 进行如下的原子操作：将内存中地址为 x[rs1]中的字记为 t，把这个字变为 t 和 x[rs2]位与的
    // 结果，把 x[rd]设为符号位扩展的 t。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let addr = cpu.state.xs.reg(self.rs1() as u8);
        let src = cpu.state.xs.reg(self.rs2() as u8) as u32;
        let value = cpu.mmu.load::<u32>(&cpu.state, addr)?;
        cpu.mmu.store::<u32>(&cpu.state, addr, src & value)?;
        let value = sext(value as RegT, 32);
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
  #[match_code(0x4000202f)]
  #[mask(0xf800707f)]
  ,AmoorW);

impl Executable for AmoorW {
    // x[rd] = AMO32(M[x[rs1]] | x[rs2])
    // 原子字或 (Atomic Memory Operation: OR Word). R-type, RV32A and RV64A.
    // 进行如下的原子操作：将内存中地址为 x[rs1]中的字记为 t，把这个字变为 t 和 x[rs2]位或的
    // 结果，把 x[rd]设为符号位扩展的 t。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let addr = cpu.state.xs.reg(self.rs1() as u8);
        let src = cpu.state.xs.reg(self.rs2() as u8) as u32;
        let value = cpu.mmu.load::<u32>(&cpu.state, addr)?;
        cpu.mmu.store::<u32>(&cpu.state, addr, src | value)?;
        let value = sext(value as RegT, 32);
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
  #[match_code(0x8000202f)]
  #[mask(0xf800707f)]
  ,AmominW);

impl Executable for AmominW {
    // x[rd] = AMO32(M[x[rs1]] MIN x[rs2])
    // 原子最小字(Atomic Memory Operation: Minimum Word). R-type, RV32A and RV64A.
    // 进行如下的原子操作：将内存中地址为 x[rs1]中的字记为 t，把这个字变为 t 和 x[rs2]中较小
    // 的一个（用二进制补码比较），把 x[rd]设为符号位扩展的 t。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let addr = cpu.state.xs.reg(self.rs1() as u8);
        let src = cpu.state.xs.reg(self.rs2() as u8) as u32 as i32;
        let value = cpu.mmu.load::<u32>(&cpu.state, addr)?;
        cpu.mmu
            .store::<u32>(&cpu.state, addr, std::cmp::min(src, value as i32) as u32)?;
        let value = sext(value as RegT, 32);
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
  #[match_code(0xa000202f)]
  #[mask(0xf800707f)]
  ,AmomaxW);

impl Executable for AmomaxW {
    // x[rd] = AMO32(M[x[rs1]] MAX x[rs2])
    // 原子最大字(Atomic Memory Operation: Maximum Word). R-type, RV32A and RV64A.
    // 进行如下的原子操作：将内存中地址为 x[rs1]中的字记为 t，把这个字变为 t 和 x[rs2]中较大
    // 的一个（用二进制补码比较），把 x[rd]设为符号位扩展的 t。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let addr = cpu.state.xs.reg(self.rs1() as u8);
        let src = cpu.state.xs.reg(self.rs2() as u8) as u32 as i32;
        let value = cpu.mmu.load::<u32>(&cpu.state, addr)?;
        cpu.mmu
            .store::<u32>(&cpu.state, addr, std::cmp::max(src, value as i32) as u32)?;
        let value = sext(value as RegT, 32);
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
  #[match_code(0xc000202f)]
  #[mask(0xf800707f)]
  ,AmominuW);

impl Executable for AmominuW {
    // x[rd] = AMO32(M[x[rs1]] MINU x[rs2])
    // 原子无符号最大字(Atomic Memory Operation: Minimum Word, Unsigned). R-type, RV32A and RV64A.
    // 进行如下的原子操作：将内存中地址为 x[rs1]中的字记为 t，把这个字变为 t 和 x[rs2]中较小
    // 的一个（用无符号比较），把 x[rd]设为符号位扩展的 t。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let addr = cpu.state.xs.reg(self.rs1() as u8);
        let src = cpu.state.xs.reg(self.rs2() as u8);
        let value = cpu.mmu.load::<u32>(&cpu.state, addr)? as RegT;
        cpu.mmu
            .store::<u32>(&cpu.state, addr, std::cmp::min(src, value) as u32)?;
        let value = sext(value, 32);
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
  #[match_code(0xe000202f)]
  #[mask(0xf800707f)]
  ,AmomaxuW);

impl Executable for AmomaxuW {
    // x[rd] = AMO32(M[x[rs1]] MAXU x[rs2])
    // 原子无符号最大字(Atomic Memory Operation: Maximum Word, Unsigned). R-type, RV32A and RV64A.
    // 进行如下的原子操作：将内存中地址为 x[rs1]中的字记为 t，把这个字变为 t 和 x[rs2]中较大
    // 的一个（用无符号比较），把 x[rd]设为符号位扩展的 t。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let addr = cpu.state.xs.reg(self.rs1() as u8);
        let src = cpu.state.xs.reg(self.rs2() as u8);
        let value = cpu.mmu.load::<u32>(&cpu.state, addr)? as RegT;
        cpu.mmu
            .store::<u32>(&cpu.state, addr, std::cmp::max(src, value) as u32)?;
        let value = sext(value, 32);
        cpu.state
            .xs
            .set_reg(self.rd() as u8, value & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}
