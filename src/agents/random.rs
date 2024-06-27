use rand::Rng;

use crate::agents;

pub struct RandomAgent {
    action_space: usize,
}

impl RandomAgent {
    pub fn new(action_space: usize) -> RandomAgent {
        RandomAgent { action_space }
    }
}

impl agents::Agent for RandomAgent {
    fn action(&mut self) -> Vec<f32> {
        let action_to_explore = rand::thread_rng().gen_range(0..self.action_space);
        let mut action = vec![0.0; self.action_space];

        action[action_to_explore] = 1.0;

        action
    }

    fn learn(&mut self, _reward: f32) -> () {}
}
