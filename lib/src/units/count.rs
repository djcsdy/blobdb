pub trait Count {
    type Output;
    fn count(&self) -> Self::Output;
}
