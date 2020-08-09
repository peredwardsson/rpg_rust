use specs::{System, WriteStorage, ReadStorage, join::Join};
use sdl2::rect::{Rect};
use crate::components::*;

pub struct Collisions;

impl<'a> System<'a> for Collisions {
    
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, CollisionBox>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, mut data: Self::SystemData) {

        for (pos, col, _, vel) in (&data.0, &data.1, &data.2, &mut data.3).join() { 
            let actor_rect = Rect::from_center(pos.0, col.width, col.height);
            for (obj, obj_col, _) in (&data.0, &data.1, !&data.2).join() {
                let object_rect = Rect::from_center(obj.0, obj_col.width, obj_col.height);
                if actor_rect.has_intersection(object_rect) {
                    println!("Collision detected!!!");
                    
                    vel.speed = 0;

                }
            }
        }
    }
}
