use specs::{WriteStorage, System, ReadStorage, WriteExpect, ReadExpect, join::Join, Entities};
use crate::components::*;
use std::collections::VecDeque;
//use sdl2::rect::{Rect, Point};

const PLAYER_MOVEMENT_SPEED: i32 = 5;

pub struct Keyboard;

use super::MovementCommand;
use super::PlayerCommands;
use super::Gamestate;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        WriteExpect<'a, VecDeque<Option<MovementCommand>>>, 
        ReadExpect<'a, Option<PlayerCommands>>,
        ReadStorage<'a, KeyboardControlled>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Facing>,
        ReadStorage<'a, CollisionBox>,
        WriteStorage<'a, Interactable>,
        ReadStorage<'a, InteractionZone>,
        Entities<'a>,
        WriteExpect<'a, Gamestate>,
    );

    fn run(&mut self, 
    (
        mut movementcommands,
        playercommands,
        is_keyboardcontrolled,
        position,
        mut velocity,
        mut facing,
        _collisionbox,
        mut interactable,
        interactionzone,
        _entities,
        mut gamestate
    ): Self::SystemData) {
        match *gamestate {
            Gamestate::Running => {
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
                            }
                            MovementCommand::Stop(dir) => {
                                if vel.direction.contains(&dir) {
                                    vel.direction.retain(|&v| v != dir);
                                } 
                            }
                        }
                        if !vel.direction.is_empty(){
                            facing.direction = *vel.direction.front().unwrap();
                        }
                    }
                }
                
                // This clause takes care of dealing with input commands.
                match &*playercommands {
                    Some(PlayerCommands::Interact) => {
                        // Todo: Make this work better when there are more than one interactable object in the zone.
                        for interzone in (&interactionzone).join() {
                            for (obj_pos, object) in (&position, &mut interactable).join() {
                                if interzone.rect.contains_point(obj_pos.0) & ( // Is there an object in the interaction zone?
                                    ( // Can we interact more with it?
                                        (object.interactions < object.max_interactions) &
                                        (object.max_interactions > 0)
                                    ) | ( // Can we interact an infinite amount with it?
                                        object.max_interactions == 0
                                    )
                                ) {
                                    (*object).interact();
                                    // Run the interaction!
                                    //println!("Interacted with a thing!");
                                    continue;
                                }
                            }
                        }

                    },
                    Some(PlayerCommands::Menu) => {
                        *gamestate = Gamestate::Menu;
                    },
                    None => {}
                };

            },
            _ => {gamestate;}
        }
    }
}
