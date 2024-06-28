use rand::Rng;

use crate::agents;

pub struct Egreedy {
    action_space: usize,
    epsilon: f32,

    action_reward: Vec<f32>,
    action_taken: Vec<u32>,

    q_values: Vec<f32>,
}

impl Egreedy {
    pub fn new(action_space: usize, epsilon: f32) -> Egreedy {
        Egreedy {
            action_space,
            epsilon,
            action_reward: vec![0.0; action_space],
            action_taken: vec![0; action_space],
            q_values: vec![0.0; action_space],
        }
    }
}

impl agents::Agent for Egreedy {
    fn action(&mut self) -> Vec<f32> {
        let mut action_vec = vec![0.0; self.action_space];
        let rand_action = rand::thread_rng().gen_range(0.0..1.0);

        if rand_action < self.epsilon {
            let action_to_explore = rand::thread_rng().gen_range(0..self.action_space);
            action_vec[action_to_explore] = 1.0;
        } else {
            let mut index_max = 0;
            for (index, element) in self.q_values.iter().enumerate() {
                if *element > self.q_values[index_max] {
                    index_max = index;
                }
            }
            action_vec[index_max] = 1.0;
        }

        action_vec
    }

    fn learn(&mut self, action: Vec<f32>, reward: f32) -> () {
        self.q_values[0] += self.q_values[0] + reward;
    }
}
