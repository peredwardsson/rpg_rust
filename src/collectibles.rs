use specs::{System, ReadStorage, join::Join, Entities};
use sdl2::rect::{Rect};
use crate::components::*;

pub struct Collectibles;

impl<'a> System<'a> for Collectibles {
    
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, CollisionBox>,
        ReadStorage<'a, Collectible>,
        ReadStorage<'a, Playable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        
        for (pos, col, _) in (&data.1, &data.2, &data.4).join() {
            let player_rect = Rect::from_center(pos.0, col.width, col.height); 

            (&data.0, &data.1, &data.2, &data.3).join()
            .for_each(
                |(entity, obj_pos, obj_col, _)| {
                    let obj_rect = Rect::from_center(obj_pos.0, obj_col.width, obj_col.height);
                    if obj_rect.intersection(player_rect).is_some(){
                        data.0.delete(entity).ok();
                        println!("Collected fruit!");
                    }
                }
            );
        }
    }
}

