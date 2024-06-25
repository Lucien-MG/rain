use rand::Rng;

use crate::agents;

pub struct RandomAgent {
    action_space: usize,
    action_vec: Vec<f32>,
}

impl RandomAgent {
    pub fn new(action_space: usize) -> RandomAgent {
        RandomAgent {
            action_space,
            action_vec: vec![0.0; action_space],
        }
    }
}

impl agents::Agent for RandomAgent {
    fn action(&mut self) -> &Vec<f32> {
        let action_to_explore = rand::thread_rng().gen_range(0..self.action_space);
        self.action_vec[action_to_explore] = 1.0;
        &self.action_vec
    }

    fn learn(&mut self, action: Vec<f32>, reward: f32) -> () {}
}
