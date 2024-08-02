use rand::Rng;

use crate::agents;

pub struct Egreedy {
    action_space: usize,
    epsilon: f32,
    alpha: f32,

    action_index: usize,

    action_reward: Vec<f32>,
    action_taken: Vec<f32>,

    q_values: Vec<f32>,
}

impl Egreedy {
    pub fn new(action_space: usize, epsilon: f32, alpha: f32) -> Egreedy {
        Egreedy {
            action_space,
            epsilon,
            alpha,
            action_index: 0,
            action_reward: vec![0.0; action_space],
            action_taken: vec![0.0; action_space],
            q_values: vec![0.0; action_space],
        }
    }
}

impl agents::Agent for Egreedy {
    fn action(&mut self) -> Vec<f32> {
        let mut action_vec = vec![0.0; self.action_space];

        self.action_index = 0;
        let rand_action = rand::thread_rng().gen_range(0.0..1.0);

        if rand_action < self.epsilon {
            self.action_index = rand::thread_rng().gen_range(0..self.action_space);
            action_vec[self.action_index] = 1.0;
        } else {
            for (index, element) in self.q_values.iter().enumerate() {
                if *element > self.q_values[self.action_index] {
                    self.action_index = index;
                }
            }
            action_vec[self.action_index] = 1.0;
        }

        self.action_taken[self.action_index] += 1.0;

        action_vec
    }

    fn learn(&mut self, _action: Vec<f32>, reward: f32) -> () {
        self.q_values[self.action_index] +=
            self.alpha * (reward - self.q_values[self.action_index]);
    }
}
