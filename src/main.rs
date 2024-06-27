use std::thread;
use std::time::Instant;

pub mod agents;
pub mod environments;

fn main() {
    let mut env = environments::karmed::KArmedBanditEnv::new(10, 1000);

    let mut agent_0 = agents::random::RandomAgent::new(10);
    let mut agent_1 = agents::egreedy::Egreedy::new(10, 0.1);

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
    let mean_reward = train_env(&mut env, &mut agent_0, 2000);
    let mean_reward = train_env(&mut env, &mut agent_1, 2000);

    let elapsed = now.elapsed();

    println!("Elapsed: {:.?}", elapsed);
    println!("Mean Reward: {:.?}", mean_reward);
}

fn train_env<E: environments::Environement, A: agents::Agent>(
    env: &mut E,
    agent: &mut A,
    nb_run: u32,
) -> () {
    for _ in 0..nb_run {
        run_env(env, agent);
    }
}

fn run_env<E: environments::Environement, A: agents::Agent>(env: &mut E, agent: &mut A) -> f32 {
    let mut mean_reward = 0.0;

    env.reset();

    loop {
        let action = agent.action();

        let (reward, terminated) = env.step(action);

        mean_reward += reward;

        if terminated {
            break;
        }
    }

    // agent.learn(action, reward);

    mean_reward
}
