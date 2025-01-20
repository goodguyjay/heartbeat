use rand::Rng;

#[derive(PartialEq, Copy, Clone)]
pub enum Action {
    Stimulate,
    SlowDown,
    StayStill,
    Quit,
}

pub struct Game {
    pub heartbeat: i32,
    pub last_action: Option<Action>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            heartbeat: 72,
            last_action: None,
        }
    }

    pub fn apply_action(&mut self, action: &Action) {
        let mut rng = rand::thread_rng();

        match action {
            Action::Stimulate => {
                if self.last_action == Some(Action::Stimulate) {
                    println!("You feel the effects diminishing...");
                }

                let change = rng.gen_range(5..=15);
                self.heartbeat += change;
                println!("You stimulated the heart rate by {}.", change);
            }

            Action::SlowDown => {
                if self.last_action == Some(Action::SlowDown) {
                    println!("Anna hesitates, slowing down further...");
                }

                let change = rng.gen_range(-15..=-5);
                self.heartbeat += change;
                println!("You slowed down the heart rate by {}.", change.abs());
            }

            Action::StayStill => {
                println!("Time passes, but at what cost?");
            }

            Action::Quit => {}
        }

        self.last_action = Some(*action);

        if self.heartbeat < 40 || self.heartbeat > 100 {
            println!("The heartbeat went out of range.");
            std::process::exit(0);
        }
    }
}
