mod actions;
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
            let actions = actions::initialize_actions();
            let mut stats = statistics::Statistics::new(NUM_SIMS);
            let mut f1 = fighter::Fighter {
                health: 100.0,
                offense: 50.0,
                defense: 50.0,
                action: 0,
                anim_time: 0,
            };
            let mut f2 = fighter::Fighter {
                health: 100.0,
                offense: 50.0,
                defense: 50.0,
                action: 0,
                anim_time: 0,
            };

            //------------------------------------------------------------------------------
            for _ in 0..NUM_SIMS_PER_THREAD {
                let mut fight_is_on = true;

                while fight_is_on {
                    if f1.anim_time > 0 {
                        f1.anim_time -= 1;
                    } else {
                        f1.action = rng.gen_range(1..6);
                        f1.anim_time = actions[f1.action].duration;

                        if (f1.action == 4) || (f1.action == 5) {
                            stats.f1_attack += 1;
                        }
                    }

                    if f2.anim_time > 0 {
                        f2.anim_time -= 1;
                    } else {
                        f2.action = rng.gen_range(1..6);
                        f2.anim_time = actions[f2.action].duration;

                        if (f2.action == 4) || (f2.action == 5) {
                            stats.f2_attack += 1;
                        }
                    }

                    if (f1.anim_time == actions[f1.action].hit_frame) && (f1.anim_time > 0) {
                        match f2.action {
                            1 => {
                                f2.health -= actions[f1.action].damage;
                                stats.f1_succ_attack += 1;
                            }
                            2 => {}
                            3 => {
                                f2.health -= actions[f1.action].damage / 2.0;
                            }
                            4 => {
                                f2.health -= actions[f1.action].damage * 1.5;
                                stats.f1_succ_attack += 1;
                            }
                            5 => {
                                f2.health -= actions[f1.action].damage * 1.5;
                                stats.f1_succ_attack += 1;
                            }
                            _ => {}
                        }
                    }

                    if (f2.anim_time == actions[f2.action].hit_frame) && (f2.anim_time > 0) {
                        match f1.action {
                            1 => {
                                f1.health -= actions[f2.action].damage;
                                stats.f2_succ_attack += 1;
                            }
                            2 => {}
                            3 => {
                                f1.health -= actions[f2.action].damage / 2.0;
                            }
                            4 => {
                                f1.health -= actions[f2.action].damage * 1.5;
                                stats.f2_succ_attack += 1;
                            }
                            5 => {
                                f1.health -= actions[f2.action].damage * 1.5;
                                stats.f2_succ_attack += 1;
                            }
                            _ => {}
                        }
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

                    stats.frames += 1;
                }

                //------------------------------------------------------------------------------
                f1.health = 100.0;
                f2.health = 100.0;
                stats.update_stats_after_fight();
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
