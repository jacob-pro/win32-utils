#[inline]
pub fn LOWORD(l: u32) -> u32 {
    l & 0xffff
}
