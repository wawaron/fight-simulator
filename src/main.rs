mod fighter;

use rand::{thread_rng, Rng};

//------------------------------------------------------------------------------
fn main() {
    const NUM_SIM: u64 = 100000;
    let mut rng = thread_rng();

    let mut stat_rounds: u64 = 0;
    let mut stat_total_rounds: u64 = 0;
    let mut stat_min_rounds: u64 = u64::MAX;
    let mut stat_max_rounds: u64 = 0;
    let mut stat_f1_win: u64 = 0;
    let mut stat_f2_win: u64 = 0;
    let mut stat_draw: u64 = 0;
    let mut stat_f1_attack: u64 = 0;
    let mut stat_f1_succ_attack: u64 = 0;
    let mut stat_f2_attack: u64 = 0;
    let mut stat_f2_succ_attack: u64 = 0;

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
    println!("\nSimulating {} fights...\n", NUM_SIM);
    for _ in 0..NUM_SIM {
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
                    stat_f1_succ_attack += 1;
                }
                stat_f1_attack += 1;
            }

            if f2_action == 2 {
                if f1_action != 3 {
                    f1.health -= 10.0;
                    stat_f2_succ_attack += 1;
                }
                stat_f2_attack += 1;
            }

            if (f1.health <= 0.0) || (f2.health <= 0.0) {
                fight_is_on = false;

                if (f1.health <= 0.0) && (f2.health <= 0.0) {
                    stat_draw += 1;
                } else if f1.health <= 0.0 {
                    stat_f2_win += 1;
                } else {
                    stat_f1_win += 1;
                }
            }
            stat_rounds += 1;
        }

        //------------------------------------------------------------------------------
        stat_total_rounds += stat_rounds;
        if stat_rounds < stat_min_rounds {
            stat_min_rounds = stat_rounds;
        }
        if stat_rounds > stat_max_rounds {
            stat_max_rounds = stat_rounds;
        }
        stat_rounds = 0;
        f1.health = 100.0;
        f2.health = 100.0;
    }

    //------------------------------------------------------------------------------

    let stat_f1_win_pct = (stat_f1_win as f64) * 100.0 / (NUM_SIM as f64);
    let stat_f2_win_pct = (stat_f2_win as f64) * 100.0 / (NUM_SIM as f64);
    let stat_draw_pct = (stat_draw as f64) * 100.0 / (NUM_SIM as f64);
    let stat_win_equity = if stat_f1_win_pct > stat_f2_win_pct {
        stat_f1_win_pct - stat_f2_win_pct
    } else {
        stat_f2_win_pct - stat_f1_win_pct
    };
    let stat_f1_attack_pct = (stat_f1_succ_attack as f64) * 100.0 / (stat_f1_attack as f64);
    let stat_f2_attack_pct = (stat_f2_succ_attack as f64) * 100.0 / (stat_f2_attack as f64);

    println!("Results:");
    println!("Equity: {:.2}", stat_win_equity);
    println!("Fighter1 won: {:.2}%", stat_f1_win_pct);
    println!("Fighter2 won: {:.2}%", stat_f2_win_pct);
    println!("Draw: {:.2}%", stat_draw_pct);
    println!("\nGeneral:");
    println!(
        "Average number of rounds: {}",
        (stat_total_rounds as f64 / NUM_SIM as f64).round()
    );
    println!("Minimum number of rounds: {}", stat_min_rounds);
    println!("Maximum number of rounds: {}", stat_max_rounds);

    println!("\nFighter1:");
    println!("Attack Success Rate: {:.2}%", stat_f1_attack_pct);

    println!("\nFighter2:");
    println!("Attack Success Rate: {:.2}%", stat_f2_attack_pct);

    println!("");
}
