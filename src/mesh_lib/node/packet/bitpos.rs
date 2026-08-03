use core::ops::BitAnd;

#[inline]
pub fn is_flag_set(bit_field: u8, bit_mask: u8) -> bool {
    bit_field.bitand(bit_mask) == bit_mask
}

#[inline]
pub fn set_flag(bit_field: &mut u8, bit_mask: u8, new_state: bool) {
    *bit_field = (*bit_field & !bit_mask) | (bit_mask & (new_state as u8).wrapping_neg());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_flag_set_true() {
        let bit_field: u8 = 0b00000001;
        assert!(is_flag_set(bit_field, 0b00000001));
    }
    #[test]
    fn test_if_flag_set_false() {
        let bit_field: u8 = 0b00000000;
        assert!(!is_flag_set(bit_field, 0b00000001));
    }
    #[test]
    fn set_flag_from_false_to_true() {
        let mut bit_field: u8 = 0b00000000;
        set_flag(&mut bit_field, 0b00000001, true);
        assert!(is_flag_set(bit_field, 0b00000001));
    }
    #[test]
    fn set_flag_from_true_to_false() {
        let mut bit_field: u8 = 0b00000001;
        set_flag(&mut bit_field, 0b00000001, false);
        assert!(!is_flag_set(bit_field, 0b00000001));
    }
    #[test]
    fn set_flag_from_true_to_true() {
        let mut bit_field: u8 = 0b00000001;
        set_flag(&mut bit_field, 0b00000001, true);
        assert!(is_flag_set(bit_field, 0b00000001));
    }
    #[test]
    fn set_flag_from_false_to_false() {
        let mut bit_field: u8 = 0b00000000;
        set_flag(&mut bit_field, 0b00000001, false);
        assert!(!is_flag_set(bit_field, 0b00000001));
    }
    #[test]
    fn set_second_flag() {
        let mut bit_field: u8 = 0b00000000;
        set_flag(&mut bit_field, 0b00000001, true);
        assert!(is_flag_set(bit_field, 0b00000001));
        set_flag(&mut bit_field, 0b00000010, true);
        assert!(is_flag_set(bit_field, 0b00000010));
    }
}
