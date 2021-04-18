/// 基础整数指令集
use crate::{
    cpu::Cpu, trap::Exception, Executable, Format, Insn, PrivilegeMode, RegT, SRegT, XLen,
    INSN_SLICE,
};
use bit_field::BitField;
use proc_macros::Instruction;

use super::sext;

def_insn!(
  #[derive(Instruction)]
  #[format(U)]
  #[match_code(0x37)]
  #[mask(0x7f)]
  ,Lui);

impl Executable for Lui {
    // x[rd] = sext(immediate[31:12] << 12)
    // 高位立即数加载 (Load Upper Immediate). U-type, RV32I and RV64I.
    // 将符号位扩展的 20 位立即数 immediate 左移 12 位，并将低 12 位置零，写入 x[rd]中。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        cpu.state.xs.set_reg(
            self.rd() as u8,
            sext(self.imm() as RegT, self.imm_len()) & cpu.xlen.mask(),
        );
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(U)]
  #[match_code(0x17)]
  #[mask(0x7f)]
  ,Auipc);

impl Executable for Auipc {
    // x[rd] = pc + sext(immediate[31:12] << 12)
    // PC 加立即数 (Add Upper Immediate to PC). U-type, RV32I and RV64I.
    // 把符号位扩展的 20 位（左移 12 位）立即数加到 pc 上，结果写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let offset_sext = sext(self.imm() as RegT, self.imm_len());
        cpu.state.xs.set_reg(
            self.rd() as u8,
            cpu.state.pc.wrapping_add(offset_sext) & cpu.xlen.mask(),
        );
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(J)]
  #[match_code(0x6f)]
  #[mask(0x7f)]
  ,Jal);

impl Executable for Jal {
    // x[rd] = pc+4; pc += sext(offset)
    // 跳转并链接 (Jump and Link). J-type, RV32I and RV64I.
    // 把下一条指令的地址(pc+4)，然后把 pc 设置为当前值加上符号位扩展的offset。rd 默认为 x1。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let offset_sext = sext(self.imm() as RegT, self.imm_len());
        cpu.state.xs.set_reg(self.rd() as u8, cpu.state.pc + 4);
        cpu.state
            .update_pc(cpu.state.pc.wrapping_add(offset_sext) & cpu.xlen.mask());
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(I)]
  #[match_code(0x67)]
  #[mask(0x707f)]
  ,Jalr);

impl Executable for Jalr {
    // t=pc+4; pc=(x[rs1]+sext(offset))&~1; x[rd]=t
    // 跳转并寄存器链接 (Jump and Link Register). I-type, RV32I and RV64I.
    // 把 pc 设置为 x[rs1] + sign-extend(offset)，把计算出的地址的最低有效位设为 0，并将原 pc+4的值写入 f[rd]。rd 默认为 x1。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let offset_sext = sext(self.imm() as RegT, self.imm_len());
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let t = cpu.state.pc + 4;
        cpu.state.update_pc(rs1.wrapping_add(offset_sext) & !1);
        cpu.state.xs.set_reg(self.rd() as u8, t);
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(B)]
  #[match_code(0x63)]
  #[mask(0x707f)]
  ,Beq);

impl Executable for Beq {
    // if (rs1 == rs2) pc += sext(offset)
    // 相等时分支 (Branch if Equal). B-type, RV32I and RV64I.
    // 若寄存器 x[rs1]和寄存器 x[rs2]的值相等，把 pc 的值设为当前值加上符号位扩展的偏移 offset。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let rs2 = cpu.state.xs.reg(self.rs2() as u8);
        let offset_sext = sext(self.imm() as RegT, self.imm_len());
        if rs1 == rs2 {
            cpu.state.update_pc(cpu.state.pc.wrapping_add(offset_sext));
        } else {
            cpu.state.update_pc(cpu.state.pc + 4);
        }
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(B)]
  #[match_code(0x1063)]
  #[mask(0x707f)]
  ,Bne);

impl Executable for Bne {
    // if (rs1 ≠ rs2) pc += sext(offset)
    // 不相等时分支 (Branch if Not Equal). B-type, RV32I and RV64I.
    // 若寄存器 x[rs1]和寄存器 x[rs2]的值不相等，把 pc 的值设为当前值加上符号位扩展的偏移offset。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let rs2 = cpu.state.xs.reg(self.rs2() as u8);
        let offset_sext = sext(self.imm() as RegT, self.imm_len());
        if rs1 != rs2 {
            cpu.state.update_pc(cpu.state.pc.wrapping_add(offset_sext));
        } else {
            cpu.state.update_pc(cpu.state.pc + 4);
        }
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(B)]
  #[match_code(0x4063)]
  #[mask(0x707f)]
  ,Blt);

impl Executable for Blt {
    // if (rs1 <s rs2) pc += sext(offset)
    // 小于时分支 (Branch if Less Than). B-type, RV32I and RV64I.
    // 若寄存器 x[rs1]的值小于寄存器 x[rs2]的值（均视为二进制补码），把 pc 的值设为当前值加上符号位扩展的偏移 offset。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8) as SRegT;
        let rs2 = cpu.state.xs.reg(self.rs2() as u8) as SRegT;
        let offset_sext = sext(self.imm() as RegT, self.imm_len());

        if rs1 < rs2 {
            cpu.state.update_pc(cpu.state.pc.wrapping_add(offset_sext));
        } else {
            cpu.state.update_pc(cpu.state.pc + 4);
        }
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(B)]
  #[match_code(0x5063)]
  #[mask(0x707f)]
  ,Bge);

impl Executable for Bge {
    // if (rs1 ≥s rs2) pc += sext(offset)
    // 大于等于时分支 (Branch if Greater Than or Equal). B-type, RV32I and RV64I.
    // 若寄存器 x[rs1]的值大于等于寄存器 x[rs2]的值（均视为二进制补码），把 pc 的值设为当前值加上符号位扩展的偏移 offset。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8) as SRegT;
        let rs2 = cpu.state.xs.reg(self.rs2() as u8) as SRegT;
        let offset_sext = sext(self.imm() as RegT, self.imm_len());

        if rs1 >= rs2 {
            cpu.state.update_pc(cpu.state.pc.wrapping_add(offset_sext));
        } else {
            cpu.state.update_pc(cpu.state.pc + 4);
        }
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(B)]
  #[match_code(0x6063)]
  #[mask(0x707f)]
  ,Bltu);

impl Executable for Bltu {
    // if (rs1 <u rs2) pc += sext(offset)
    // 无符号小于时分支 (Branch if Less Than, Unsigned). B-type, RV32I and RV64I.
    // 若寄存器 x[rs1]的值小于寄存器 x[rs2]的值（均视为无符号数），把 pc 的值设为当前值加上符号位扩展的偏移 offset。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let rs2 = cpu.state.xs.reg(self.rs2() as u8);
        let offset_sext = sext(self.imm() as RegT, self.imm_len());

        if rs1 < rs2 {
            cpu.state.update_pc(cpu.state.pc.wrapping_add(offset_sext));
        } else {
            cpu.state.update_pc(cpu.state.pc + 4);
        }
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(B)]
  #[match_code(0x7063)]
  #[mask(0x707f)]
  ,Bgeu);

impl Executable for Bgeu {
    // if (rs1 ≥u rs2) pc += sext(offset)
    //  无符号大于等于时分支 (Branch if Greater Than or Equal, Unsigned). B-type, RV32I and RV64I.
    //若寄存器 x[rs1]的值大于等于寄存器 x[rs2]的值（均视为无符号数），把 pc 的值设为当前值加上符号位扩展的偏移 offset。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let rs2 = cpu.state.xs.reg(self.rs2() as u8);
        let offset_sext = sext(self.imm() as RegT, self.imm_len());

        if rs1 >= rs2 {
            cpu.state.update_pc(cpu.state.pc.wrapping_add(offset_sext));
        } else {
            cpu.state.update_pc(cpu.state.pc + 4);
        }
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(I)]
  #[match_code(0x3)]
  #[mask(0x707f)]
  ,Lb);

impl Executable for Lb {
    // x[rd] = sext(M[x[rs1] + sext(offset)][7:0])
    // 字节加载 (Load Byte). I-type, RV32I and RV64I.
    // 从地址 x[rs1] + sign-extend(offset)读取一个字节，经符号位扩展后写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);

        let offset_sext = sext(self.imm() as RegT, self.imm_len());
        let data = cpu
            .mmu
            .load::<u8>(&cpu.state, rs1.wrapping_add(offset_sext))?;
        let value = sext(data as RegT, 8) & cpu.xlen.mask();
        cpu.state.xs.set_reg(self.rd() as u8, value);
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(I)]
  #[match_code(0x1003)]
  #[mask(0x707f)]
  ,Lh);

impl Executable for Lh {
    // x[rd] = sext(M[x[rs1] + sext(offset)][15:0])
    // 半字加载 (Load Halfword). I-type, RV32I and RV64I.
    // 从地址 x[rs1] + sign-extend(offset)读取两个字节，经符号位扩展后写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);

        let offset_sext = sext(self.imm() as RegT, self.imm_len());
        let data = cpu
            .mmu
            .load::<u16>(&cpu.state, rs1.wrapping_add(offset_sext))?;
        let value = sext(data as RegT, 16) & cpu.xlen.mask();
        cpu.state.xs.set_reg(self.rd() as u8, value);
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(I)]
  #[match_code(0x2003)]
  #[mask(0x707f)]
  ,Lw);

impl Executable for Lw {
    // x[rd] = sext(M[x[rs1] + sext(offset)][31:0])
    // 字加载 (Load Word). I-type, RV32I and RV64I.
    // 从地址 x[rs1] + sign-extend(offset)读取四个字节，写入 x[rd]。对于 RV64I，结果要进行符号位扩展。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);

        let offset_sext = sext(self.imm() as RegT, self.imm_len());
        let data = cpu
            .mmu
            .load::<u32>(&cpu.state, rs1.wrapping_add(offset_sext))?;
        let value = sext(data as RegT, 32) & cpu.xlen.mask();
        cpu.state.xs.set_reg(self.rd() as u8, value);
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(I)]
  #[match_code(0x4003)]
  #[mask(0x707f)]
  ,Lbu);

impl Executable for Lbu {
    // x[rd] = M[x[rs1] + sext(offset)][7:0]
    // 无符号字节加载 (Load Byte, Unsigned). I-type, RV32I and RV64I.
    // 从地址 x[rs1] + sign-extend(offset)读取一个字节，经零扩展后写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);

        let offset_sext = sext(self.imm() as RegT, self.imm_len());
        let data = cpu
            .mmu
            .load::<u8>(&cpu.state, rs1.wrapping_add(offset_sext))?;
        cpu.state.xs.set_reg(self.rd() as u8, data as u64);
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(I)]
  #[match_code(0x5003)]
  #[mask(0x707f)]
  ,Lhu);

impl Executable for Lhu {
    // x[rd] = M[x[rs1] + sext(offset)][15:0]
    // 无符号半字加载 (Load Halfword, Unsigned). I-type, RV32I and RV64I.
    // 从地址 x[rs1] + sign-extend(offset)读取两个字节，经零扩展后写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);

        let offset_sext = sext(self.imm() as RegT, self.imm_len());
        let data = cpu
            .mmu
            .load::<u16>(&cpu.state, rs1.wrapping_add(offset_sext))?;
        cpu.state.xs.set_reg(self.rd() as u8, data as u64);
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(S)]
    #[match_code(0x23)]
    #[mask(0x707f)]
    ,Sb);

impl Executable for Sb {
    // M[x[rs1] + sext(offset) = x[rs2][7: 0]
    // 存字节(Store Byte). S-type, RV32I and RV64I.
    // 将 x[rs2]的低位字节存入内存地址 x[rs1]+sign-extend(offset)。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);

        let offset_sext = sext(self.imm() as RegT, self.imm_len());
        let data = cpu.state.xs.reg(self.rs2() as u8).get_bits(0..8) as u8;

        cpu.mmu
            .store::<u8>(&cpu.state, rs1.wrapping_add(offset_sext), data)?;
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(S)]
    #[match_code(0x1023)]
    #[mask(0x707f)]
    ,Sh);

impl Executable for Sh {
    // M[x[rs1] + sext(offset) = x[rs2][15: 0]
    // 存半字(Store Halfword). S-type, RV32I and RV64I.
    // 将 x[rs2]的低位 2 个字节存入内存地址 x[rs1]+sign-extend(offset)。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);

        let offset_sext = sext(self.imm() as RegT, self.imm_len());
        let data = cpu.state.xs.reg(self.rs2() as u8).get_bits(0..16) as u16;
        cpu.mmu
            .store::<u16>(&cpu.state, rs1.wrapping_add(offset_sext), data)?;
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(S)]
    #[match_code(0x2023)]
    #[mask(0x707f)]
    ,Sw);

impl Executable for Sw {
    // M[x[rs1] + sext(offset) = x[rs2][31: 0]
    // 存字(Store Word). S-type, RV32I and RV64I.
    // 将 x[rs2]的低位 4 个字节存入内存地址 x[rs1]+sign-extend(offset)。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);

        let offset_sext = sext(self.imm() as RegT, self.imm_len());
        let data = cpu.state.xs.reg(self.rs2() as u8).get_bits(0..32) as u32;
        cpu.mmu
            .store::<u32>(&cpu.state, rs1.wrapping_add(offset_sext), data)?;
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x13)]
    #[mask(0x707f)]
    ,Addi);

impl Executable for Addi {
    // x[rd] = x[rs1] + sext(immediate)
    // 加立即数(Add Immediate). I-type, RV32I and RV64I.
    // 把符号位扩展的立即数加到寄存器 x[rs1]上，结果写入 x[rd]。忽略算术溢出。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        cpu.state.xs.set_reg(
            self.rd() as u8,
            rs1.wrapping_add(sext(self.imm() as RegT, self.imm_len())),
        );
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x2013)]
    #[mask(0x707f)]
    ,Slti);

impl Executable for Slti {
    // x[rd] = (x[rs1] <𝑠 sext(immediate))
    // 小于立即数则置位(Set if Less Than Immediate). I-type, RV32I and RV64I.
    // 比较 x[rs1]和有符号扩展的 immediate，如果 x[rs1]更小，向 x[rd]写入 1，否则写入 0。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let imm_sext = sext(self.imm() as RegT, self.imm_len());

        let v = if (rs1 as SRegT) < (imm_sext as SRegT) {
            1
        } else {
            0
        };
        cpu.state.xs.set_reg(self.rd() as u8, v);
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x3013)]
    #[mask(0x707f)]
    ,Sltiu);

impl Executable for Sltiu {
    // x[rd] = (x[rs1] <𝑢 sext(immediate))
    // 无符号小于立即数则置位(Set if Less Than Immediate, Unsigned). I-type, RV32I and RV64I.
    // 比较 x[rs1]和有符号扩展的 immediate，比较时视为无符号数。如果 x[rs1]更小，向 x[rd]写入1，否则写入 0。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let imm_sext = sext(self.imm() as RegT, self.imm_len());
        let v = if rs1 < imm_sext { 1 } else { 0 };
        cpu.state.xs.set_reg(self.rd() as u8, v);
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x4013)]
    #[mask(0x707f)]
    ,Xori);

impl Executable for Xori {
    // x[rd] = x[rs1] ^ sext(immediate)
    // 立即数异或(Exclusive-OR Immediate). I-type, RV32I and RV64I.
    // x[rs1]和有符号扩展的 immediate 按位异或，结果写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let imm_sext = sext(self.imm() as RegT, self.imm_len());
        cpu.state
            .xs
            .set_reg(self.rd() as u8, (rs1 ^ imm_sext) & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x6013)]
    #[mask(0x707f)]
    ,Ori);

impl Executable for Ori {
    // x[rd] = x[rs1] | sext(immediate)
    // 立即数取或(OR Immediate). R-type, RV32I and RV64I.
    // 把寄存器 x[rs1]和有符号扩展的立即数 immediate 按位取或，结果写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let imm_sext = sext(self.imm() as RegT, self.imm_len());
        cpu.state
            .xs
            .set_reg(self.rd() as u8, (rs1 | imm_sext) & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x7013)]
    #[mask(0x707f)]
    ,Andi);

impl Executable for Andi {
    // x[rd] = x[rs1] & sext(immediate)
    // 与立即数 (And Immediate). I-type, RV32I and RV64I.
    // 把符号位扩展的立即数和寄存器 x[rs1]上的值进行位与，结果写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let imm_sext = sext(self.imm() as RegT, self.imm_len());
        cpu.state
            .xs
            .set_reg(self.rd() as u8, (rs1 & imm_sext) & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x1013)]
    #[mask(0xfc00707f)]
    ,Slli);

impl Executable for Slli {
    // x[rd] = x[rs1] ≪ shamt
    // 立即数逻辑左移(Shift Left Logical Immediate). I-type, RV32I and RV64I.
    // 把寄存器x[rs1]左移shamt位，空出的位置填入0，结果写入x[rd]。
    // 对于RV32I，仅当shamt[5]=0时，指令才是有效的。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let shamt = self.imm() & cpu.xlen.shamt_mask();
        cpu.state
            .xs
            .set_reg(self.rd() as u8, (rs1.wrapping_shl(shamt)) & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x5013)]
    #[mask(0xfc00707f)]
    ,Srli);

impl Executable for Srli {
    // x[rd] = (x[rs1] ≫𝑢 shamt)
    // 立即数逻辑右移(Shift Right Logical Immediate). I-type, RV32I and RV64I.
    // 把寄存器x[rs1]右移shamt位，空出的位置填入0，结果写入x[rd]。
    // 对于RV32I，仅当shamt[5]=0时，指令才是有效的。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let shamt = self.imm() & cpu.xlen.shamt_mask();
        cpu.state
            .xs
            .set_reg(self.rd() as u8, rs1.wrapping_shr(shamt) & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x40005013)]
    #[mask(0xfc00707f)]
    ,Srai);

impl Executable for Srai {
    // x[rd] = (x[rs1] ≫𝑠 shamt)
    // 立即数算术右移(Shift Right Arithmetic Immediate). I-type, RV32I and RV64I.
    // 把寄存器 x[rs1]右移 shamt 位，空位用 x[rs1]的最高位填充，结果写入 x[rd]。
    // 对于RV32I，仅当shamt[5]=0时，指令才是有效的。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8) as SRegT;
        let shamt = self.imm() & cpu.xlen.shamt_mask();
        cpu.state.xs.set_reg(
            self.rd() as u8,
            (rs1.wrapping_shr(shamt) as RegT) & cpu.xlen.mask(),
        );
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(R)]
  #[match_code(0x33)]
  #[mask(0xfe00707f)]
  ,Add);

impl Executable for Add {
    // x[rd] = x[rs1] + x[rs2]
    // 加 (Add). R-type, RV32I and RV64I.
    // 把寄存器 x[rs2]加到寄存器 x[rs1]上，结果写入 x[rd]。忽略算术溢出。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let rs2 = cpu.state.xs.reg(self.rs2() as u8);
        cpu.state
            .xs
            .set_reg(self.rd() as u8, rs1.wrapping_add(rs2) & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(R)]
    #[match_code(0x40000033)]
    #[mask(0xfe00707f)]
    ,Sub);

impl Executable for Sub {
    // x[rd] = x[rs1] − x[rs2]
    // 减(Substract). R-type, RV32I and RV64I.
    // x[rs1]减去 x[rs2]，结果写入 x[rd]。忽略算术溢出。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let rs2 = cpu.state.xs.reg(self.rs2() as u8);
        cpu.state
            .xs
            .set_reg(self.rd() as u8, rs1.wrapping_sub(rs2) & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(R)]
    #[match_code(0x1033)]
    #[mask(0xfe00707f)]
    ,Sll);

impl Executable for Sll {
    // x[rd] = x[rs1] ≪ x[rs2]
    // 逻辑左移(Shift Left Logical). R-type, RV32I and RV64I.
    // 把寄存器 x[rs1]左移 x[rs2]位，空出的位置填入 0，结果写入 x[rd]。
    // x[rs2]的低 5 位（如果是RV64I 则是低 6 位）代表移动位数，其高位则被忽略。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let rs2 = (cpu.state.xs.reg(self.rs2() as u8) as u32) & cpu.xlen.shamt_mask();
        cpu.state
            .xs
            .set_reg(self.rd() as u8, rs1.wrapping_shl(rs2) & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(R)]
    #[match_code(0x2033)]
    #[mask(0xfe00707f)]
    ,Slt);

impl Executable for Slt {
    // x[rd] = (x[rs1] <𝑠 x[rs2])
    // 小于则置位(Set if Less Than). R-type, RV32I and RV64I.
    // 比较 x[rs1]和 x[rs2]中的数，如果 x[rs1]更小，向 x[rd]写入 1，否则写入 0。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8) as SRegT;
        let rs2 = cpu.state.xs.reg(self.rs2() as u8) as SRegT;
        let v = if rs1 < rs2 { 1 } else { 0 };
        cpu.state.xs.set_reg(self.rd() as u8, v);
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(R)]
    #[match_code(0x3033)]
    #[mask(0xfe00707f)]
    ,Sltu);

impl Executable for Sltu {
    // x[rd] = (x[rs1] <𝑢 x[rs2])
    // 无符号小于则置位(Set if Less Than, Unsigned). R-type, RV32I and RV64I.
    // 比较 x[rs1]和 x[rs2]，比较时视为无符号数。如果 x[rs1]更小，向 x[rd]写入 1，否则写入 0。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let rs2 = cpu.state.xs.reg(self.rs2() as u8);
        let v = if rs1 < rs2 { 1 } else { 0 };
        cpu.state.xs.set_reg(self.rd() as u8, v);
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(R)]
    #[match_code(0x4033)]
    #[mask(0xfe00707f)]
    ,Xor);

impl Executable for Xor {
    // x[rd] = x[rs1] ^ x[rs2]
    // 异或(Exclusive-OR). R-type, RV32I and RV64I.
    // x[rs1]和 x[rs2]按位异或，结果写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let rs2 = cpu.state.xs.reg(self.rs2() as u8);
        cpu.state
            .xs
            .set_reg(self.rd() as u8, (rs1 ^ rs2) & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(R)]
    #[match_code(0x5033)]
    #[mask(0xfe00707f)]
    ,Srl);

impl Executable for Srl {
    // x[rd] = (x[rs1] ≫𝑢 x[rs2])
    // 逻辑右移(Shift Right Logical). R-type, RV32I and RV64I.
    // 把寄存器 x[rs1]右移 x[rs2]位，空出的位置填入 0，结果写入 x[rd]。
    // x[rs2]的低 5 位（如果是RV64I 则是低 6 位）代表移动位数，其高位则被忽略。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let rs2 = (cpu.state.xs.reg(self.rs2() as u8) as u32) & cpu.xlen.shamt_mask();
        cpu.state
            .xs
            .set_reg(self.rd() as u8, rs1.wrapping_shr(rs2) & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(R)]
    #[match_code(0x40005033)]
    #[mask(0xfe00707f)]
    ,Sra);

impl Executable for Sra {
    // x[rd] = (x[rs1] ≫𝑠 x[rs2])
    // 算术右移(Shift Right Arithmetic). R-type, RV32I and RV64I.
    // 把寄存器 x[rs1]右移 x[rs2]位，空位用 x[rs1]的最高位填充，结果写入 x[rd]。
    // x[rs2]的低 5 位（如果是 RV64I 则是低 6 位）为移动位数，高位则被忽略。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8) as SRegT;
        let rs2 = (cpu.state.xs.reg(self.rs2() as u8) as u32) & cpu.xlen.shamt_mask();
        cpu.state.xs.set_reg(
            self.rd() as u8,
            (rs1.wrapping_shr(rs2) as RegT) & cpu.xlen.mask(),
        );
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(R)]
    #[match_code(0x6033)]
    #[mask(0xfe00707f)]
    ,Or);

impl Executable for Or {
    // x[rd] = x[rs1] | 𝑥[𝑟𝑠2]
    // 取或(OR). R-type, RV32I and RV64I.
    // 把寄存器 x[rs1]和寄存器 x[rs2]按位取或，结果写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let rs2 = cpu.state.xs.reg(self.rs2() as u8);
        cpu.state
            .xs
            .set_reg(self.rd() as u8, (rs1 | rs2) & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(R)]
    #[match_code(0x7033)]
    #[mask(0xfe00707f)]
    ,And);

impl Executable for And {
    // x[rd] = x[rs1] & x[rs2]
    // 与 (And). R-type, RV32I and RV64I.
    // 将寄存器 x[rs1]和寄存器 x[rs2]位与的结果写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let rs2 = cpu.state.xs.reg(self.rs2() as u8);
        cpu.state
            .xs
            .set_reg(self.rd() as u8, (rs1 & rs2) & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0xf)]
    #[mask(0x707f)]
    ,Fence);

impl Executable for Fence {
    // Fence(pred, succ)
    // 同步内存和 I/O(Fence Memory and I/O). I-type, RV32I and RV64I.
    // 在后续指令中的内存和 I/O 访问对外部（例如其他线程）可见之前，使这条指令之前的内存
    // 及 I/O 访问对外部可见。比特中的第 3,2,1 和 0 位分别对应于设备输入，设备输出，内存读
    // 写。例如 fence r,rw，将前面读取与后面的读取和写入排序，使用 pred = 0010 和 succ = 0011
    // 进行编码。如果省略了参数，则表示 fence iorw, iorw，即对所有访存请求进行排序。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x100f)]
    #[mask(0x707f)]
    ,FenceI);

impl Executable for FenceI {
    // Fence(Store, Fetch)
    // 同步指令流(Fence Instruction Stream). I-type, RV32I and RV64I.
    // 使对内存指令区域的读写，对后续取指令可见。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x73)]
    #[mask(0xffffffff)]
    ,Ecall);

impl Executable for Ecall {
    // RaiseException(EnvironmentCall)
    // 环境调用 (Environment Call). I-type, RV32I and RV64I.
    // 通过引发环境调用异常来请求执行环境。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        match cpu.state.privilege {
            crate::PrivilegeMode::User => Err(Exception::UserEnvCall),
            crate::PrivilegeMode::Supervisor => Err(Exception::SupervisorEnvCall),
            crate::PrivilegeMode::Machine => Err(Exception::MachineEnvCall),
        }
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x100073)]
    #[mask(0xffffffff)]
    ,Ebreak);

impl Executable for Ebreak {
    // RaiseException(Breakpoint)
    // 环境断点 (Environment Breakpoint). I-type, RV32I and RV64I.
    // 通过抛出断点异常的方式请求调试器。
    fn exec(&self, _: &mut Cpu) -> Result<(), Exception> {
        Err(Exception::Breakpoint)
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x1073)]
    #[mask(0x707f)]
    ,Csrrw);

impl Executable for Csrrw {
    // t = CSRs[csr]; CSRs[csr] = x[rs1]; x[rd] = t
    // 读后写控制状态寄存器 (Control and Status Register Read and Write). I-type, RV32I and RV64I.
    // 记控制状态寄存器 csr 中的值为 t。把寄存器 x[rs1]的值写入 csr，再把 t 写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let scr_num = self.imm() as u16;
        let t = cpu.state.csrs.csr(scr_num);
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        cpu.state.csrs.set_csr(scr_num, rs1 & cpu.xlen.mask());

        cpu.state.xs.set_reg(self.rd() as u8, t & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x2073)]
    #[mask(0x707f)]
    ,Csrrs);

impl Executable for Csrrs {
    // t = CSRs[csr]; CSRs[csr] = t | x[rs1]; x[rd] = t
    // 读后置位控制状态寄存器 (Control and Status Register Read and Set). I-type, RV32I and RV64I.
    // 记控制状态寄存器 csr 中的值为 t。把 t 和寄存器 x[rs1]按位或的结果写入 csr，再把 t 写入x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let scr_num = self.imm() as u16;
        let t = cpu.state.csrs.csr(scr_num);
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        cpu.state.csrs.set_csr(scr_num, (t | rs1) & cpu.xlen.mask());
        cpu.state.xs.set_reg(self.rd() as u8, t & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x3073)]
    #[mask(0x707f)]
    ,Csrrc);

impl Executable for Csrrc {
    // t = CSRs[csr]; CSRs[csr] = t &~x[rs1]; x[rd] = t
    // 读后清除控制状态寄存器 (Control and Status Register Read and Clear). I-type, RV32I and RV64I.
    // 记控制状态寄存器 csr 中的值为 t。把 t 和寄存器 x[rs1]按位与的结果写入 csr，再把 t 写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let scr_num = self.imm() as u16;
        let t = cpu.state.csrs.csr(scr_num);
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        cpu.state
            .csrs
            .set_csr(scr_num, (t & !rs1) & cpu.xlen.mask());
        cpu.state.xs.set_reg(self.rd() as u8, t & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x5073)]
    #[mask(0x707f)]
    ,Csrrwi);

impl Executable for Csrrwi {
    // x[rd] = CSRs[csr]; CSRs[csr] = zimm
    // 立即数读后写控制状态寄存器 (Control and Status Register Read and Write Immediate). I-type, RV32I and RV64I.
    // 把控制状态寄存器 csr 中的值拷贝到 x[rd]中，再把五位的零扩展的立即数 zimm 的值写入csr。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let scr_num = self.imm() as u16;
        let zimm = self.rs1() as RegT;
        let t = cpu.state.csrs.csr(scr_num);
        cpu.state.xs.set_reg(self.rd() as u8, t & cpu.xlen.mask());
        cpu.state.csrs.set_csr(scr_num, zimm);
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x6073)]
    #[mask(0x707f)]
    ,Csrrsi);

impl Executable for Csrrsi {
    // t = CSRs[csr]; CSRs[csr] = t | zimm; x[rd] = t
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let scr_num = self.imm() as u16;
        let zimm = self.rs1() as RegT;
        let t = cpu.state.csrs.csr(scr_num);
        cpu.state
            .csrs
            .set_csr(scr_num, (t | zimm) & cpu.xlen.mask());
        cpu.state.xs.set_reg(self.rd() as u8, t & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x7073)]
    #[mask(0x707f)]
    ,Csrrci);

impl Executable for Csrrci {
    // t = CSRs[csr]; CSRs[csr] = t &~zimm; x[rd] = t
    // 立即数读后清除控制状态寄存器 (Control and Status Register Read and Clear Immediate). Itype, RV32I and RV64I.
    // 记控制状态寄存器 csr 中的值为 t。把 t 和五位的零扩展的立即数 zimm 按位与的结果写入csr，再把 t 写入 x[rd]（csr 寄存器的第 5 位及更高位不变）。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let scr_num = self.imm() as u16;
        let zimm = self.rs1() as RegT;
        let t = cpu.state.csrs.csr(scr_num);
        cpu.state
            .csrs
            .set_csr(scr_num, (t & !zimm) & cpu.xlen.mask());
        cpu.state.xs.set_reg(self.rd() as u8, t & cpu.xlen.mask());
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x6003)]
    #[mask(0x707f)]
    ,Lwu);

impl Executable for Lwu {
    // x[rd] = M[x[rs1] + sext(offset)][31:0]
    // 无符号字加载 (Load Word, Unsigned). I-type, RV64I.
    // 从地址 x[rs1] + sign-extend(offset)读取四个字节，零扩展后写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        if let XLen::X32 = cpu.xlen {
            return Err(Exception::InstructionFault);
        }
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);

        let offset_sext = sext(self.imm() as RegT, self.imm_len());
        let data = cpu
            .mmu
            .load::<u32>(&cpu.state, rs1.wrapping_add(offset_sext))?;
        cpu.state.xs.set_reg(self.rd() as u8, data as RegT);
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x3003)]
    #[mask(0x707f)]
    ,Ld);

impl Executable for Ld {
    // x[rd] = M[x[rs1] + sext(offset)][63:0]
    // 双字加载 (Load Doubleword). I-type, RV32I and RV64I.
    // 从地址 x[rs1] + sign-extend(offset)读取八个字节，写入 x[rd]。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        if let XLen::X32 = cpu.xlen {
            return Err(Exception::InstructionFault);
        }
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let offset_sext = sext(self.imm() as RegT, self.imm_len());
        let data = cpu
            .mmu
            .load::<u64>(&cpu.state, rs1.wrapping_add(offset_sext))?;
        cpu.state.xs.set_reg(self.rd() as u8, data as RegT);
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(S)]
    #[match_code(0x3023)]
    #[mask(0x707f)]
    ,Sd);

impl Executable for Sd {
    // M[x[rs1] + sext(offset)] = x[rs2][63: 0]
    // 存双字(Store Doubleword). S-type, RV64I only.
    // 将 x[rs2]中的 8 字节存入内存地址 x[rs1]+sign-extend(offset)。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        if let XLen::X32 = cpu.xlen {
            return Err(Exception::InstructionFault);
        }
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let offset_sext = sext(self.imm() as RegT, self.imm_len());
        let data = cpu.state.xs.reg(self.rs2() as u8);
        cpu.mmu
            .store::<u64>(&cpu.state, rs1.wrapping_add(offset_sext), data)?;
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x1b)]
    #[mask(0x707f)]
    ,Addiw);

impl Executable for Addiw {
    // x[rd] = sext((x[rs1] + sext(immediate))[31:0])
    // 加立即数字(Add Word Immediate). I-type, RV64I.
    // 把符号位扩展的立即数加到 x[rs1]，将结果截断为 32 位，把符号位扩展的结果写入 x[rd]。忽略算术溢出。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        if let XLen::X32 = cpu.xlen {
            return Err(Exception::InstructionFault);
        }
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let imm_sext = sext(self.imm() as RegT, self.imm_len());

        cpu.state.xs.set_reg(
            self.rd() as u8,
            sext(rs1.wrapping_add(imm_sext), 32) & cpu.xlen.mask(),
        );
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x101b)]
    #[mask(0xfe00707f)]
    ,Slliw);

impl Executable for Slliw {
    // x[rd] = sext((x[rs1] ≪ shamt)[31: 0])
    // 立即数逻辑左移字(Shift Left Logical Word Immediate). I-type, RV64I only.
    // 把寄存器 x[rs1]左移 shamt 位，空出的位置填入 0，结果截为 32 位，进行有符号扩展后写入x[rd]。
    // 仅当 shamt[5]=0 时，指令才是有效的。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        if let XLen::X32 = cpu.xlen {
            return Err(Exception::InstructionFault);
        }
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let shamt = self.imm() & cpu.xlen.shamt_mask();

        let value = sext(rs1.wrapping_shl(shamt as u32), 32);
        cpu.state.xs.set_reg(self.rd() as u8, value);
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x4000501b)]
    #[mask(0xfe00707f)]
    ,Sraiw);

impl Executable for Sraiw {
    // x[rd] = sext(x[rs1][31: 0] ≫𝑠 shamt)
    // 立即数算术右移字(Shift Right Arithmetic Word Immediate). I-type, RV64I only.
    // 把寄存器 x[rs1]的低 32 位右移 shamt 位，空位用 x[rs1][31]填充，结果进行有符号扩展后写入 x[rd]。
    // 仅当 shamt[5]=0 时指令有效。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        if let XLen::X32 = cpu.xlen {
            return Err(Exception::InstructionFault);
        }
        let rs1 = cpu.state.xs.reg(self.rs1() as u8) as u32 as RegT;
        let shamt = self.imm() & cpu.xlen.shamt_mask();
        let value = sext(rs1.wrapping_shr(shamt as u32), 32 - shamt as usize);
        cpu.state.xs.set_reg(self.rd() as u8, value);
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(I)]
    #[match_code(0x501b)]
    #[mask(0xfe00707f)]
    ,Srliw);

impl Executable for Srliw {
    // x[rd] = sext(x[rs1][31: 0] ≫𝑢 shamt)
    // 立即数逻辑右移字(Shift Right Logical Word Immediate). I-type, RV64I only.
    // 把寄存器 x[rs1]右移 shamt 位，空出的位置填入 0，结果截为 32 位，进行有符号扩展后写入
    // x[rd]。仅当 shamt[5]=0 时，指令才是有效的。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        if let XLen::X32 = cpu.xlen {
            return Err(Exception::InstructionFault);
        }
        let rs1 = cpu.state.xs.reg(self.rs1() as u8) as u32 as RegT;
        let shamt = self.imm() & cpu.xlen.shamt_mask();
        let value = sext(rs1.wrapping_shr(shamt as u32), 32);
        cpu.state.xs.set_reg(self.rd() as u8, value);
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
  #[derive(Instruction)]
  #[format(R)]
  #[match_code(0x3b)]
  #[mask(0xfe00707f)]
  ,Addw);

impl Executable for Addw {
    // x[rd] = sext((x[rs1] + x[rs2])[31:0])
    // 加字(Add Word). R-type, RV64I.
    // 把寄存器 x[rs2]加到寄存器 x[rs1]上，将结果截断为 32 位，把符号位扩展的结果写入 x[rd]。忽略算术溢出。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        if let XLen::X32 = cpu.xlen {
            return Err(Exception::InstructionFault);
        }
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let rs2 = cpu.state.xs.reg(self.rs2() as u8);
        cpu.state.xs.set_reg(
            self.rd() as u8,
            sext(rs1.wrapping_add(rs2), 32) & cpu.xlen.mask(),
        );
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(R)]
    #[match_code(0x4000003b)]
    #[mask(0xfe00707f)]
    ,Subw);

impl Executable for Subw {
    // x[rd] = sext((x[rs1] − x[rs2])[31: 0])
    // 减去字(Substract Word). R-type, RV64I only.
    // x[rs1]减去 x[rs2]，结果截为 32 位，有符号扩展后写入 x[rd]。忽略算术溢出。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        if let XLen::X32 = cpu.xlen {
            return Err(Exception::InstructionFault);
        }
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let rs2 = cpu.state.xs.reg(self.rs2() as u8);
        cpu.state.xs.set_reg(
            self.rd() as u8,
            sext(rs1.wrapping_sub(rs2), 32) & cpu.xlen.mask(),
        );
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(R)]
    #[match_code(0x103b)]
    #[mask(0xfe00707f)]
    ,Sllw);

impl Executable for Sllw {
    // x[rd] = sext((x[rs1] ≪ x[rs2][4: 0])[31: 0])
    // 逻辑左移字(Shift Left Logical Word). R-type, RV64I only.
    // 把寄存器 x[rs1]的低 32 位左移 x[rs2]位，空出的位置填入 0，结果进行有符号扩展后写入
    // x[rd]。x[rs2]的低 5 位代表移动位数，其高位则被忽略。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        if let XLen::X32 = cpu.xlen {
            return Err(Exception::InstructionFault);
        }
        let rs1 = cpu.state.xs.reg(self.rs1() as u8);
        let rs2 = cpu.state.xs.reg(self.rs2() as u8);

        cpu.state.xs.set_reg(
            self.rd() as u8,
            sext(rs1.wrapping_shl((rs2 & 0x1f) as u32), 32),
        );
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(R)]
    #[match_code(0x4000503b)]
    #[mask(0xfe00707f)]
    ,Sraw);

impl Executable for Sraw {
    // x[rd] = sext(x[rs1][31: 0] ≫𝑠 x[rs2][4: 0])
    // 算术右移字(Shift Right Arithmetic Word). R-type, RV64I only.
    // 把寄存器 x[rs1]的低 32 位右移 x[rs2]位，空位用 x[rs1][31]填充，结果进行有符号扩展后写入 x[rd]。
    // x[rs2]的低 5 位为移动位数，高位则被忽略。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        if let XLen::X32 = cpu.xlen {
            return Err(Exception::InstructionFault);
        }
        let rs1 = cpu.state.xs.reg(self.rs1() as u8) as u32 as RegT;
        let rs2 = (cpu.state.xs.reg(self.rs2() as u8) & 0x1f) as u32;

        cpu.state.xs.set_reg(
            self.rd() as u8,
            sext(rs1.wrapping_shr(rs2), 32 - rs2 as usize),
        );
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(R)]
    #[match_code(0x10200073)]
    #[mask(0xffffffff)]
    ,Sret);

impl Executable for Sret {
    // ExceptionReturn(Supervisor)
    // 管理员模式例外返回(Supervisor-mode Exception Return). R-type, RV32I and RV64I 特权指令。
    // 从管理员模式的例外处理程序中返回，设置 pc 为 CSRs[spec]，权限模式为 CSRs[sstatus].SPP，
    // CSRs[sstatus].SIE 为 CSRs[sstatus].SPIE，CSRs[sstatus].SPIE 为 1，CSRs[sstatus].spp 为 0
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        // println!("这里是 sret sepc: {:x} pc: {:X}", cpu.state.csrs.sepc(), cpu.state.pc);

        cpu.state.update_pc(cpu.state.csrs.sepc());
        let mut sstatus = cpu.state.csrs.sstatus();
        cpu.state.privilege = sstatus.spp();
        sstatus.set_sie(sstatus.spie());
        sstatus.set_spie(true);
        sstatus.set_spp(PrivilegeMode::User);
        cpu.state.csrs.set_sstatus(sstatus.bits());
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(R)]
    #[match_code(0x30200073)]
    #[mask(0xffffffff)]
    ,Mret);

impl Executable for Mret {
    // ExceptionReturn(Machine)
    // 机器模式异常返回(Machine-mode Exception Return). R-type, RV32I and RV64I 特权架构
    // 从机器模式异常处理程序返回。将 pc 设置为 CSRs[mepc], 将特权级设置成
    // CSRs[mstatus].MPP, CSRs[mstatus].MIE 置成 CSRs[mstatus].MPIE, 并且将
    // CSRs[mstatus].MPIE 为 1;并且，如果支持用户模式，则将 CSR [mstatus].MPP 设置为 0。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        cpu.state.update_pc(cpu.state.csrs.mepc());
        let mut mstatus = cpu.state.csrs.mstatus();
        cpu.state.privilege = mstatus.mpp();
        mstatus.set_mie(mstatus.mpie());
        mstatus.set_mpie(true);
        mstatus.set_mpp(PrivilegeMode::User);
        cpu.state.csrs.set_mstatus(mstatus.bits());
        Ok(())
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(R)]
    #[match_code(0x10500073)]
    #[mask(0xffffffff)]
    ,Wfi);

impl Executable for Wfi {
    // while (noInterruptPending) idle
    // 等待中断(Wait for Interrupt). R-type, RV32I and RV64I 特权指令。
    // 如果没有待处理的中断，则使处理器处于空闲状态。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        cpu.state.update_pc(cpu.state.pc + 4);
        todo!();
    }
}

def_insn!(
    #[derive(Instruction)]
    #[format(R)]
    #[match_code(0x12000073)]
    #[mask(0xfe007fff)]
    ,SfenceVma);

impl Executable for SfenceVma {
    // Fence(Store, AddressTranslation)
    // 虚拟内存屏障(Fence Virtual Memory). R-type, RV32I and RV64I 特权指令。
    // 根据后续的虚拟地址翻译对之前的页表存入进行排序。当 rs2=0 时，所有地址空间的翻译都
    // 会受到影响；否则，仅对 x[rs2]标识的地址空间的翻译进行排序。当 rs1=0 时，对所选地址
    // 空间中的所有虚拟地址的翻译进行排序；否则，仅对其中包含虚拟地址 x[rs1]的页面地址翻译进行排序。
    fn exec(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        cpu.state.update_pc(cpu.state.pc + 4);
        Ok(())
    }
}
