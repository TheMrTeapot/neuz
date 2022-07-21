use std::{time::Duration, thread, ops::Range};

use rand::Rng;

use crate::platform::{send_keystroke, Key, KeyMode, PlatformAccessor, send_message};

#[derive(Debug, Clone, Copy)]
pub enum MovementDirection {
    Forward,
    Backward,
    Random,
}

#[derive(Debug, Clone, Copy)]
pub enum RotationDirection {
    Left,
    Right,
    Random,
}

#[derive(Debug, Clone)]
pub enum ActionDuration {
    Fixed(u64),
    Random(Range<u64>),
}

impl ActionDuration {
    fn to_duration(&self, rng: &mut rand::rngs::ThreadRng) -> Duration {
        match self {
            Self::Fixed(ms) => Duration::from_millis(*ms),
            Self::Random(range) => Duration::from_millis(rng.gen_range(range.clone())),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Movement {
    Jump,
    Move(MovementDirection, ActionDuration),
    Rotate(RotationDirection, ActionDuration),
    Type(String),
    Wait(ActionDuration),
}

pub struct MovementCoordinator<'a> {
    rng: rand::rngs::ThreadRng,
    platform: &'a PlatformAccessor<'a>,
}

impl<'a> MovementCoordinator<'a> {
    pub fn new(platform: &'a PlatformAccessor<'_>) -> Self {
        let rng = rand::thread_rng();

        Self { rng, platform }
    }

    // Helper functions

    pub fn get_random_direction(&mut self) -> RotationDirection {
        if self.rng.gen() {
            RotationDirection::Left
        } else {
            RotationDirection::Right
        }
    }

    pub fn get_random_duration_between_ms(&mut self, min: u64, max: u64) -> Duration {
        Duration::from_millis(self.rng.gen_range(min..max))
    }

    // Wrapper functions

    pub fn with_probability<F>(&mut self, probability: f64, func: F)
    where
        F: Fn(&Self),
    {
        if self.rng.gen_bool(probability) {
            func(self);
        }
    }

    // Movement functions

    pub fn play<M>(&mut self, movements: M) where M: AsRef<[Movement]> {
        for movement in movements.as_ref() {
            self.play_single(movement.clone());
        }
    }

    fn play_single(&mut self, movement: Movement) {
        match movement {
            Movement::Jump => {
                send_keystroke(Key::Space, KeyMode::Press);
            },
            Movement::Move(direction, duration) => {
                let key = match direction {
                    MovementDirection::Forward => Key::W,
                    MovementDirection::Backward => Key::S,
                    MovementDirection::Random => {
                        if self.rng.gen() {
                            Key::W
                        } else {
                            Key::S
                        }
                    }
                };
                send_keystroke(key, KeyMode::Hold);
                thread::sleep(duration.to_duration(&mut self.rng));
                send_keystroke(key, KeyMode::Release);
            },
            Movement::Rotate(direction, duration) => {
                let key = match direction {
                    RotationDirection::Left => Key::A,
                    RotationDirection::Right => Key::D,
                    RotationDirection::Random => {
                        if self.rng.gen() {
                            Key::A
                        } else {
                            Key::D
                        }
                    }
                };
                send_keystroke(key, KeyMode::Hold);
                thread::sleep(duration.to_duration(&mut self.rng));
                send_keystroke(key, KeyMode::Release);
            },
            Movement::Wait(duration) => thread::sleep(duration.to_duration(&mut self.rng)),
            Movement::Type(text) => {
                send_message(&text);
            }
        }
    }

    pub fn jump(&self) {
        send_keystroke(Key::Space, KeyMode::Press);
    }
}
