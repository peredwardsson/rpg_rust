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
        ReadExpect<'a, Option<PlayerCommands>>,
        ReadStorage<'a, KeyboardControlled>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Facing>,
        ReadStorage<'a, CollisionBox>,
        ReadStorage<'a, Interactable>,
        ReadStorage<'a, InteractionZone>,
    );

    fn run(&mut self, 
    (
        mut movementcommands,
        playercommands,
        is_keyboardcontrolled,
        position,
        mut velocity,
        mut facing,
        collisionbox,
        interactable,
        interactionzone,
    ): Self::SystemData) {
        
        // This clause takes care of movement
        while !&movementcommands.is_empty() {
            let movement_command = match (movementcommands).pop_front() {
                Some(Some(mmcmd)) => mmcmd,
                _ => return,    
            }; 
            for (_, vel, facing) in (&is_keyboardcontrolled, &mut velocity, &mut facing).join() {
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
        match &*playercommands {
            Some(PlayerCommands::Interact) => {
                println!("Interacting!!");
                let mut interaction_zone: InteractionZone;
                for (pos, interzone) in (&position, &interactionzone).join() {
                    interaction_zone = (*interzone).clone();
                    for (obj_pos, _) in (&position, &interactable).join() {
                        if interaction_zone.rect.contains_point(obj_pos.0) {
                            println!("Found one!!");
                        }
                    }
                }

            },
            Some(PlayerCommands::Menu) => {

            },
            None => {}
        };

        // if let m = *(playercommands.iter()).collect() {
        //     match m {
        //         PlayerCommands::Interact => {
        //             println!("Interacting!!");

        //             let mut interaction_zone;
        //             for (_, player_pos, facing) in (&is_keyboardcontrolled, &position, &facing).join() {
        //                 if facing.0 == Direction::Left || facing.0 == Direction::Right {
        //                     interaction_zone = Rect::new(
        //                         player_pos.0.x, 
        //                         player_pos.0.y, 
        //                         INTERACTION_HEIGHT, 
        //                         INTERACTION_WIDTH,
        //                     );
        //                 } else { 
        //                     interaction_zone = Rect::new(
        //                         player_pos.0.x, 
        //                         player_pos.0.y, 
        //                         INTERACTION_WIDTH, 
        //                         INTERACTION_HEIGHT,
        //                     );
        //                 }

        //                 for (obj_pos, obj_col, _) in (&position, &collisionbox, &interactable).join() {
        //                     let x = obj_pos.0.x + obj_col.width as i32;
        //                     let y = obj_pos.0.y + obj_col.height as i32;
        //                     if interaction_zone.contains_point(Point::new(x, y)) {
        //                         println!("Thing in interaction zone!!")
        //                     }
        //                 }
                        
        //             }
                    
        //         },
        //         PlayerCommands::Menu => {
        //             println!("Open the darn menu!!")
        //         },
        //     }
        // }
        
    }
}
