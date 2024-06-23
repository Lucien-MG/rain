use rand::distributions::Standard;
use rand::prelude::*;

#[derive(Debug)]
pub struct NArmedBanditEnv {
    arms: Vec<f32>,
}

impl NArmedBanditEnv {
    pub fn new(nb_arms: usize) -> NArmedBanditEnv {
        NArmedBanditEnv {
            arms: (0..nb_arms)
                .map(|_| StdRng::from_entropy().sample(Standard))
                .collect(),
        }
    }

    pub fn step(&self, a: usize) -> f32 {
        self.arms[a] + StdRng::from_entropy().sample::<f32, Standard>(Standard)
    }
}
