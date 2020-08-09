use specs::{WriteStorage, System, ReadStorage, WriteExpect, join::Join};
use crate::components::*;
use std::collections::VecDeque;

const PLAYER_MOVEMENT_SPEED: i32 = 5;

pub struct Keyboard;

use super::MovementCommand;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        WriteExpect<'a, VecDeque<Option<MovementCommand>>>, 
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, mut data: Self::SystemData) {

        while !&data.0.is_empty() {
            let movement_command = match (*data.0).pop_front() {
                Some(mmcmd) => mmcmd.unwrap(),
                None => return,    
            };
            for (_, vel) in (&data.1, &mut data.2).join() {
            match movement_command {
                MovementCommand::Move(direction) => {
                    vel.speed = PLAYER_MOVEMENT_SPEED;
                    vel.direction.push_back(direction);
                }
                MovementCommand::Stop(dir) => {
                    if vel.direction.contains(&dir) {
                        vel.direction.retain(|&v| v != dir);
                    } else {
                        println!("Attempted to remove {:?} from vel; failed.", dir);
                    }
                }
                _ => ()
            }
        }
        }
        
    }
}
