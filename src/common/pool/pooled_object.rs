pub(crate) trait PooledObject {
    fn object(&self) -> Box<dyn PooledObject>;

    fn release(&self);
}
