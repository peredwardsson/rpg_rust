use specs::{ReadStorage, join::Join, WriteStorage, System};
use crate::components::*;

pub struct IZUpdater;

impl<'a> System<'a> for IZUpdater {
    
    type SystemData = (
        ReadStorage<'a, Position>,
        WriteStorage<'a, InteractionZone>,
        ReadStorage<'a, Facing>,
        ReadStorage<'a, CollisionBox>
    );

    fn run(&mut self, (position, mut interactionzone, facing, collisionbox): Self::SystemData) {

        for (pos, intzone, theface, col) in (&position, &mut interactionzone, &facing, &collisionbox).join() {

            let mut best_point;
            
            match theface.direction {
                Direction::Up | Direction::Down => {
                    if intzone.flipped {
                        intzone.flip();
                    }
                    best_point = (col.height as i32 + intzone.rect.height() as i32)/2;
                    if theface.direction == Direction::Up {
                        best_point *= -1;
                    }
                    intzone.rect.center_on(pos.0.offset(0, best_point));
                }
                Direction::Left | Direction::Right => {
                    if !intzone.flipped {
                        intzone.flip();
                    }
                    best_point = (col.width as i32 + intzone.rect.width() as i32)/2;
                    if theface.direction == Direction::Left {
                        best_point *= -1;
                    }
                    intzone.rect.center_on(pos.0.offset(best_point, 0));
                }
            }
            
        }
    }
}