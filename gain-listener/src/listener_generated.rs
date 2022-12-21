// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_BIND_ERROR: i16 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_BIND_ERROR: i16 = 6;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_BIND_ERROR: [BindError; 7] = [
  BindError::None,
  BindError::TooManyBindings,
  BindError::AlreadyBound,
  BindError::InvalidAcceptSize,
  BindError::InvalidName,
  BindError::NameTooLong,
  BindError::UnsupportedPort,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct BindError(pub i16);
#[allow(non_upper_case_globals)]
impl BindError {
  pub const None: Self = Self(0);
  pub const TooManyBindings: Self = Self(1);
  pub const AlreadyBound: Self = Self(2);
  pub const InvalidAcceptSize: Self = Self(3);
  pub const InvalidName: Self = Self(4);
  pub const NameTooLong: Self = Self(5);
  pub const UnsupportedPort: Self = Self(6);

  pub const ENUM_MIN: i16 = 0;
  pub const ENUM_MAX: i16 = 6;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::None,
    Self::TooManyBindings,
    Self::AlreadyBound,
    Self::InvalidAcceptSize,
    Self::InvalidName,
    Self::NameTooLong,
    Self::UnsupportedPort,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::None => Some("None"),
      Self::TooManyBindings => Some("TooManyBindings"),
      Self::AlreadyBound => Some("AlreadyBound"),
      Self::InvalidAcceptSize => Some("InvalidAcceptSize"),
      Self::InvalidName => Some("InvalidName"),
      Self::NameTooLong => Some("NameTooLong"),
      Self::UnsupportedPort => Some("UnsupportedPort"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for BindError {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for BindError {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<i16>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for BindError {
    type Output = BindError;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<i16>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for BindError {
  type Scalar = i16;
  #[inline]
  fn to_little_endian(self) -> i16 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: i16) -> Self {
    let b = i16::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for BindError {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    i16::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for BindError {}
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_ACCEPT_ERROR: i16 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_ACCEPT_ERROR: i16 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_ACCEPT_ERROR: [AcceptError; 1] = [
  AcceptError::None,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct AcceptError(pub i16);
#[allow(non_upper_case_globals)]
impl AcceptError {
  pub const None: Self = Self(0);

  pub const ENUM_MIN: i16 = 0;
  pub const ENUM_MAX: i16 = 0;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::None,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::None => Some("None"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for AcceptError {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for AcceptError {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<i16>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for AcceptError {
    type Output = AcceptError;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<i16>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for AcceptError {
  type Scalar = i16;
  #[inline]
  fn to_little_endian(self) -> i16 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: i16) -> Self {
    let b = i16::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for AcceptError {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    i16::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for AcceptError {}
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_ACCEPT_SIZE: i16 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_ACCEPT_SIZE: i16 = 44;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_ACCEPT_SIZE: [AcceptSize; 2] = [
  AcceptSize::Invalid,
  AcceptSize::Basic,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct AcceptSize(pub i16);
#[allow(non_upper_case_globals)]
impl AcceptSize {
  pub const Invalid: Self = Self(0);
  pub const Basic: Self = Self(44);

  pub const ENUM_MIN: i16 = 0;
  pub const ENUM_MAX: i16 = 44;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::Invalid,
    Self::Basic,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::Invalid => Some("Invalid"),
      Self::Basic => Some("Basic"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for AcceptSize {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for AcceptSize {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<i16>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for AcceptSize {
    type Output = AcceptSize;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<i16>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for AcceptSize {
  type Scalar = i16;
  #[inline]
  fn to_little_endian(self) -> i16 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: i16) -> Self {
    let b = i16::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for AcceptSize {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    i16::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for AcceptSize {}
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_FUNCTION: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_FUNCTION: u8 = 1;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_FUNCTION: [Function; 2] = [
  Function::NONE,
  Function::BindTLS,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Function(pub u8);
#[allow(non_upper_case_globals)]
impl Function {
  pub const NONE: Self = Self(0);
  pub const BindTLS: Self = Self(1);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 1;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NONE,
    Self::BindTLS,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NONE => Some("NONE"),
      Self::BindTLS => Some("BindTLS"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for Function {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for Function {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for Function {
    type Output = Function;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for Function {
  type Scalar = u8;
  #[inline]
  fn to_little_endian(self) -> u8 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: u8) -> Self {
    let b = u8::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for Function {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Function {}
pub struct FunctionUnionTableOffset {}

// struct IPAddr, aligned to 4
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct IPAddr(pub [u8; 16]);
impl Default for IPAddr { 
  fn default() -> Self { 
    Self([0; 16])
  }
}
impl core::fmt::Debug for IPAddr {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.debug_struct("IPAddr")
      .field("a", &self.a())
      .field("b", &self.b())
      .field("c", &self.c())
      .field("d", &self.d())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for IPAddr {}
impl<'a> flatbuffers::Follow<'a> for IPAddr {
  type Inner = &'a IPAddr;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a IPAddr>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a IPAddr {
  type Inner = &'a IPAddr;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<IPAddr>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for IPAddr {
    type Output = IPAddr;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        let src = ::core::slice::from_raw_parts(self as *const IPAddr as *const u8, Self::size());
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for IPAddr {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}

impl<'a> IPAddr {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    a: u32,
    b: u32,
    c: u32,
    d: u32,
  ) -> Self {
    let mut s = Self([0; 16]);
    s.set_a(a);
    s.set_b(b);
    s.set_c(c);
    s.set_d(d);
    s
  }

  pub fn a(&self) -> u32 {
    let mut mem = core::mem::MaybeUninit::<<u32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<u32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_a(&mut self, x: u32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<<u32 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn b(&self) -> u32 {
    let mut mem = core::mem::MaybeUninit::<<u32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[4..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<u32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_b(&mut self, x: u32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[4..].as_mut_ptr(),
        core::mem::size_of::<<u32 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn c(&self) -> u32 {
    let mut mem = core::mem::MaybeUninit::<<u32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[8..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<u32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_c(&mut self, x: u32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[8..].as_mut_ptr(),
        core::mem::size_of::<<u32 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn d(&self) -> u32 {
    let mut mem = core::mem::MaybeUninit::<<u32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[12..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<u32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_d(&mut self, x: u32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[12..].as_mut_ptr(),
        core::mem::size_of::<<u32 as EndianScalar>::Scalar>(),
      );
    }
  }

}

// struct AcceptBasic, aligned to 4
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct AcceptBasic(pub [u8; 28]);
impl Default for AcceptBasic { 
  fn default() -> Self { 
    Self([0; 28])
  }
}
impl core::fmt::Debug for AcceptBasic {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.debug_struct("AcceptBasic")
      .field("error", &self.error())
      .field("conn_id", &self.conn_id())
      .field("addr", &self.addr())
      .field("port", &self.port())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for AcceptBasic {}
impl<'a> flatbuffers::Follow<'a> for AcceptBasic {
  type Inner = &'a AcceptBasic;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a AcceptBasic>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a AcceptBasic {
  type Inner = &'a AcceptBasic;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<AcceptBasic>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for AcceptBasic {
    type Output = AcceptBasic;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        let src = ::core::slice::from_raw_parts(self as *const AcceptBasic as *const u8, Self::size());
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for AcceptBasic {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}

impl<'a> AcceptBasic {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    error: AcceptError,
    conn_id: i32,
    addr: &IPAddr,
    port: u16,
  ) -> Self {
    let mut s = Self([0; 28]);
    s.set_error(error);
    s.set_conn_id(conn_id);
    s.set_addr(addr);
    s.set_port(port);
    s
  }

  pub fn error(&self) -> AcceptError {
    let mut mem = core::mem::MaybeUninit::<<AcceptError as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<AcceptError as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_error(&mut self, x: AcceptError) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<<AcceptError as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn conn_id(&self) -> i32 {
    let mut mem = core::mem::MaybeUninit::<<i32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[4..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<i32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_conn_id(&mut self, x: i32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[4..].as_mut_ptr(),
        core::mem::size_of::<<i32 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn addr(&self) -> &IPAddr {
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid struct in this slot
    unsafe { &*(self.0[8..].as_ptr() as *const IPAddr) }
  }

  #[allow(clippy::identity_op)]
  pub fn set_addr(&mut self, x: &IPAddr) {
    self.0[8..8 + 16].copy_from_slice(&x.0)
  }

  pub fn port(&self) -> u16 {
    let mut mem = core::mem::MaybeUninit::<<u16 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[24..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<u16 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_port(&mut self, x: u16) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[24..].as_mut_ptr(),
        core::mem::size_of::<<u16 as EndianScalar>::Scalar>(),
      );
    }
  }

}

pub enum AcceptOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Accept<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Accept<'a> {
  type Inner = Accept<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Accept<'a> {
  pub const VT_BASIC: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Accept { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args AcceptArgs<'args>
  ) -> flatbuffers::WIPOffset<Accept<'bldr>> {
    let mut builder = AcceptBuilder::new(_fbb);
    if let Some(x) = args.basic { builder.add_basic(x); }
    builder.finish()
  }


  #[inline]
  pub fn basic(&self) -> Option<&'a AcceptBasic> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<AcceptBasic>(Accept::VT_BASIC, None)}
  }
}

impl flatbuffers::Verifiable for Accept<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<AcceptBasic>("basic", Self::VT_BASIC, false)?
     .finish();
    Ok(())
  }
}
pub struct AcceptArgs<'a> {
    pub basic: Option<&'a AcceptBasic>,
}
impl<'a> Default for AcceptArgs<'a> {
  #[inline]
  fn default() -> Self {
    AcceptArgs {
      basic: None,
    }
  }
}

pub struct AcceptBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> AcceptBuilder<'a, 'b> {
  #[inline]
  pub fn add_basic(&mut self, basic: &AcceptBasic) {
    self.fbb_.push_slot_always::<&AcceptBasic>(Accept::VT_BASIC, basic);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> AcceptBuilder<'a, 'b> {
    let start = _fbb.start_table();
    AcceptBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Accept<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Accept<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Accept");
      ds.field("basic", &self.basic());
      ds.finish()
  }
}
pub enum BindTLSOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct BindTLS<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for BindTLS<'a> {
  type Inner = BindTLS<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> BindTLS<'a> {
  pub const VT_ACCEPT_SIZE: flatbuffers::VOffsetT = 4;
  pub const VT_NAME: flatbuffers::VOffsetT = 6;
  pub const VT_PORT: flatbuffers::VOffsetT = 8;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    BindTLS { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args BindTLSArgs<'args>
  ) -> flatbuffers::WIPOffset<BindTLS<'bldr>> {
    let mut builder = BindTLSBuilder::new(_fbb);
    if let Some(x) = args.name { builder.add_name(x); }
    builder.add_port(args.port);
    builder.add_accept_size(args.accept_size);
    builder.finish()
  }


  #[inline]
  pub fn accept_size(&self) -> AcceptSize {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<AcceptSize>(BindTLS::VT_ACCEPT_SIZE, Some(AcceptSize::Invalid)).unwrap()}
  }
  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(BindTLS::VT_NAME, None)}
  }
  #[inline]
  pub fn port(&self) -> u16 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u16>(BindTLS::VT_PORT, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for BindTLS<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<AcceptSize>("accept_size", Self::VT_ACCEPT_SIZE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, false)?
     .visit_field::<u16>("port", Self::VT_PORT, false)?
     .finish();
    Ok(())
  }
}
pub struct BindTLSArgs<'a> {
    pub accept_size: AcceptSize,
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub port: u16,
}
impl<'a> Default for BindTLSArgs<'a> {
  #[inline]
  fn default() -> Self {
    BindTLSArgs {
      accept_size: AcceptSize::Invalid,
      name: None,
      port: 0,
    }
  }
}

pub struct BindTLSBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> BindTLSBuilder<'a, 'b> {
  #[inline]
  pub fn add_accept_size(&mut self, accept_size: AcceptSize) {
    self.fbb_.push_slot::<AcceptSize>(BindTLS::VT_ACCEPT_SIZE, accept_size, AcceptSize::Invalid);
  }
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(BindTLS::VT_NAME, name);
  }
  #[inline]
  pub fn add_port(&mut self, port: u16) {
    self.fbb_.push_slot::<u16>(BindTLS::VT_PORT, port, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> BindTLSBuilder<'a, 'b> {
    let start = _fbb.start_table();
    BindTLSBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<BindTLS<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for BindTLS<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("BindTLS");
      ds.field("accept_size", &self.accept_size());
      ds.field("name", &self.name());
      ds.field("port", &self.port());
      ds.finish()
  }
}
pub enum BindingOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Binding<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Binding<'a> {
  type Inner = Binding<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Binding<'a> {
  pub const VT_ERROR: flatbuffers::VOffsetT = 4;
  pub const VT_LISTEN_ID: flatbuffers::VOffsetT = 6;
  pub const VT_HOST: flatbuffers::VOffsetT = 8;
  pub const VT_PORT: flatbuffers::VOffsetT = 10;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Binding { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args BindingArgs<'args>
  ) -> flatbuffers::WIPOffset<Binding<'bldr>> {
    let mut builder = BindingBuilder::new(_fbb);
    if let Some(x) = args.host { builder.add_host(x); }
    builder.add_listen_id(args.listen_id);
    builder.add_port(args.port);
    builder.add_error(args.error);
    builder.finish()
  }


  #[inline]
  pub fn error(&self) -> BindError {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<BindError>(Binding::VT_ERROR, Some(BindError::None)).unwrap()}
  }
  #[inline]
  pub fn listen_id(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(Binding::VT_LISTEN_ID, Some(0)).unwrap()}
  }
  #[inline]
  pub fn host(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Binding::VT_HOST, None)}
  }
  #[inline]
  pub fn port(&self) -> u16 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u16>(Binding::VT_PORT, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for Binding<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<BindError>("error", Self::VT_ERROR, false)?
     .visit_field::<i32>("listen_id", Self::VT_LISTEN_ID, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("host", Self::VT_HOST, false)?
     .visit_field::<u16>("port", Self::VT_PORT, false)?
     .finish();
    Ok(())
  }
}
pub struct BindingArgs<'a> {
    pub error: BindError,
    pub listen_id: i32,
    pub host: Option<flatbuffers::WIPOffset<&'a str>>,
    pub port: u16,
}
impl<'a> Default for BindingArgs<'a> {
  #[inline]
  fn default() -> Self {
    BindingArgs {
      error: BindError::None,
      listen_id: 0,
      host: None,
      port: 0,
    }
  }
}

pub struct BindingBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> BindingBuilder<'a, 'b> {
  #[inline]
  pub fn add_error(&mut self, error: BindError) {
    self.fbb_.push_slot::<BindError>(Binding::VT_ERROR, error, BindError::None);
  }
  #[inline]
  pub fn add_listen_id(&mut self, listen_id: i32) {
    self.fbb_.push_slot::<i32>(Binding::VT_LISTEN_ID, listen_id, 0);
  }
  #[inline]
  pub fn add_host(&mut self, host: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Binding::VT_HOST, host);
  }
  #[inline]
  pub fn add_port(&mut self, port: u16) {
    self.fbb_.push_slot::<u16>(Binding::VT_PORT, port, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> BindingBuilder<'a, 'b> {
    let start = _fbb.start_table();
    BindingBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Binding<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Binding<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Binding");
      ds.field("error", &self.error());
      ds.field("listen_id", &self.listen_id());
      ds.field("host", &self.host());
      ds.field("port", &self.port());
      ds.finish()
  }
}
pub enum CallOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Call<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Call<'a> {
  type Inner = Call<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Call<'a> {
  pub const VT_FUNCTION_TYPE: flatbuffers::VOffsetT = 4;
  pub const VT_FUNCTION: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Call { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args CallArgs
  ) -> flatbuffers::WIPOffset<Call<'bldr>> {
    let mut builder = CallBuilder::new(_fbb);
    if let Some(x) = args.function { builder.add_function(x); }
    builder.add_function_type(args.function_type);
    builder.finish()
  }


  #[inline]
  pub fn function_type(&self) -> Function {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<Function>(Call::VT_FUNCTION_TYPE, Some(Function::NONE)).unwrap()}
  }
  #[inline]
  pub fn function(&self) -> Option<flatbuffers::Table<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(Call::VT_FUNCTION, None)}
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn function_as_bind_tls(&self) -> Option<BindTLS<'a>> {
    if self.function_type() == Function::BindTLS {
      self.function().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { BindTLS::init_from_table(t) }
     })
    } else {
      None
    }
  }

}

impl flatbuffers::Verifiable for Call<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_union::<Function, _>("function_type", Self::VT_FUNCTION_TYPE, "function", Self::VT_FUNCTION, false, |key, v, pos| {
        match key {
          Function::BindTLS => v.verify_union_variant::<flatbuffers::ForwardsUOffset<BindTLS>>("Function::BindTLS", pos),
          _ => Ok(()),
        }
     })?
     .finish();
    Ok(())
  }
}
pub struct CallArgs {
    pub function_type: Function,
    pub function: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for CallArgs {
  #[inline]
  fn default() -> Self {
    CallArgs {
      function_type: Function::NONE,
      function: None,
    }
  }
}

pub struct CallBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> CallBuilder<'a, 'b> {
  #[inline]
  pub fn add_function_type(&mut self, function_type: Function) {
    self.fbb_.push_slot::<Function>(Call::VT_FUNCTION_TYPE, function_type, Function::NONE);
  }
  #[inline]
  pub fn add_function(&mut self, function: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Call::VT_FUNCTION, function);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> CallBuilder<'a, 'b> {
    let start = _fbb.start_table();
    CallBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Call<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Call<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Call");
      ds.field("function_type", &self.function_type());
      match self.function_type() {
        Function::BindTLS => {
          if let Some(x) = self.function_as_bind_tls() {
            ds.field("function", &x)
          } else {
            ds.field("function", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        _ => {
          let x: Option<()> = None;
          ds.field("function", &x)
        },
      };
      ds.finish()
  }
}
