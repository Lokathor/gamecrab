use std::ops::{Deref, DerefMut};

use bitfrob::{u8_get_bit, u8_with_bit};

/// Simulates the Game Boy's LR35902 CPU.
///
/// This is the view of the CPU with 16-bit registers. To access the 8-bit
/// registers, we have a [Deref] impl from this type to the [CpuByteFields]
/// type. This lets us easily access any data register as either the 16-bit or
/// 8-bit form, but brings a drawback: the `Deref` borrows the entire struct, so
/// we can't easily "split borrow" when we're accessing the fields in 8-bit
/// mode. I don't think that will be an issue, because we won't normally have to
/// hold a borrow on the CPU longer than a single statement.
///
/// ```
/// # use gamecrab::*;
/// let cpu = Cpu::default();
/// assert_eq!(cpu.bc, Reg16::default());
/// assert_eq!(cpu.d, Reg8::default());
/// ```
///
/// * See Also: [Pandocs: CPU Registers and flags](https://gbdev.io/pandocs/CPU_Registers_and_Flags.html)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Cpu {
  pub af: Reg16,
  pub bc: Reg16,
  pub de: Reg16,
  pub hl: Reg16,
  pub sp: Reg16,
  pub pc: Reg16,
}
impl Deref for Cpu {
  type Target = CpuByteFields;
  #[inline]
  #[must_use]
  fn deref(&self) -> &Self::Target {
    // Safety:
    // * Our size is the same as the new target type
    // * Our align is greater than or equal to the new target type
    // * All bytes in self are valid bytes in the new target type
    unsafe { &*(self as *const Self as *const CpuByteFields) }
  }
}
impl DerefMut for Cpu {
  #[inline]
  #[must_use]
  fn deref_mut(&mut self) -> &mut Self::Target {
    // Safety: same as Deref
    unsafe { &mut *(self as *mut Self as *mut CpuByteFields) }
  }
}

/// A view of the CPU with the data registers broken into individual bytes.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
// Note(Lokathor): To support big-endian we'd just have to swap the ordering of
// each pair. However, until big-endian support is really requested, it's better
// to not have two near-identical structs in the codebase.
#[cfg(target_endian = "little")]
pub struct CpuByteFields {
  pub flags: Flags,
  pub a: Reg8,
  pub c: Reg8,
  pub b: Reg8,
  pub e: Reg8,
  pub d: Reg8,
  pub l: Reg8,
  pub h: Reg8,
  pub sp: Reg16,
  pub pc: Reg16,
}

/// A 16-bit CPU register.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Reg16(u16);
unsafe impl bytemuck::Zeroable for Reg16 {}
unsafe impl bytemuck::Pod for Reg16 {}
unsafe impl bytemuck::TransparentWrapper<u16> for Reg16 {}

/// An 8-bit CPU register.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Reg8(u8);
unsafe impl bytemuck::Zeroable for Reg8 {}
unsafe impl bytemuck::Pod for Reg8 {}
unsafe impl bytemuck::TransparentWrapper<u8> for Reg8 {}

/// The flags register.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Flags(u8);
unsafe impl bytemuck::Zeroable for Flags {}
unsafe impl bytemuck::Pod for Flags {}
unsafe impl bytemuck::TransparentWrapper<u8> for Flags {}
impl Flags {
  #[inline]
  #[must_use]
  pub const fn z(self) -> bool {
    u8_get_bit::<7>(self.0)
  }
  #[inline]
  pub fn set_z(&mut self, val: bool) {
    self.0 = u8_with_bit::<7>(self.0, val);
  }
  #[inline]
  #[must_use]
  pub const fn n(self) -> bool {
    u8_get_bit::<6>(self.0)
  }
  #[inline]
  pub fn set_n(&mut self, val: bool) {
    self.0 = u8_with_bit::<6>(self.0, val);
  }
  #[inline]
  #[must_use]
  pub const fn h(self) -> bool {
    u8_get_bit::<5>(self.0)
  }
  #[inline]
  pub fn set_h(&mut self, val: bool) {
    self.0 = u8_with_bit::<5>(self.0, val);
  }
  #[inline]
  #[must_use]
  pub const fn c(self) -> bool {
    u8_get_bit::<4>(self.0)
  }
  #[inline]
  pub fn set_c(&mut self, val: bool) {
    self.0 = u8_with_bit::<4>(self.0, val);
  }
}
