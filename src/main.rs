use std::fs::File;
use std::io::Write;
//use std::thread;
use std::time::Instant;

use crate::agents::Agent;

pub mod agents;
pub mod environments;

fn main() {
    let mut env = environments::karmed::KArmedBanditEnv::new(10, 1000);

    let mut agent_0 = agents::random::RandomAgent::new(10);
    let mut agent_1 = agents::egreedy::Egreedy::new(10, 0.1, 0.1);

    println!("Launch exps!");

    let now = Instant::now();

    /*let mut running_threads = Vec::with_capacity(experiences.len());

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
    }*/

    let mean_reward_1 = train_env(&mut env, &mut agent_0, 2000);
    let mean_reward_2 = train_env(&mut env, &mut agent_1, 2000);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.?}", elapsed);

    write_data_to_file(
        "/home/lucien/Workspace/Project/rain/random.txt",
        "random",
        &mean_reward_1,
    );
    write_data_to_file(
        "/home/lucien/Workspace/Project/rain/egreedy.txt",
        "egreedy",
        &mean_reward_2,
    );
}

fn write_data_to_file(file_name: &str, name: &str, data: &Vec<f32>) {
    let mut f = File::create(file_name).expect("Unable to create file");
    write!(f, "{}\n", name);
    for i in data {
        write!(f, "{}\n", i);
    }
}

fn train_env<E: environments::Environement, A: agents::Agent>(
    env: &mut E,
    agent: &mut A,
    nb_run: u32,
) -> Vec<f32> {
    let mut mean_rewards_tt = vec![0.0; 2000];

    for _ in 0..nb_run {
        let mean_rewards = run_env(env, agent);

        for i in 0..mean_rewards.len() {
            mean_rewards_tt[i] += mean_rewards[i];
        }
    }

    for i in 0..mean_rewards_tt.len() {
        mean_rewards_tt[i] = mean_rewards_tt[i] / 2000 as f32;
    }

    mean_rewards_tt
}

fn run_env<E: environments::Environement, A: agents::Agent>(
    env: &mut E,
    agent: &mut A,
) -> Vec<f32> {
    let mut rewards = vec![];

    let state = env.reset();

    loop {
        let action = agent.action();

        let (reward, terminated) = env.step(&action);

        agent.learn(action, reward);

        if terminated {
            break;
        }

        rewards.push(reward);
    }

    rewards
}
