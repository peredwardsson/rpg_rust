use specs::{WriteStorage, System, ReadStorage, join::Join};
use crate::components::*;

pub struct Animator;

impl<'a> System<'a> for Animator {
    type SystemData = (
        WriteStorage<'a, MovementAnimation>,
        WriteStorage<'a, Sprite>, 
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, EntityAnimation>,
        ReadStorage<'a, Interactable>,
    );

    fn run(&mut self, (
        mut moveanimation,
        mut sprite,
        velocity,
        mut entanimation,
        interactable
    ): Self::SystemData) {
        use crate::components::Direction::*;
        
        for (anim, sprite, vel) in (&mut moveanimation, &mut sprite, &velocity).join() {    
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

        for (anim, sprite, obj) in (&mut entanimation, &mut sprite, &interactable).join() {
            if obj.interactions > 0 {
            let frames = &anim.frames;
            *sprite = frames[anim.current_frame];
            if anim.current_frame < frames.len()-1 {
                anim.current_frame += 1;
            } else {
                continue;
            }
            }
            
        }
    }
}
