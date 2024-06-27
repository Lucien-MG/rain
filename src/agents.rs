//pub mod egreedy;
pub mod random;

pub trait Agent {
    fn action(&mut self) -> Vec<f32>;
    fn learn(&mut self, reward: f32) -> ();
}
