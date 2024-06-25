use std::str::FromStr;

use rand::distributions::Standard;
use rand::prelude::*;

#[derive(Debug)]
pub struct KArmedBanditEnv {
    pub name: String,
    arms: Vec<f32>,
    nb_steps: u32,
    step: u32,
}

impl KArmedBanditEnv {
    pub fn new(nb_arms: usize, nb_steps: u32) -> KArmedBanditEnv {
        KArmedBanditEnv {
            name: String::from_str("k-armed").unwrap(),
            arms: (0..nb_arms)
                .map(|_| StdRng::from_entropy().sample(Standard))
                .collect(),
            nb_steps,
            step: 0,
        }
    }

    pub fn step(&mut self, a: usize) -> (f32, bool) {
        self.step += 1;
        (
            self.arms[a] + StdRng::from_entropy().sample::<f32, Standard>(Standard),
            self.step >= self.nb_steps,
        )
    }

    pub fn reset(&mut self) -> () {
        self.step = 0;
    }
}
