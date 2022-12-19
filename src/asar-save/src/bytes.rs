use crate::AsarError;

/// A simplified and const generic version of arrayref
#[inline]
pub(crate) fn take<const N: usize>(data: &[u8]) -> [u8; N] {
    debug_assert!(data.len() >= N);
    unsafe { *(data.as_ptr() as *const [u8; N]) }
}

#[inline]
pub(crate) fn get_split<const N: usize>(data: &[u8]) -> Option<([u8; N], &[u8])> {
    if N <= data.len() {
        let (head, tail) = data.split_at(N);
        Some((take::<N>(head), tail))
    } else {
        None
    }
}

#[inline]
pub(crate) fn get_u32(data: &[u8]) -> Result<(u32, &[u8]), AsarError> {
    let (num, rem) = get_split::<4>(data).ok_or(AsarError::Eof)?;
    Ok((u32::from_le_bytes(num), rem))
}
