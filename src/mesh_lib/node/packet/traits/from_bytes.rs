pub trait FromBytes<const TYPE_SIZE: usize> {
    fn from_be_bytes(bytes: [u8; TYPE_SIZE]) -> Self;
}
