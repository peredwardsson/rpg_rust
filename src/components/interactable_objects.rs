use crate::components::*;
use super::super::Gamestate;

// components::sdl-test2::Gamestate
pub fn interacted_with_object(obj: &Interactable) {
    println!("Ryan used this like an object:");
    println!("{:?}", obj);
}

pub fn chest(_obj: &Interactable) {
    let item = "potion";
    println!("You found a {}!", item);
}

pub fn pickup(_obj: &Interactable) {
    println!("Pickup not implemented yet!");
}

pub fn destroyed_on_use(_obj: &Interactable) {
    println!("Destroyed on use not implemented yet!");
}

pub fn character (_obj: &Interactable, mut thegame: Gamestate) {
    
    thegame = Gamestate::Dialogue;
    
    // Do dialogue stuff

    thegame = Gamestate::Running;
}

pub fn lever (_obj: &Interactable) {
    println!("Lever not implemented yet!");
}