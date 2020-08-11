use specs::{ReadStorage, join::Join, WriteStorage, System};
use crate::components::*;
use sdl2::rect::Point;

pub struct IZUpdater;

impl<'a> System<'a> for IZUpdater {
    
    type SystemData = (
        ReadStorage<'a, Position>,
        WriteStorage<'a, InteractionZone>,
        ReadStorage<'a, Facing>,
        ReadStorage<'a, CollisionBox>
    );

    fn run(&mut self, (position, mut interactionzone, facing, collisionbox): Self::SystemData) {

        for (pos, intzone, dir, col) in (&position, &mut interactionzone, &facing, &collisionbox).join() {

            let mut best_point;
            intzone.rect.center_on(pos.0);
            match dir.0 {
                Direction::Up => {
                    
                    //println!("Updating!! pos = {:?}", pos);
                    best_point = -(col.height as i32 + intzone.rect.height() as i32);
                    best_point /= 2;
                    best_point += pos.0.y;
                    let p = Point::new(pos.0.x, best_point);
                    intzone.rect.center_on(p);
                },
                Direction::Down => {
                    best_point = col.height as i32 + intzone.rect.height() as i32;
                    best_point /= 2;
                    best_point += pos.0.y;
                    let p = Point::new(pos.0.x, best_point);
                    intzone.rect.center_on(p);
                },
                Direction::Left => {
                    // TODO: implement a rotated interaction zone for this direction as well as Right
                }
                Direction::Right => {

                }
            }
            
        }
    }
}