mod fighter;

use rand::{thread_rng, Rng};

//------------------------------------------------------------------------------
fn main() {
    let mut f1 = fighter::Fighter {
        name: "Ryu".to_string(),
        health: 100.0,
        offense: 50.0,
        defense: 50.0,
    };
    let mut f2 = fighter::Fighter {
        name: "Ken".to_string(),
        health: 100.0,
        offense: 50.0,
        defense: 50.0,
    };

    let mut stat_rounds = 0;
    let mut stat_f1_attack = 0;
    let mut stat_f1_succ_attack = 0;
    let mut stat_f2_attack = 0;
    let mut stat_f2_succ_attack = 0;

    let mut rng = thread_rng();
    let mut fight_is_on = true;
    println!("");
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
                println!("Double KO!");
            } else if f1.health <= 0.0 {
                println!("{} wins!", f2.name);
            } else {
                println!("{} wins!", f1.name);
            }
        }
        stat_rounds += 1;
    }

    println!("\nGeneral:");
    println!("Number of rounds: {}", stat_rounds);
    println!("\n{}:", f1.name);
    println!("Health remaining: {}", f1.health);
    println!("Attack success: {}/{}", stat_f1_succ_attack, stat_f1_attack);
    println!("\n{}:", f2.name);
    println!("Health remaining: {}", f2.health);
    println!("Attack success: {}/{}", stat_f2_succ_attack, stat_f2_attack);
    println!("");
}
