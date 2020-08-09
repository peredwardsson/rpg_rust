mod animator;
mod collisions;
mod components;
mod keyboard;
mod physics;
mod randomwalker;
mod renderer;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::{
    image::LoadTexture,
    keyboard::Keycode,
    rect::{Point, Rect},
};
use std::time::{Duration, Instant};
use std::collections::VecDeque;

use crate::components::*;
use specs::prelude::*;

const SPRITE_WIDTH_PLAYER: i32 = 26;
const SPRITE_HEIGHT_PLAYER: i32 = 36;

const SPRITE_WIDTH_REAPER: i32 = 32;
const SPRITE_HEIGHT_REAPER: i32 = 36;

const ANIMATION_N_FRAMES: u8 = 3;

fn generate_animation(
    spritesheet_idx: usize,
    top_left: Rect,
    dir: Direction,
    sprite_width: i32,
) -> Vec<Sprite> {
    let mut frames: Vec<Sprite> = Vec::new();

    let y_offset = top_left.y() + direction_to_animation_row(dir);
    let (width, height) = top_left.size();

    for i in 0..ANIMATION_N_FRAMES as i32 {
        frames.push(Sprite {
            spritesheet: spritesheet_idx,
            region: Rect::new(top_left.x() + sprite_width * i, y_offset, width, height),
        })
    }
    frames
}

pub enum MovementCommand {
    Stop(Direction),
    Move(Direction),
}

fn direction_to_animation_row(dir: Direction) -> i32 {
    use Direction::*;
    match dir {
        Down => 0,
        Left => SPRITE_HEIGHT_PLAYER as i32,
        Right => 2 * SPRITE_HEIGHT_PLAYER as i32,
        Up => 3 * SPRITE_HEIGHT_PLAYER as i32,
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video().expect("Could not init video system");

    let window = video_subsystem
        .window("mah game!!", 800, 600)
        .build()
        .expect("Internal error.");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not make a canvas");
    let texture_creator = canvas.texture_creator();

    let mut dispatcher = DispatcherBuilder::new()
        .with(keyboard::Keyboard, "Keyboard", &[])
        .with(physics::Physics, "Physics", &["Keyboard"])
        .with(animator::Animator, "Animator", &["Keyboard"])
        .with(randomwalker::RandomWalker, "RandomWalker", &["Physics"])
        .build();

    let mut world_clock: Option<Instant> = None;

    let mut world = World::new();
    dispatcher.setup(&mut world);
    renderer::SystemData::setup(&mut world);
    let movement_command: VecDeque<Option<MovementCommand>> = VecDeque::new();
    world.insert(movement_command);
    world.insert(world_clock);

    let textures = [
        texture_creator.load_texture("assets/bardo.png")?,
        texture_creator.load_texture("assets/reaper.png")?,
        texture_creator.load_texture("assets/food.png")?,
    ];

    let player_texture_idx = 0;
    let player_top_left = Rect::new(
        0,
        0,
        SPRITE_WIDTH_PLAYER as u32,
        SPRITE_HEIGHT_PLAYER as u32,
    );
    let player_animations = MovementAnimation {
        current_frame: 0,
        up_frames: generate_animation(
            player_texture_idx,
            player_top_left,
            Direction::Up,
            SPRITE_WIDTH_PLAYER,
        ),
        down_frames: generate_animation(
            player_texture_idx,
            player_top_left,
            Direction::Down,
            SPRITE_WIDTH_PLAYER,
        ),
        left_frames: generate_animation(
            player_texture_idx,
            player_top_left,
            Direction::Left,
            SPRITE_WIDTH_PLAYER,
        ),
        right_frames: generate_animation(
            player_texture_idx,
            player_top_left,
            Direction::Right,
            SPRITE_WIDTH_PLAYER,
        ),
    };
    let reaper_texture_idx = 1;
    let reaper_top_left = Rect::new(
        0,
        0,
        SPRITE_WIDTH_REAPER as u32,
        SPRITE_HEIGHT_REAPER as u32,
    );
    let reaper_animations = MovementAnimation {
        current_frame: 0,
        up_frames: generate_animation(
            reaper_texture_idx,
            reaper_top_left,
            Direction::Up,
            SPRITE_WIDTH_REAPER,
        ),
        right_frames: generate_animation(
            reaper_texture_idx,
            reaper_top_left,
            Direction::Right,
            SPRITE_WIDTH_REAPER,
        ),
        down_frames: generate_animation(
            reaper_texture_idx,
            reaper_top_left,
            Direction::Down,
            SPRITE_WIDTH_REAPER,
        ),
        left_frames: generate_animation(
            reaper_texture_idx,
            reaper_top_left,
            Direction::Left,
            SPRITE_WIDTH_REAPER,
        ),
    };
    let starting_velocity: VecDeque<Direction> = VecDeque::new();
    let mut starting_velocity_npc: VecDeque<Direction> = VecDeque::new();

    //starting_velocity.push_back(Direction::Down);
    starting_velocity_npc.push_back(Direction::Up);

    // Create the player
    world
        .create_entity()
        .with(KeyboardControlled)
        .with(Position(Point::new(0, 0)))
        .with(Velocity {
            speed: 0 as i32,
            direction: starting_velocity,
        })
        .with(CollisionBox {
            width: SPRITE_WIDTH_PLAYER as u32,
            height: SPRITE_HEIGHT_PLAYER as u32,
        })
        .with(Playable)
        .with(FlagForMovement{moving: false, new_pos: Position(Point::new( 0,  0))})
        .with(player_animations.right_frames[0])
        .with(player_animations)
        .build();

    // Create another thing
    world
        .create_entity()
        .with(Position(Point::new(50, 50)))
        .with(NPCWalker)
        .with(Velocity {
            speed: 0,
            direction: starting_velocity_npc,
        })
        .with(reaper_animations.right_frames[0])
        .with(Unplayable)
        .with(CollisionBox {
            width: SPRITE_WIDTH_REAPER as u32,
            height: SPRITE_HEIGHT_REAPER as u32,
        })
        .with(reaper_animations.clone())
        .build();

    
    world_clock = Some(Instant::now());

    let mut color: Color = Color::RGB(100, 255, 0);

    let mut i: i64 = 4;
    let mut going_up: i8 = 3;

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        if (i > 100) | (i < 3) {
            going_up *= -1;
        }
        i += going_up as i64;
        // println!("i = {}", i);
        color.b = 100 + i as u8;
        let mut movement_command: VecDeque<Option<MovementCommand>> = VecDeque::new();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    repeat: false,
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    movement_command.push_back(Some(MovementCommand::Move(Direction::Up)));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    movement_command.push_back(Some(MovementCommand::Move(Direction::Down)));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    movement_command.push_back(Some(MovementCommand::Move(Direction::Left)));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    movement_command.push_back(Some(MovementCommand::Move(Direction::Right)));
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    println!("KeyUp: Up");
                    movement_command.push_back(Some(MovementCommand::Stop(Direction::Up)));
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    println!("KeyUp: Down");
                    movement_command.push_back(Some(MovementCommand::Stop(Direction::Down)));
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    println!("KeyUp: Left");
                    movement_command.push_back(Some(MovementCommand::Stop(Direction::Left)));
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    movement_command.push_back(Some(MovementCommand::Stop(Direction::Right)));
                    println!("KeyUp: Right");
                }
                
                _ => {}
            }
        }

        *world.write_resource() = movement_command;
        *world.write_resource() = world_clock;

        // Update
        dispatcher.dispatch(&world);
        world.maintain();

        // Render
        renderer::render(&mut canvas, color, &textures, world.system_data())?;

        // Time Management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}