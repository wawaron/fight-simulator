#[derive(Debug)]
pub struct Statistics {
    pub num_sims: u64,
    pub rounds: u64,
    pub total_rounds: u64,
    pub min_rounds: u64,
    pub max_rounds: u64,
    pub f1_win: u64,
    pub f2_win: u64,
    pub draw: u64,
    pub f1_attack: u64,
    pub f1_succ_attack: u64,
    pub f2_attack: u64,
    pub f2_succ_attack: u64,
    pub f1_win_pct: f64,
    pub f2_win_pct: f64,
    pub draw_pct: f64,
    pub win_equity: f64,
    pub f1_attack_pct: f64,
    pub f2_attack_pct: f64,
}

impl Statistics {
    pub fn new(num_sims: u64) -> Self {
        Self {
            num_sims,
            rounds: 0,
            total_rounds: 0,
            min_rounds: 0,
            max_rounds: 0,
            f1_win: 0,
            f2_win: 0,
            draw: 0,
            f1_attack: 0,
            f1_succ_attack: 0,
            f2_attack: 0,
            f2_succ_attack: 0,
            f1_win_pct: 0.0,
            f2_win_pct: 0.0,
            draw_pct: 0.0,
            win_equity: 0.0,
            f1_attack_pct: 0.0,
            f2_attack_pct: 0.0,
        }
    }

    pub fn calculate(mut self) -> Self {
        self.f1_win_pct = (self.f1_win as f64) * 100.0 / (self.num_sims as f64);
        self.f2_win_pct = (self.f2_win as f64) * 100.0 / (self.num_sims as f64);
        self.draw_pct = (self.draw as f64) * 100.0 / (self.num_sims as f64);
        self.win_equity = if self.f1_win_pct > self.f2_win_pct {
            self.f1_win_pct - self.f2_win_pct
        } else {
            self.f2_win_pct - self.f1_win_pct
        };
        self.f1_attack_pct = (self.f1_succ_attack as f64) * 100.0 / (self.f1_attack as f64);
        self.f2_attack_pct = (self.f2_succ_attack as f64) * 100.0 / (self.f2_attack as f64);

        self
    }

    pub fn join(mut self, join: &Self) -> Self {
        self.total_rounds += join.total_rounds;
        if (join.min_rounds < self.min_rounds) || (self.min_rounds == 0) {
            self.min_rounds = join.min_rounds;
        }
        if join.max_rounds > self.max_rounds {
            self.max_rounds = join.max_rounds;
        }
        self.f1_win += join.f1_win;
        self.f2_win += join.f2_win;
        self.draw += join.draw;
        self.f1_attack += join.f1_attack;
        self.f1_succ_attack += join.f1_succ_attack;
        self.f2_attack += join.f2_attack;
        self.f2_succ_attack += join.f2_succ_attack;

        self
    }

    pub fn display(&self) {
        println!("Results:");
        println!("Equity: {:.2}", self.win_equity);
        println!("Fighter1 won: {:.2}%", self.f1_win_pct);
        println!("Fighter2 won: {:.2}%", self.f2_win_pct);
        println!("Draw: {:.2}%", self.draw_pct);

        println!("\nGeneral:");
        println!(
            "Average number of rounds: {}",
            (self.total_rounds as f64 / self.num_sims as f64).round()
        );
        println!("Minimum number of rounds: {}", self.min_rounds);
        println!("Maximum number of rounds: {}", self.max_rounds);

        println!("\nFighter1:");
        println!("Attack Success Rate: {:.2}%", self.f1_attack_pct);

        println!("\nFighter2:");
        println!("Attack Success Rate: {:.2}%", self.f2_attack_pct);
    }
}
