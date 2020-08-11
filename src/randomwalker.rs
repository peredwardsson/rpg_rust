use specs::{WriteStorage, System, ReadStorage, ReadExpect, join::Join};
use crate::components::*;
use std::time::Instant;
use rand::{thread_rng, Rng};

pub struct RandomWalker;

impl<'a> System<'a> for RandomWalker {
    
    type SystemData = (
        ReadExpect<'a, Option<Instant>>,
        ReadStorage<'a, NPCWalker>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (timer, is_walker, mut velocity): Self::SystemData) {

        let time = match &*timer {
            Some(timer) => timer,
            None => { println!("No timer found.");
                return},
        };
        let mut r = thread_rng();

        for (_, vel) in (&is_walker, &mut velocity).join() {
            if time.elapsed().as_millis() % 50 == 0 {
                let sample: f64 = r.gen();
                if sample < 0.9 {
                    let dir: Direction = r.gen();
                    vel.direction.push_back(dir);
                    //println!("Changing direction! New dir: {:?}", dir);
                } 
                
            }
            
        }
    }
}