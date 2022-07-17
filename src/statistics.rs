#[derive(Debug)]
pub struct Statistics {
    pub num_sims: u64,
    pub frames: u64,
    pub total_frames: u64,
    pub min_frames: u64,
    pub max_frames: u64,
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
            frames: 0,
            total_frames: 0,
            min_frames: 0,
            max_frames: 0,
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
        self.total_frames += join.total_frames;
        if (join.min_frames < self.min_frames) || (self.min_frames == 0) {
            self.min_frames = join.min_frames;
        }
        if join.max_frames > self.max_frames {
            self.max_frames = join.max_frames;
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
            "Average number of frames: {}",
            (self.total_frames as f64 / self.num_sims as f64).round()
        );
        println!("Minimum number of frames: {}", self.min_frames);
        println!("Maximum number of frames: {}", self.max_frames);

        println!("\nFighter1:");
        println!("Attack Success Rate: {:.2}%", self.f1_attack_pct);

        println!("\nFighter2:");
        println!("Attack Success Rate: {:.2}%", self.f2_attack_pct);
    }

    pub fn update_stats_after_fight(&mut self) {
        self.total_frames += self.frames;
        if (self.frames < self.min_frames) || (self.min_frames == 0) {
            self.min_frames = self.frames;
        }
        if self.frames > self.max_frames {
            self.max_frames = self.frames;
        }
        self.frames = 0;
    }
}
