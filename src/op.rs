#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct OpCode(u8);
impl OpCode {
  pub const fn x(self) -> u8 {
    u8_ge
  }
}

/// A register-based 8-bit value.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum R8 {
  #[default]
  B = 0,
  C = 1,
  D = 2,
  E = 3,
  H = 4,
  L = 5,
  /// In this case, we don't use HL itself, we use the byte in memory at the
  /// address that the HL value points to.
  HLm = 6,
  A = 7,
}

/// A register-based 16-bit value where SP is the 4th possibility.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum R16p {
  #[default]
  BC = 0,
  DE = 1,
  HL = 2,
  SP = 3,
}

/// A register-based 16-bit value where AF is the 4th possibility.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum R16f {
  #[default]
  BC = 0,
  DE = 1,
  HL = 2,
  AF = 3,
}

/// A condition code, for conditional operations.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Cond {
  #[default]
  NZ = 0,
  Z = 1,
  NC = 2,
  C = 3,
}

/// Arithmetic Logic Unit operations.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Alu {
  #[default]
  Add = 0,
  Adc = 1,
  Sub = 2,
  Sbc = 3,
  And = 4,
  Xor = 5,
  Or = 6,
  Cp = 7,
}

/// Rotation operations.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Rot {
  #[default]
  Rlc = 0,
  Rrc = 1,
  Rl = 2,
  Rr = 3,
  Sla = 4,
  Sra = 5,
  Swap = 6,
  Srl = 7,
}

pub enum Op {
  Nop,
}
