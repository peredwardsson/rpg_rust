mod animator;
mod collisions;
mod components;
mod keyboard;
mod physics;
mod randomwalker;
mod renderer;
mod collectibles;
mod update_interaction;

use rand::{Rng, thread_rng};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::{
    image::{LoadTexture},
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

const SPRITE_HEIGHT_FRUIT: i32 = 16;
const SPRITE_WIDTH_FRUIT: i32 = 16;

const SPRITE_HEIGHT_CHEST: i32 = 24;
const SPRITE_WIDTH_CHEST: i32 = 24;

const ANIMATION_N_FRAMES: u8 = 3;
const ANIMATION_N_FRAMES_CHEST: u8 = 3;


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

fn generate_animation_chest(
    spritesheet_idx: usize
) -> Vec<Sprite> {
    let mut frames: Vec<Sprite> = Vec::new();

    for i in 0..ANIMATION_N_FRAMES_CHEST as i32 {
        frames.push(Sprite {
            spritesheet: spritesheet_idx,
            region: Rect::new(
                0,
                SPRITE_HEIGHT_CHEST * i, 
                SPRITE_WIDTH_CHEST as u32, 
                SPRITE_HEIGHT_CHEST as u32),
        })
    }

    frames
}

pub enum MovementCommand {
    Stop(Direction),
    Move(Direction),
}

pub enum PlayerCommands {
    Interact,
    Menu,
}

pub fn add_player(world: &mut World) -> Result<(), String> {
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

    world
        .create_entity()
        .with(KeyboardControlled)
        .with(Position(Point::new(0, 0)))
        .with(Velocity {
            speed: 0 as i32,
            direction: VecDeque::new(),
        })
        .with(CollisionBox {
            width: SPRITE_WIDTH_PLAYER as u32,
            height: SPRITE_HEIGHT_PLAYER as u32,
        })
        .with(Playable)
        .with(FlagForMovement{moving: false, new_pos: Position(Point::new( 0,  0))})
        .with(player_animations.down_frames[0])
        .with(player_animations)
        .with(Facing::default())
        .with(InteractionZone::default())
        .build();

    Ok(())
}

pub fn spawn_fruit(world: &mut World, x: i32, y: i32) -> Result<(), String> {
    let spritesheet = 2;

    let mut r = thread_rng();
    let row = r.gen_range(0, 7);
    let col = r.gen_range(0, 7);

    let region = Rect::new(
        row * SPRITE_HEIGHT_FRUIT,
        col * SPRITE_WIDTH_FRUIT,
        SPRITE_WIDTH_FRUIT as u32,
        SPRITE_HEIGHT_FRUIT as u32,
    );

    let fruit_sprite = Sprite{spritesheet, region};

    world
        .create_entity()
        .with(Position(Point::new(x, y)))
        .with(CollisionBox {
            width: SPRITE_WIDTH_FRUIT as u32,
            height: SPRITE_HEIGHT_FRUIT as u32,
        })
        .with(Unplayable)
        .with(Collectible)
        .with(fruit_sprite)
        .build();

    Ok(())
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

pub fn spawn_chest(world: &mut World, x: i32, y: i32) -> Result<(), String> {
    let spritesheet = 3;

    let chest_frames = generate_animation_chest(spritesheet);
    let chest_animation = EntityAnimation{
        current_frame: 0,
        frames: chest_frames,
    };
    let i = Interactable {
        interactions: 0,
        max_interactions: 0,
        interaction_type: InteractableType::Chest
    };
    world
        .create_entity()
        .with(Position(Point::new(x, y)))
        .with(CollisionBox {
            width: SPRITE_WIDTH_CHEST as u32,
            height: SPRITE_HEIGHT_CHEST as u32,
        })
        .with(Unplayable)
        .with(i)
        .with(chest_animation.clone())
        .with(chest_animation.frames[0])
        .build();
    Ok(())
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video().expect("Could not init video system");

    let window = video_subsystem
        .window("mah game!!", 800, 600)
        .position(0, 0)
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
        .with(collectibles::Collectibles, "Collectibles",&["Physics"])
        .with(update_interaction::IZUpdater, "Interaction Zone", &["Physics"])
        .build();

    let mut world_clock: Option<Instant> = None;

    let mut world = World::new();
    dispatcher.setup(&mut world);
    renderer::SystemData::setup(&mut world);
    
    let movement_command: VecDeque<Option<MovementCommand>> = VecDeque::new();
    let player_command: Option<PlayerCommands> = None;

    let mut draw_bounding_box = true;
    let mut draw_interaction_zone = true;

    world.insert(movement_command);
    world.insert(world_clock);
    world.insert(player_command);
    world.register::<EntityAnimation>();

    let textures = [
        texture_creator.load_texture("assets/bardo.png")?,
        texture_creator.load_texture("assets/reaper.png")?,
        texture_creator.load_texture("assets/food.png")?,
        texture_creator.load_texture("assets/chest.png")?,
    ];
    
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
    let mut starting_velocity_npc: VecDeque<Direction> = VecDeque::new();

    starting_velocity_npc.push_back(Direction::Up);

    add_player(&mut world)?;

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
        let mut player_command: Option<PlayerCommands> = None;

        for event in event_pump.poll_iter() {
            // TODO: Add support for setting draw_bounding and draw_interaction.
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
                    //println!("KeyUp: Up");
                    movement_command.push_back(Some(MovementCommand::Stop(Direction::Up)));
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    //println!("KeyUp: Down");
                    movement_command.push_back(Some(MovementCommand::Stop(Direction::Down)));
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    //println!("KeyUp: Left");
                    movement_command.push_back(Some(MovementCommand::Stop(Direction::Left)));
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    movement_command.push_back(Some(MovementCommand::Stop(Direction::Right)));
                    //println!("KeyUp: Right");
                }

                Event::KeyDown {
                    keycode: Some(Keycode::F1),
                    ..
                } => {
                    draw_bounding_box = !draw_bounding_box;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::F2),
                    ..
                } => {
                    draw_interaction_zone = !draw_interaction_zone;
                }

                Event::MouseButtonDown{x, y, ..} => {
                    println!("Mouse down! ({}, {})", x, y);
                    let (w, h) = canvas.output_size()?;
                    let w = w as i32;
                    let h = h as i32;
                    spawn_chest(&mut world, x-w/2, y-h/2)?; 
                },

                Event::KeyDown{
                    keycode: Some(Keycode::Z),
                    ..
                } => {
                    player_command = Some(PlayerCommands::Interact);
                }
                Event::KeyUp{
                    keycode: Some(Keycode::Z),
                    ..
                } => {
                    //println!("{:?}", world.entities().);
                    player_command = None;
                }

                _ => {}
            }
        }

        *world.write_resource() = movement_command;
        *world.write_resource() = world_clock;
        *world.write_resource() = player_command;

        // Update
        dispatcher.dispatch(&world);
        world.maintain();

        // Render
        renderer::render(&mut canvas, 
            color, 
            &textures, 
            world.system_data(), 
            draw_bounding_box, 
            draw_interaction_zone)?;

        // Time Management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
