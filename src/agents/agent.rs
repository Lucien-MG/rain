pub trait Agent {
    fn action(&self) -> Vec<f32>;
    fn learn(&self) -> ();
}
