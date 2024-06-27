use std::str::FromStr;

use rand::distributions::Standard;
use rand::prelude::*;

use crate::environments;

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
}

impl environments::Environement for KArmedBanditEnv {
    fn step(&mut self, actions: &Vec<f32>) -> (f32, bool) {
        self.step += 1;
        (
            self.arms[0] + StdRng::from_entropy().sample::<f32, Standard>(Standard),
            self.step >= self.nb_steps,
        )
    }

    fn reset(&mut self) -> () {
        self.step = 0;
    }
}
