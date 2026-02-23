/// Convert u8/u16/u32/u64 to i64, saturating on overflow.
/// u8/u16/u32 always fit; u64 values above i64::MAX become i64::MAX.

pub fn to_i64<T>(n: T) -> i64
where
  // Needed so we can lift to a wide, unsigned type first
  u128: From<T>,
{
  let v = u128::from(n);
  if v > i64::MAX as u128 {
    i64::MAX
  } else {
    v as i64
  }
}

/// Convert any signed integer (`i8`, `i16`, `i32`, `i64`, `i128`) into `u32`
/// using **saturating semantics**:
///
/// - Negative values clamp to `0`
/// - Values larger than `u32::MAX` clamp to `u32::MAX`
/// - Otherwise, return the exact value
pub fn to_u32<T>(n: T) -> u32
where
  i128: From<T>,
{
  let v = i128::from(n);

  if v <= 0 {
    0
  } else if v > u32::MAX as i128 {
    u32::MAX
  } else {
    v as u32
  }
}
