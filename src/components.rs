mod interactable_objects;
use interactable_objects::*;

use std::{fmt::Debug, collections::VecDeque, fs::File, io::{self, Read}};
use specs_derive::Component;
use specs::prelude::*;
use sdl2::rect::{Rect, Point};
use rand::distributions::{Standard, Distribution};
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub enum Size3 {
    Small,
    Medium,
    Large,
}

impl Default for Size3 {
    fn default() -> Self {
        Size3::Small
    }
}


#[derive(Debug, Component, Clone)]
#[storage(VecStorage)]
pub struct Dialogue {
    pub sprite: Sprite,
    pub dialogue_file: String,
    pub show: bool,
}

#[derive(Debug)]
pub struct Dialogue_Single_item {
    pub speaker_name: String,
    pub dialogue_text: String,
    pub background_size: Size3,
}

#[derive(Debug, Component, Clone)]
pub struct InteractionZone {
    pub rect: Rect,
    pub flipped: bool,
}

impl Default for InteractionZone {
    fn default() -> Self {
        InteractionZone { 
            rect: Rect::new(0, 0, 50, 80),
            flipped : false,
        }
    }
}

impl InteractionZone {
    pub fn flip (&mut self) {
        let w = self.rect.width();
        let h = self.rect.height();
        self.rect.set_height(w);
        self.rect.set_width(h);
        self.flipped = !self.flipped;
    }
}


#[derive(Debug, Component, Default)]
pub struct Facing {
    pub direction: Direction,
}

#[allow(dead_code)]
pub enum InteractableType {
    Chest,
    Pickup,
    DestroyedOnUse,
    Character,
    Lever,
}

impl Debug for InteractableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            InteractableType::Chest => {"Chest"},
            InteractableType::Pickup => {"Pickup"},
            InteractableType::DestroyedOnUse => {"DestroyedOnUse"},
            InteractableType::Character => {"Character"},
            InteractableType::Lever => {"Lever"},
        };
        write!(f, "{}",s.to_string())
    }

}


#[derive(Debug, Component)]
pub struct Interactable{
    pub interactions: i64,
    pub max_interactions: i64,
    pub interaction_type: InteractableType,
}

impl Default for Interactable {
    fn default() -> Self {
        Interactable {
            interactions: 0,
            max_interactions: 1,
            interaction_type: InteractableType::DestroyedOnUse,
        }
    }
}


impl Interactable {
    pub fn interact(&mut self) {
        interacted_with_object(&self);
        match (*self).interaction_type {
            InteractableType::Chest => (chest(&self)),
            InteractableType::Pickup => (pickup(&self)),
            InteractableType::DestroyedOnUse => (destroyed_on_use(&self)),
            InteractableType::Character => (
                //character(&self)
                Ok(())
            ),
            InteractableType::Lever => (lever(&self)),
        };
        if ((*self).max_interactions > (*self).interactions) |
        ((*self).max_interactions == 0) {
            (*self).interactions += 1;
        }
    }

    
}

#[derive(Debug, Component, Default)]
pub struct Collectible;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0, 3) {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Up,
        }
    }  
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Down
    }
}

#[derive(Component, Clone, Default, Debug)]
#[storage(VecStorage)]
pub struct FlagForMovement {
    pub moving: bool,
    pub new_pos: Position,
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Playable;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Unplayable;

#[derive(Component, Clone, Debug, Default)]
#[storage(VecStorage)]
pub struct CollisionBox {
    pub width: u32,
    pub height: u32,
}

impl CollisionBox {
    #[allow(dead_code)]
    pub fn flip (&mut self) {
        let w = self.width;
        let h = self.height;
        self.height = w;
        self.width = h;
    }
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct KeyboardControlled;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct NPCWalker;

#[derive(Component, Clone, Copy, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Point);

impl Default for Position {
    fn default() -> Self {
        Position(Point::new(0,0))
    }
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Velocity {
    pub speed: i32,
    pub direction: VecDeque<Direction>,
}

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct Sprite {
    pub spritesheet: usize,
    pub region: Rect,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct MovementAnimation {
    pub current_frame: usize,
    pub up_frames: Vec<Sprite>,
    pub right_frames: Vec<Sprite>,
    pub down_frames: Vec<Sprite>,
    pub left_frames: Vec<Sprite>,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct EntityAnimation {
    pub current_frame: usize,
    pub frames: Vec<Sprite>,
}

impl Default for EntityAnimation {
    fn default() -> Self {
        EntityAnimation{
            current_frame: 0,
            frames: Vec::new(),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Gamestate {
    Running,
    Pause,
    Menu,
    Dialogue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dialogue_Helper {
    pub text: String,
    pub width: u32,
    pub height: u32,
}

pub struct Ability {
    pub name: String,
    pub range: i32, // Err... maybe? In pixels rn?
    pub animation: EntityAnimation,
    pub effects: i32 // Probably 
}