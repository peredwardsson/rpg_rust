

use std::collections::VecDeque;
use specs_derive::Component;
use specs::prelude::*;
use sdl2::rect::{Rect, Point};
use rand::distributions::{Standard, Distribution};
use rand::Rng;

#[derive(Debug, Component, Default)]
pub struct Facing(pub Direction);

#[derive(Debug, Component, Default)]
pub struct Interactable;

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