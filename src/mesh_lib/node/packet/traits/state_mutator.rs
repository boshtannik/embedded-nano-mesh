pub trait StateMutator {
    fn mutated(self) -> Self;
}
