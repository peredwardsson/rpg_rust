use crate::components::*;

pub fn one_time_use(obj: &Interactable) {
    println!("Ryan used this like an object:");
    println!("{:?}", obj);
}

pub fn chest(obj: &Interactable) {
    let item = "potion";
    println!("You found a {}!", item);
}