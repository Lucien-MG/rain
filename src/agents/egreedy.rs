use rand::Rng;

pub struct Egreedy {
    action_space: usize,
    epsilon: f32,

    q_values: Vec<f32>,
}

impl Egreedy {
    pub fn new(action_space: usize, epsilon: f32) -> Egreedy {
        Egreedy {
            action_space,
            epsilon,
            q_values: vec![0.0; action_space],
        }
    }

    pub fn action(&self) -> usize {
        let rand_action = rand::thread_rng().gen_range(0.0..1.0);

        if rand_action < self.epsilon {
            rand::thread_rng().gen_range(0..self.action_space)
        } else {
            self.q_values
                .iter()
                .enumerate()
                .max_by(|(_, value0), (_, value1)| value0.cmp(value1))
                .map(|(idx, _)| idx)
                .unwrap()
        }
    }

    pub fn learn(&self, action: usize, reward: f32) -> () {
        self.q_values[action] += self.q_values[action];
    }
}
