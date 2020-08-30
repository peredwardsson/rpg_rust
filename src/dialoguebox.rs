use specs::{WriteStorage, System, ReadStorage, join::Join};
use sdl2::rect::{Rect};
use crate::components::*;

pub struct DialogueBox;

impl<'a> System<'a> for DialogueBox {
    
    type SystemData = (
    ReadStorage<'a, Dialogue>,
    ReadExpect<'a, Option<PlayerCommands>>,
    );

    fn run(&mut self,
        (
            dialogue,
            playercmd,
        ): Self::SystemData) {
            // If there is an item in the dialogue object, set the current dialogue to this item. 
            // Y'know, maybe this whole thing is unnecessary.
    }
}
