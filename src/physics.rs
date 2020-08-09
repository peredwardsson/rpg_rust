use specs::{WriteStorage, System, ReadStorage, join::Join};
use sdl2::rect::{Rect};
use crate::components::*;

// The stuff we want from a movement system:
// 1. Pressing any movement key m starts moving us on direction m.
// 2. Pressing anonter button afterwards should do nothing if in motion. 
// 3. 
//  

pub struct Physics;

impl<'a> System<'a> for Physics {
    
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
        ReadStorage<'a, CollisionBox>,
        WriteStorage<'a, FlagForMovement>,
        ReadStorage<'a, Playable>,
        ReadStorage<'a, Unplayable>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let mut new_pos = Position::default();
        for (pos, vel, _, flag) in (&data.0, &data.1, &data.2, &mut data.3).join() {
            if !vel.direction.is_empty() { 
                for (obj_pos, obj_col) in (&data.0, &data.2).join() {
                    let obj_rect = Rect::from_center(obj_pos.0, obj_col.width, obj_col.height); 
                    let dir = &vel.direction.front().unwrap(); 
                    new_pos.0 = match dir {
                        Direction::Right => (pos.0.offset(vel.speed, 0)),
                        Direction::Left => (pos.0.offset(-vel.speed, 0)),
                        Direction::Down => (pos.0.offset(0, vel.speed)),
                        Direction::Up => (pos.0.offset(0,-vel.speed)),
                    };
                    match obj_rect.intersect_line(pos.0, new_pos.0) {
                        Some(_) => {},
                        None => {
                            flag.moving = true;
                            flag.new_pos = new_pos;
                        },
                    };
                }
            }
        }

        for (pos, flag) in (&mut data.0, &data.3).join() {
            if flag.moving {
                pos.0 = flag.new_pos.0;
            }
        }
    }
}
