pub mod karmed;

pub trait Environement {
    fn step(&mut self, actions: &Vec<f32>) -> (f32, bool);
    fn reset(&mut self) -> ();
}
