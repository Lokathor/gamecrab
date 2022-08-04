use bitfrob::{u8_get_bit, u8_get_value};

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
impl TryFrom<u8> for R8 {
  type Error = ();
  fn try_from(u: u8) -> Result<Self, Self::Error> {
    Ok(match u {
      0 => R8::B,
      1 => R8::C,
      2 => R8::D,
      3 => R8::E,
      4 => R8::H,
      5 => R8::L,
      6 => R8::HLm,
      7 => R8::A,
      _ => return Err(()),
    })
  }
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
impl TryFrom<u8> for R16p {
  type Error = ();
  fn try_from(u: u8) -> Result<Self, Self::Error> {
    Ok(match u {
      0 => R16p::BC,
      1 => R16p::DE,
      2 => R16p::HL,
      3 => R16p::SP,
      _ => return Err(()),
    })
  }
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
impl TryFrom<u8> for R16f {
  type Error = ();
  fn try_from(u: u8) -> Result<Self, Self::Error> {
    Ok(match u {
      0 => R16f::BC,
      1 => R16f::DE,
      2 => R16f::HL,
      3 => R16f::AF,
      _ => return Err(()),
    })
  }
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
impl TryFrom<u8> for Cond {
  type Error = ();
  fn try_from(u: u8) -> Result<Self, Self::Error> {
    Ok(match u {
      0 => Cond::NZ,
      1 => Cond::Z,
      2 => Cond::NC,
      3 => Cond::C,
      _ => return Err(()),
    })
  }
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
impl TryFrom<u8> for Alu {
  type Error = ();
  fn try_from(u: u8) -> Result<Self, Self::Error> {
    Ok(match u {
      0 => Alu::Add,
      1 => Alu::Adc,
      2 => Alu::Sub,
      3 => Alu::Sbc,
      4 => Alu::And,
      5 => Alu::Xor,
      6 => Alu::Or,
      7 => Alu::Cp,
      _ => return Err(()),
    })
  }
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
impl TryFrom<u8> for Rot {
  type Error = ();
  fn try_from(u: u8) -> Result<Self, Self::Error> {
    Ok(match u {
      0 => Rot::Rlc,
      1 => Rot::Rrc,
      2 => Rot::Rl,
      3 => Rot::Rr,
      4 => Rot::Sla,
      5 => Rot::Sra,
      6 => Rot::Swap,
      7 => Rot::Srl,
      _ => return Err(()),
    })
  }
}

const fn x(op_code: u8) -> u8 {
  u8_get_value::<6, 7>(op_code)
}
const fn y(op_code: u8) -> u8 {
  u8_get_value::<3, 5>(op_code)
}
const fn z(op_code: u8) -> u8 {
  u8_get_value::<0, 2>(op_code)
}
const fn p(op_code: u8) -> u8 {
  u8_get_value::<4, 5>(op_code)
}
const fn q(op_code: u8) -> bool {
  u8_get_bit::<3>(op_code)
}

/// Gives the number of bytes that must come after this op code to complete the instruction.
pub const fn op_code_tail_bytes(op_code: u8) -> usize {
  match x(op_code) {
    0 => match z(op_code) {
      0 => match y(op_code) {
        0 => 0,
        1 => 2,
        2 => 1, /* Stop has a trailing unused byte */
        3 => 1,
        y => 1,
      },
      1 => {
        if q(op_code) {
          0
        } else {
          2
        }
      }
      2 => 0,
      3 => 0,
      4 => 0,
      5 => 0,
      6 => 1,
      7 => 0,
      _ => unreachable!(),
    },
    1 => 0,
    2 => 0,
    3 => match z(op_code) {
      0 => match y(op_code) {
        0..=3 => 0,
        4 => 1,
        5 => 1,
        6 => 1,
        7 => 1,
        _ => unreachable!(),
      },
      1 => 0,
      2 => match y(op_code) {
        0..=3 => 2,
        4 => 0,
        5 => 2,
        6 => 0,
        7 => 2,
        _ => unreachable!(),
      },
      3 => match y(op_code) {
        0 => 2,
        1 => 1,
        2 => 0,
        3 => 0,
        4 => 0,
        5 => 0,
        6 => 1,
        7 => 1,
        _ => unreachable!(),
      },
      4 => match y(op_code) {
        0..=3 => 2,
        _ => 0,
      },
      5 => {
        if q(op_code) {
          if p(op_code) == 0 {
            2
          } else {
            0
          }
        } else {
          0
        }
      }
      6 => 1,
      7 => 0,
      _ => unreachable!(),
    },
    _ => unreachable!(),
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Op {
  /// `nop`
  ///
  /// No operation
  #[default]
  Nop,
  /// `ld (<u16>), sp`
  ///
  /// Writes `sp` to the immediate address.
  LdImmAddrSp(u16),
  /// `stop`
  Stop,
  /// `jr i8`
  ///
  /// Jumps to the signed value +2
  JumpRel(i8),
  /// `jr <cond> i8`
  ///
  /// If condition, jumps to the signed value +2
  JumpRelCond(Cond, i8),
  /// `add hl, <r16p>`
  AddHLR16p(R16p),
  /// `ld <r16p>, <u16>`
  LdR16pImm16(R16p, u16),
  /// `ld (bc), a`
  LdBCmA,
  /// `ld (de), a`
  LdDEmA,
  /// `ld (hl+), a`
  LdHLmiA,
  /// `ld (hl-), a`
  LdHLmdA,
  /// `ld a, (bc)`
  LdABCm,
  /// `ld a, (de)`
  LdADEm,
  /// `ld a, (hl+)`
  LdAHLmi,
  /// `ld a, (hl-)`
  LdAHLmd,
  /// `inc <r16p>`
  Inc16(R16p),
  /// `dec <r16p>`
  Dec16(R16p),
  /// `inc <r8>`
  Inc8(R8),
  /// `dec <r8>`
  Dec8(R8),
  /// `ld <r8>, <imm8>`
  LdR8Imm(R8, u8),
  /// `rlca`
  Rlca,
  /// `rrca`
  Rrca,
  /// `rla`
  Rla,
  /// `rra`
  Rra,
  /// `daa`
  Daa,
  /// `cpl`
  Cpl,
  /// `scf`
  Scf,
  /// `ccf`
  Ccf,
  /// `halt`
  Halt,
  /// `ld <r8>, <r8>`
  LdR8R8(R8, R8),
  /// `<alu> A, <r8>`
  AluAR8(Alu, R8),
  /// `ret <cond>`
  RetCond(Cond),
  /// `ld ($FF00+<u8>), a`
  LdHpImmA(u8),
  /// `add sp, <i8>`
  AddSPd(i8),
  /// `ld a, ($FF00+<u8>)`
  LdAHpImm(u8),
  /// `ld hl, sp+<i8>`
  LdHLSPd(i8),
  /// `pop <r16f>`
  Pop(R16f),
  /// `ret`
  Ret,
  /// `reti`
  Reti,
  /// `jp hl`
  JpHL,
  /// `ld sp, hl`
  LdSPHL,
  /// `jp <cond>, <u16>`
  JpCond(Cond, u16),
  /// `ld ($FF00+C), a`
  LdHpCA,
  /// `ld (<u16>), a`
  LdImm16A(u16),
  /// `ld a, ($FF00+c)`
  LdAHpC,
  /// `ld a, (<u16>)`
  LdAImm16(u16),
  /// `jp <u16>`
  Jp(u16),
  /// `di`
  Di,
  /// `ei`
  Ei,
  /// `<rot> <r8>`
  Rot8(Rot, R8),
  /// `bit <b>, <r8>`
  Bit(u8, R8),
  /// `res <b>, <r8>`
  Res(u8, R8),
  /// `set <b>, <r8>`
  Set(u8, R8),
  /// `call <cond>, <u16>`
  CallCond(Cond, u16),
  /// `call <u16>`
  Call(u16),
  /// `push <r16f>`
  Push(R16f),
  /// `<alu> A, <u8>`
  AluAImm(Alu, u8),
  /// `rst <u8>`
  Rst(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DecodeError {
  Incomplete(u8),
  Incomplete2(u8, u8),
  Illegal(u8),
}

pub struct OpDecoder<I> {
  i: I,
  bytes_previous: u32,
}
impl<I> From<I> for OpDecoder<I>
where
  I: Iterator<Item = u8>,
{
  fn from(i: I) -> Self {
    Self { i, bytes_previous: 0 }
  }
}
impl<I> OpDecoder<I>
where
  I: Iterator<Item = u8>,
{
  /// Reads the next byte and builds them into a `u16`
  ///
  /// If there's not enough bytes to read, then you'll get a DecodeError.
  fn next_u8(&mut self, op_code: u8) -> Result<u8, DecodeError> {
    let first = match self.i.next() {
      Some(f) => f,
      None => return Err(DecodeError::Incomplete(op_code)),
    };
    Ok(first)
  }

  /// Reads the next two bytes and builds them into a `u16`
  ///
  /// If there's not enough bytes to read, then you'll get a DecodeError.
  fn next_u16(&mut self, op_code: u8) -> Result<u16, DecodeError> {
    let first = match self.i.next() {
      Some(f) => {
        self.bytes_previous += 1;
        f
      }
      None => return Err(DecodeError::Incomplete(op_code)),
    };
    let second = match self.i.next() {
      Some(f) => {
        self.bytes_previous += 1;
        f
      }
      None => return Err(DecodeError::Incomplete2(op_code, first)),
    };
    Ok(u16::from_le_bytes([first, second]))
  }
}

impl<I> Iterator for OpDecoder<I>
where
  I: Iterator<Item = u8>,
{
  type Item = (u32, Result<Op, DecodeError>);
  fn next(&mut self) -> Option<Self::Item> {
    // If there's no bytes to pull, we don't produce an opcode. However, once
    // any bytes have been pulled out, we're "locked in" to producing some sort
    // of op output, even if we have to return an "incomplete" op.
    let start_byte = self.bytes_previous;
    let op_code = self.i.next()?;
    self.bytes_previous += 1;
    Some((
      start_byte,
      Ok(match x(op_code) {
        0 => match z(op_code) {
          0 => match y(op_code) {
            0 => Op::Nop,
            1 => {
              return Some((
                start_byte,
                self.next_u16(op_code).map(|u| Op::LdImmAddrSp(u)),
              ))
            }
            2 => Op::Stop,
            3 => {
              return Some((
                start_byte,
                self.next_u8(op_code).map(|u| Op::JumpRel(u as i8)),
              ))
            }
            y => {
              return Some((
                start_byte,
                self.next_u8(op_code).map(|u| {
                  Op::JumpRelCond(Cond::try_from(y - 4).unwrap(), u as i8)
                }),
              ))
            }
          },
          1 => {
            if q(op_code) {
              Op::AddHLR16p(R16p::try_from(p(op_code)).unwrap())
            } else {
              return Some((
                start_byte,
                self.next_u16(op_code).map(|u| {
                  Op::LdR16pImm16(R16p::try_from(p(op_code)).unwrap(), u)
                }),
              ));
            }
          }
          2 => {
            if q(op_code) {
              match p(op_code) {
                0 => Op::LdABCm,
                1 => Op::LdADEm,
                2 => Op::LdAHLmi,
                3 => Op::LdAHLmd,
                _ => unreachable!(),
              }
            } else {
              match p(op_code) {
                0 => Op::LdBCmA,
                1 => Op::LdDEmA,
                2 => Op::LdHLmiA,
                3 => Op::LdHLmdA,
                _ => unreachable!(),
              }
            }
          }
          3 => {
            if q(op_code) {
              Op::Dec16(R16p::try_from(p(op_code)).unwrap())
            } else {
              Op::Inc16(R16p::try_from(p(op_code)).unwrap())
            }
          }
          4 => Op::Inc8(R8::try_from(y(op_code)).unwrap()),
          5 => Op::Dec8(R8::try_from(y(op_code)).unwrap()),
          6 => {
            return Some((
              start_byte,
              self
                .next_u8(op_code)
                .map(|u| Op::LdR8Imm(R8::try_from(y(op_code)).unwrap(), u)),
            ))
          }
          7 => match y(op_code) {
            0 => Op::Rlca,
            1 => Op::Rrca,
            2 => Op::Rla,
            3 => Op::Rra,
            4 => Op::Daa,
            5 => Op::Cpl,
            6 => Op::Scf,
            7 => Op::Ccf,
            _ => unreachable!(),
          },
          _ => unreachable!(),
        },
        1 => {
          if (y(op_code) == 6) & (z(op_code) == 6) {
            Op::Halt
          } else {
            Op::LdR8R8(
              R8::try_from(y(op_code)).unwrap(),
              R8::try_from(z(op_code)).unwrap(),
            )
          }
        }
        2 => Op::AluAR8(
          Alu::try_from(y(op_code)).unwrap(),
          R8::try_from(z(op_code)).unwrap(),
        ),
        3 => match z(op_code) {
          0 => match y(op_code) {
            0..=3 => Op::RetCond(Cond::try_from(y(op_code)).unwrap()),
            4 => {
              return Some((
                start_byte,
                self.next_u8(op_code).map(|u| Op::LdHpImmA(u)),
              ))
            }
            5 => {
              return Some((
                start_byte,
                self.next_u8(op_code).map(|u| Op::AddSPd(u as i8)),
              ))
            }
            6 => {
              return Some((
                start_byte,
                self.next_u8(op_code).map(|u| Op::LdAHpImm(u)),
              ))
            }
            7 => {
              return Some((
                start_byte,
                self.next_u8(op_code).map(|u| Op::LdHLSPd(u as i8)),
              ))
            }
            _ => unreachable!(),
          },
          1 => {
            if q(op_code) {
              match p(op_code) {
                0 => Op::Ret,
                1 => Op::Reti,
                2 => Op::JpHL,
                3 => Op::LdSPHL,
                _ => unreachable!(),
              }
            } else {
              Op::Pop(R16f::try_from(p(op_code)).unwrap())
            }
          }
          2 => match y(op_code) {
            0..=3 => {
              return Some((
                start_byte,
                self
                  .next_u16(op_code)
                  .map(|u| Op::JpCond(Cond::try_from(y(op_code)).unwrap(), u)),
              ))
            }
            4 => Op::LdHpCA,
            5 => {
              return Some((
                start_byte,
                self.next_u16(op_code).map(|u| Op::LdImm16A(u)),
              ))
            }
            6 => Op::LdAHpC,
            7 => {
              return Some((
                start_byte,
                self.next_u16(op_code).map(|u| Op::LdAImm16(u)),
              ))
            }
            _ => unreachable!(),
          },
          3 => match y(op_code) {
            0 => {
              return Some((
                start_byte,
                self.next_u16(op_code).map(|u| Op::Jp(u)),
              ))
            }
            1 => {
              return Some((
                start_byte,
                self.next_u8(op_code).map(|u| match x(u) {
                  0 => Op::Rot8(
                    Rot::try_from(y(op_code)).unwrap(),
                    R8::try_from(z(op_code)).unwrap(),
                  ),
                  1 => Op::Bit(y(op_code), R8::try_from(z(op_code)).unwrap()),
                  2 => Op::Res(y(op_code), R8::try_from(z(op_code)).unwrap()),
                  3 => Op::Set(y(op_code), R8::try_from(z(op_code)).unwrap()),
                  _ => unreachable!(),
                }),
              ))
            }
            2 => return Some((start_byte, Err(DecodeError::Illegal(op_code)))),
            3 => return Some((start_byte, Err(DecodeError::Illegal(op_code)))),
            4 => return Some((start_byte, Err(DecodeError::Illegal(op_code)))),
            5 => return Some((start_byte, Err(DecodeError::Illegal(op_code)))),
            6 => Op::Di,
            7 => Op::Ei,
            _ => unreachable!(),
          },
          4 => match y(op_code) {
            0..=3 => {
              return Some((
                start_byte,
                self.next_u16(op_code).map(|u| {
                  Op::CallCond(Cond::try_from(y(op_code)).unwrap(), u)
                }),
              ))
            }
            _ => return Some((start_byte, Err(DecodeError::Illegal(op_code)))),
          },
          5 => {
            if q(op_code) {
              if p(op_code) == 0 {
                return Some((
                  start_byte,
                  self.next_u16(op_code).map(|u| Op::Call(u)),
                ));
              } else {
                return Some((start_byte, Err(DecodeError::Illegal(op_code))));
              }
            } else {
              Op::Push(R16f::try_from(p(op_code)).unwrap())
            }
          }
          6 => {
            return Some((
              start_byte,
              self
                .next_u8(op_code)
                .map(|u| Op::AluAImm(Alu::try_from(y(op_code)).unwrap(), u)),
            ))
          }
          7 => Op::Rst(y(op_code) * 8),
          _ => unreachable!(),
        },
        _ => unreachable!(),
      }),
    ))
  }
}

#[test]
fn test_OpDecoder_next_never_panics() {
  let mut buffer = vec![0_u8; 1024];
  getrandom::getrandom(&mut buffer).unwrap();
  for _ in OpDecoder::from(buffer.iter().copied()) {
    //
  }
}

#[test]
fn test_OpDecoder_next() {
  let bytes = [0x00, 0xC3, 0x50, 0x01];
  let mut decoder = OpDecoder::from(bytes.iter().copied());
  assert_eq!(decoder.next(), Some((0, Ok(Op::Nop))));
  assert_eq!(decoder.next(), Some((1, Ok(Op::Jp(0x0150)))));
  assert_eq!(decoder.next(), None);
}
