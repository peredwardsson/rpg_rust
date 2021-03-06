use specs::{WriteStorage, System, ReadStorage, WriteExpect, ReadExpect, join::Join, Entities};
use crate::components::*;
use std::{fs::File, collections::VecDeque, io::Read};
//use sdl2::rect::{Rect, Point};

const PLAYER_MOVEMENT_SPEED: i32 = 5;

pub struct Keyboard;

use super::MovementCommand;
use super::PlayerCommands;
use super::Gamestate;
use regex::Regex;

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
        ReadStorage<'a, Dialogue>,
        WriteExpect<'a, VecDeque<Dialogue_Single_item>>
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
        mut gamestate,
        dialogue,
        mut dialogue_list,

    ): Self::SystemData) {
        
        let mut change_to_dialogue = false;
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
                            for (obj_pos, object, dialogue) in (&position, &mut interactable, (&dialogue).maybe()).join() {
                                if interzone.rect.contains_point(obj_pos.0) & ( // Is there an object in the interaction zone?
                                    ( // Can we interact more with it?
                                        (object.interactions < object.max_interactions) &
                                        (object.max_interactions > 0)
                                    ) | ( // Can we interact an infinite amount with it?
                                        object.max_interactions == 0
                                    )
                                ) {
                                    match (*object).interaction_type {
                                        InteractableType::Character => {
                                            if let Some(d) = dialogue {

                                                //gamestate = Gamestate::Dialogue;

                                                change_to_dialogue = true;
                                                
                                                let conversation_pattern = Regex::new(r#": "(.+)", (.+)\n(.+)"#).unwrap();

                                                println!("Reading file {}", &d.dialogue_file);
                                                let mut conv = String::new();
                                                {
                                                    let mut file = File::open(&d.dialogue_file).unwrap();
                                                    file.read_to_string(&mut conv).unwrap();
                                                }
                                                
                                                for cap in conversation_pattern.captures_iter(&conv) {
                                                    dialogue_list.push_back(Dialogue_Single_item {
                                                        speaker_name: (&cap[1]).into(),
                                                        background_size: Size3::Small,
                                                        dialogue_text: (&cap[3]).into(),
                                                    });
                                                }
                                            };
                                        },
                                        _ => (*object).interact()
                                    }
                                    //println!("Interacted with a {:?}", *object);
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
            Gamestate::Dialogue => {
                if let Some(PlayerCommands::Interact) = &*playercommands {
                    (*dialogue_list).pop_front();
                    if (*dialogue_list).is_empty() {
                        *gamestate = Gamestate::Running;
                    } 
                }
            }
            _ => {println!("Not running, player commands disabled.");}
        }
        if change_to_dialogue {
            *gamestate = Gamestate::Dialogue;
        }
        
    }
}
