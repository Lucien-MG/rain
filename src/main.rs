use std::thread;
use std::time::Instant;

use crate::agents::agent;

pub mod agents;
pub mod environments;

fn main() {
    let exp1 = (
        environments::karmed::KArmedBanditEnv::new(10, 1000),
        Box::new(agents::random::RandomAgent::new(10)),
    );
    let exp2 = (
        environments::karmed::KArmedBanditEnv::new(10, 1000),
        Box::new(agents::egreedy::Egreedy::new(10, 0.1)),
    );
    let experiences: Vec<(environments::karmed::KArmedBanditEnv, &dyn agent::Agent)> = Vec::new();
    let tests: Vec<Box<dyn agent::Agent>> = Vec::new();

    let test: agent::Agent = agents::egreedy::Egreedy::new(10, 0.1);
    tests.push(Box::new(test));

    println!("Launch exps!");

    let now = Instant::now();

    let mut running_threads = Vec::with_capacity(experiences.len());

    for exp in experiences {
        let (mut env, mut agent) = exp;

        let handle = thread::spawn(move || {
            println!("-> Start exp {}", env.name);
            train_env(&mut env, &mut agent, 2000);
            println!("<- End exp {}", env.name);
            // println!("Test {:#?}", env);
        });
        running_threads.push(handle);
    }

    for thread in running_threads {
        let _ = thread.join();
    }

    let elapsed = now.elapsed();

    println!("Elapsed: {:.?}", elapsed);
}

fn train_env(
    env: &mut environments::karmed::KArmedBanditEnv,
    agent: &mut dyn agents::Agent,
    nb_run: u32,
) -> () {
    for _ in 0..nb_run {
        run_env(env, agent);
        env.reset();
    }
}

fn run_env(env: &mut environments::karmed::KArmedBanditEnv, agent: &mut dyn agents::Agent) -> () {
    loop {
        let action = agent.action();

        let (_reward, terminated) = env.step(action);

        if terminated {
            break;
        }
    }
}
