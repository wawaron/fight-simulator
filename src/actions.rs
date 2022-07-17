#[derive(Debug)]
pub struct ActionInfo {
    pub name: String,
    pub duration: u8,
    pub hit_frame: u8,
    pub damage: f64,
}

pub fn initialize_actions() -> [ActionInfo; 6] {
    [
        // 000 - Neutral
        ActionInfo {
            name: "Neutral".to_string(),
            duration: 0,
            hit_frame: 0,
            damage: 0.0,
        },
        // 001 - Wait
        ActionInfo {
            name: "Wait".to_string(),
            duration: 1,
            hit_frame: 0,
            damage: 0.0,
        },
        // 002 - Evade
        ActionInfo {
            name: "Evade".to_string(),
            duration: 1,
            hit_frame: 0,
            damage: 0.0,
        },
        // 003 - Block
        ActionInfo {
            name: "Block".to_string(),
            duration: 1,
            hit_frame: 0,
            damage: 0.0,
        },
        // 004 - Punch
        ActionInfo {
            name: "Punch".to_string(),
            duration: 2,
            hit_frame: 1,
            damage: 10.0,
        },
        // 005 - Kick
        ActionInfo {
            name: "Kick".to_string(),
            duration: 4,
            hit_frame: 2,
            damage: 25.0,
        },
    ]
}
