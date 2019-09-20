#[inline]
pub fn extract(val: u32, off: u8, len: u8) -> u32 {
    debug_assert!(off < 32 && len < 32);
    (val >> off) & ((1u32 << len) - 1)
}

#[inline]
pub fn bit(val: u32, bit: u8) -> u32 {
    debug_assert!(bit < 32);
    (val >> bit) & 1
}

#[inline]
pub fn sign_extend(val: u32, len: u8) -> u32 {
    debug_assert!(len < 32);
    let off = 32 - len;
    (((val as i32) << off) >> off) as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn signed_conversions() {
        let val = 0xf0000000u32;
        let sval = val as i32;
        assert_eq!(0xff000000u32, (sval >> 4) as u32);
        assert_eq!(val, sval as u32);
        assert_eq!(0xf000u16 as i16 as u32, 0xfffff000u32);
    }

    #[test]
    fn test_overrotate() {
        let val = 0xf000000u32;
        assert_eq!(val.rotate_right(68), 0x0f00000u32);
    }

    #[test]
    fn test_shifts() {
        assert_eq!(shift_lsl(0x11000000, 4), (0x10000000, 1));
        assert_eq!(shift_lsl(0x11000000, 5), (0x20000000, 0));
        assert_eq!(shift_lsr(0x11, 1), (0x8, 1));
        assert_eq!(shift_lsr(0x11, 2), (0x4, 0));
        assert_eq!(shift_ror(0x11, 1), (0x80000008, 1));
    }

    #[test]
    fn test_sign_extend() {
        assert_eq!(0xfffffff0, sign_extend(0xf0, 8));
        assert_eq!(0x00000070, sign_extend(0x70, 8));
    }
}
