pub(crate) trait PooledObject {
    type Object;

    fn object(&self) -> Self::Object;

    fn release(&self);
}
