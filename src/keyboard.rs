use specs::{WriteStorage, System, ReadStorage, WriteExpect, ReadExpect, join::Join};
use crate::components::*;
use std::collections::VecDeque;
use sdl2::rect::{Rect, Point};

const PLAYER_MOVEMENT_SPEED: i32 = 5;

const INTERACTION_WIDTH: u32 = 50;
const INTERACTION_HEIGHT: u32 = 80;

pub struct Keyboard;

use super::MovementCommand;
use super::PlayerCommands;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        WriteExpect<'a, VecDeque<Option<MovementCommand>>>, 
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Velocity>,
        ReadExpect<'a, Option<PlayerCommands>>,
        ReadStorage<'a, Interactable>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Facing>,
        ReadStorage<'a, CollisionBox>,
        ReadStorage<'a, InteractionZone>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        
        // This clause takes care of movement
        while !&data.0.is_empty() {
            let movement_command = match (*data.0).pop_front() {
                Some(Some(mmcmd)) => mmcmd,
                _ => return,    
            };
            for (_, vel, facing) in (&data.1, &mut data.2, &mut data.6).join() {
                match movement_command {
                    MovementCommand::Move(direction) => {
                        vel.speed = PLAYER_MOVEMENT_SPEED;
                        vel.direction.push_back(direction);
                        facing.0 = direction;
                    }
                    MovementCommand::Stop(dir) => {
                        if vel.direction.contains(&dir) {
                            vel.direction.retain(|&v| v != dir);
                        } 
                    }
                }
            }
        }
        
        // This clause takes care of dealing with input commands.
        if let Some(m) = &*data.3 {
            match m {
                PlayerCommands::Interact => {
                    println!("Interacting!!");

                    let mut interaction_zone;
                    for (_, player_pos, facing) in (&data.1, &data.5, &data.6).join() {
                        if facing.0 == Direction::Left || facing.0 == Direction::Right {
                            interaction_zone = Rect::new(
                                player_pos.0.x, 
                                player_pos.0.y, 
                                INTERACTION_HEIGHT, 
                                INTERACTION_WIDTH,
                            );
                        } else { 
                            interaction_zone = Rect::new(
                                player_pos.0.x, 
                                player_pos.0.y, 
                                INTERACTION_WIDTH, 
                                INTERACTION_HEIGHT,
                            );
                        }

                        for (obj_pos, obj_col, _) in (&data.5, &data.7, &data.4).join() {
                            let x = obj_pos.0.x + obj_col.width as i32;
                            let y = obj_pos.0.y + obj_col.height as i32;
                            if interaction_zone.contains_point(Point::new(x, y)) {
                                println!("Thing in interaction zone!!")
                            }
                        }
                        
                    }
                    
                },
                PlayerCommands::Menu => println!("Open the darn menu!!"),
            }
        }
        
    }
}
