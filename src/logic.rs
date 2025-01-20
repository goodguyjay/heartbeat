use rand::Rng;

#[derive(PartialEq)]
pub enum Action {
    Stimulate,
    SlowDown,
    Quit,
}

pub struct Game {
    pub heartbeat: i32,
}

impl Game {
    pub fn new() -> Self {
        Self { heartbeat: 72 }
    }

    pub fn apply_action(&mut self, action: Action) {
        let mut rng = rand::thread_rng();

        match action {
            Action::Stimulate => {
                let change = rng.gen_range(5..=15);
                self.heartbeat += change;
                println!("You stimulated the heart rate by {}.", change);
            }

            Action::SlowDown => {
                let change = rng.gen_range(-15..=-5);
                self.heartbeat += change;
                println!("You slowed down the heart rate by {}.", change.abs());
            }

            Action::Quit => {
                println!("Goodbye!");
                std::process::exit(0);
            }
        }

        if self.heartbeat < 40 || self.heartbeat > 100 {
            println!("The heartbeat went out of range.");
            std::process::exit(0);
        }
    }
}
