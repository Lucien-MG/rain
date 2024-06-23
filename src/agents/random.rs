use rand::Rng;

pub struct RandomAgent {
    action_space: usize,
}

impl RandomAgent {
    pub fn new(action_space: usize) -> RandomAgent {
        RandomAgent { action_space }
    }

    pub fn action(&self) -> usize {
        rand::thread_rng().gen_range(0..self.action_space)
    }
}
