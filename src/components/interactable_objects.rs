use crate::components::*;
use super::super::Gamestate;
use std::fs::File;
use std::io::prelude::*;

// components::sdl-test2::Gamestate
pub fn interacted_with_object(obj: &Interactable) {
    println!("Ryan used this like an object:");
    println!("{:?}", obj);
}

pub fn chest(_obj: &Interactable)-> std::io::Result<()> {
    let item = "potion";
    println!("You found a {}!", item);
    Ok(())
}

pub fn pickup(_obj: &Interactable)-> std::io::Result<()> {
    println!("Pickup not implemented yet!");
    Ok(())
}

pub fn destroyed_on_use(_obj: &Interactable) -> std::io::Result<()> {
    println!("Destroyed on use not implemented yet!");
    Ok(())
}

pub fn character (_obj: &Interactable, thegame: &mut Gamestate, dialogue: &mut Dialogue) -> std::io::Result<()> {

    Ok(())
}

pub fn lever (_obj: &Interactable) -> std::io::Result<()> {
    println!("Lever not implemented yet!");
    Ok(())
}