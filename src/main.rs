mod fighter;
mod statistics;

use rand::{thread_rng, Rng};
use std::sync::mpsc;
use std::thread;

//------------------------------------------------------------------------------
fn main() {
    const NUM_THREADS: u64 = 10;
    const NUM_SIMS_PER_THREAD: u64 = 100000;
    const NUM_SIMS: u64 = NUM_THREADS * NUM_SIMS_PER_THREAD;

    println!(
        "\nSimulating {} fights on {} threads...\n",
        NUM_SIMS, NUM_THREADS
    );

    //------------------------------------------------------------------------------
    let (tx, rx) = mpsc::channel();

    for _ in 0..NUM_THREADS {
        let tx = tx.clone();
        thread::spawn(move || {
            let mut rng = thread_rng();
            let mut stats = statistics::Statistics::new(NUM_SIMS);
            let mut f1 = fighter::Fighter {
                health: 100.0,
                offense: 50.0,
                defense: 50.0,
            };
            let mut f2 = fighter::Fighter {
                health: 100.0,
                offense: 50.0,
                defense: 50.0,
            };

            //------------------------------------------------------------------------------
            for _ in 0..NUM_SIMS_PER_THREAD {
                let mut fight_is_on = true;

                while fight_is_on {
                    // Fighters choose an action:
                    // 1. Standing (1/3)
                    // 2. Attack (1/3)
                    // 3. Defense (1/3)
                    let f1_action: u8 = rng.gen_range(1..=3);
                    let f2_action: u8 = rng.gen_range(1..=3);

                    if f1_action == 2 {
                        if f2_action != 3 {
                            f2.health -= 10.0;
                            stats.f1_succ_attack += 1;
                        }
                        stats.f1_attack += 1;
                    }

                    if f2_action == 2 {
                        if f1_action != 3 {
                            f1.health -= 10.0;
                            stats.f2_succ_attack += 1;
                        }
                        stats.f2_attack += 1;
                    }

                    if (f1.health <= 0.0) || (f2.health <= 0.0) {
                        fight_is_on = false;

                        if (f1.health <= 0.0) && (f2.health <= 0.0) {
                            stats.draw += 1;
                        } else if f1.health <= 0.0 {
                            stats.f2_win += 1;
                        } else {
                            stats.f1_win += 1;
                        }
                    }

                    stats.rounds += 1;
                }

                //------------------------------------------------------------------------------
                stats.total_rounds += stats.rounds;
                if (stats.rounds < stats.min_rounds) || (stats.min_rounds == 0) {
                    stats.min_rounds = stats.rounds;
                }
                if stats.rounds > stats.max_rounds {
                    stats.max_rounds = stats.rounds;
                }
                stats.rounds = 0;
                f1.health = 100.0;
                f2.health = 100.0;
            }

            tx.send(stats).expect("Transmission failed");
        });
    }

    drop(tx);

    //------------------------------------------------------------------------------
    let mut global_stats = statistics::Statistics::new(NUM_SIMS);
    for received in rx {
        global_stats = global_stats.join(&received);
    }
    global_stats.calculate().display();
    println!("");
}
