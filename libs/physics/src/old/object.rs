pub trait Object {
    fn destroy(&mut self);
    fn is_destoryed(&self);
}
