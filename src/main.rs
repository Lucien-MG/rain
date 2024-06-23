use std::thread;
use std::time::Instant;

pub mod agents;
pub mod environments;

fn main() {
    println!("Launch exp!");

    let now = Instant::now();

    let mut running_threads = Vec::with_capacity(10);

    for i in 0..8 {
        let handle = thread::spawn(move || {
            println!("Run exp {}", i);
            let size = 10;
            let agent = agents::random::RandomAgent::new(size);
            let env = environments::NArmedBanditEnv::new(size);
            run_env(&env, &agent);
            println!("End exp {}", i);
            // println!("Test {:#?}", env);
        });
        running_threads.push(handle);
    }

    for thread in running_threads {
        thread.join();
    }

    let elapsed = now.elapsed();

    println!("Elapsed: {:.?}", elapsed);
}

fn run_env(env: &environments::NArmedBanditEnv, agent: &agents::random::RandomAgent) -> () {
    for _ in 0..1_000_000 {
        let action = agent.action();
        let _res = (&env).step(action);
        //        println!("test: {}.", env.step(action));
    }
}
