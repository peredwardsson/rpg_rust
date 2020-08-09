use specs::{WriteStorage, System, ReadStorage, join::Join};
use crate::components::*;

pub struct Animator;

impl<'a> System<'a> for Animator {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (
        WriteStorage<'a, MovementAnimation>,
        WriteStorage<'a, Sprite>, 
        ReadStorage<'a, Velocity>
    );

    fn run(&mut self, mut data: Self::SystemData) {
        use crate::components::Direction::*;
        // This joins the component storages for Position
        // and Velocity together; it's also possible to do this
        // in parallel using rayon's `ParallelIterator`s.
        // See `ParJoin` for more.
        
        for (anim, sprite, vel) in (&mut data.0, &mut data.1, &data.2).join() {    
            if vel.direction.is_empty() {
                continue;
            }
            
            let frames = match vel.direction[0] {
                Left => &anim.left_frames,
                Right => &anim.right_frames,
                Up => &anim.up_frames,
                Down => &anim.down_frames,
            };

            anim.current_frame = (anim.current_frame + 1) % frames.len();
            *sprite = frames[anim.current_frame];
        }
    }
}
