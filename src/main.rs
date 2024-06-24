use std::thread;
use std::time::Instant;

pub mod agents;
pub mod environments;

fn main() {
    let experiences = vec![
        (
            environments::karmed::NArmedBanditEnv::new(10, 1000),
            agents::random::RandomAgent::new(10),
        ),
        (
            environments::karmed::NArmedBanditEnv::new(10, 1000),
            agents::random::RandomAgent::new(10),
        ),
        (
            environments::karmed::NArmedBanditEnv::new(10, 1000),
            agents::random::RandomAgent::new(10),
        ),
    ];

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
    env: &mut environments::karmed::NArmedBanditEnv,
    agent: &mut agents::random::RandomAgent,
    nb_run: u32,
) -> () {
    for _ in 0..nb_run {
        run_env(env, agent);

        env.reset();
    }
}

fn run_env(
    env: &mut environments::karmed::NArmedBanditEnv,
    agent: &mut agents::random::RandomAgent,
) -> () {
    loop {
        let action = agent.action();

        let (_reward, terminated) = env.step(action);

        if terminated {
            break;
        }
    }
}
