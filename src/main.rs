mod fighter;

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
}
