// Project has come along quite well atm. A few paths for upcoming goals are
// - Implement an editor to easily add collision maps for background images. This requires some GUI and some serialization.
// ->> Add dialogue support. Read from file, modify game state, that kinda thing.
// - Add animations for attacks. This path will go by animation in PS (or something). Quite the detour, but very interesting.


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
    pixels::PixelFormatEnum,
};
use std::time::{Duration, Instant};
use std::collections::{VecDeque, HashMap};
use noise::{Perlin, NoiseFn};

use crate::components::*;
use specs::prelude::*;

extern crate sdl2;

const SPRITE_WIDTH_PLAYER: i32 = 26;
const SPRITE_HEIGHT_PLAYER: i32 = 36;

const SPRITE_WIDTH_REAPER: i32 = 32;
const SPRITE_HEIGHT_REAPER: i32 = 36;

const SPRITE_HEIGHT_FRUIT: i32 = 16;
const SPRITE_WIDTH_FRUIT: i32 = 16;

const SPRITE_HEIGHT_CHEST: i32 = 24;
const SPRITE_WIDTH_CHEST: i32 = 24;

const ANIMATION_N_FRAMES: u8 = 3;
const ANIMATION_N_FRAMES_CHEST: u8 = 4;


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
                SPRITE_WIDTH_CHEST * i,
                0,
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

enum Spawner {
    Fruit,
    Chests,
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
        // with.(ability{name, range, animation, effects?})

    Ok(())
}

pub fn add_reaper(world: &mut World) -> Result<(), String> {
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
    let starting_velocity_npc: VecDeque<Direction> = VecDeque::new();

    let dialogue = Dialogue {
        sprite : Sprite {
            spritesheet: 4,
            region: Rect::new(0, 0, 800, 200)
        },
        dialogue_file: "assets/test_dialogue.txt".to_string(),
        show: false,
    };

    world
        .create_entity()
        .with(Position(Point::new(50, 50)))
        .with(NPCWalker)
        .with(Velocity {
            speed: 0,
            direction: starting_velocity_npc,
        })
        .with(reaper_animations.down_frames[0])
        .with(Interactable{
            interaction_type: InteractableType::Character,
            interactions: 0,
            max_interactions: 0,
        })
        .with(Unplayable)
        .with(CollisionBox {
            width: SPRITE_WIDTH_REAPER as u32,
            height: SPRITE_HEIGHT_REAPER as u32,
        })
        .with(reaper_animations.clone())
        .with(dialogue)
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

pub fn spawn_chest(world: &mut World, x: i32, y: i32) -> Result<(), String> {
    let spritesheet = 3;

    let chest_frames = generate_animation_chest(spritesheet);
    let chest_animation = EntityAnimation{
        current_frame: 0,
        frames: chest_frames,
    };
    let i = Interactable {
        interactions: 0,
        max_interactions: 1,
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

#[allow(dead_code)]
pub fn load_dialogue(world: &mut World) -> Result<(), String> {
    let spritesheet = vec![4, 5, 6];
    let small_dialogue_sprite = Sprite {
        region: Rect::new(0, 0, 800, 200),
        spritesheet: spritesheet[0],
    };
    let medium_dialogue_sprite = Sprite {
        region: Rect::new(0, 0, 800, 400),
        spritesheet: spritesheet[1],
    };
    let large_dialogue_sprite = Sprite {
        region: Rect::new(0, 0, 800, 600),
        spritesheet: spritesheet[2],
    };

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
        .with(animator::Animator, "Animator", &["Keyboard", "Physics"])
        .with(randomwalker::RandomWalker, "RandomWalker", &["Physics"])
        .with(collectibles::Collectibles, "Collectibles",&["Physics", "Animator", "Keyboard"])
        .with(update_interaction::IZUpdater, "Interaction Zone", &["Physics", "Keyboard"])
        .build();

    let mut world_clock: Option<Instant> = None;

    let mut world = World::new();
    dispatcher.setup(&mut world);
    renderer::SystemData::setup(&mut world);

    let movement_command: VecDeque<Option<MovementCommand>> = VecDeque::new();
    let player_command: Option<PlayerCommands> = None;

    let mut draw_bounding_box = true;
    let mut draw_interaction_zone = true;
    let mut thegame = Gamestate::Running;

    // This lint is incorrect. I think.
    let mut dialogue_list: VecDeque<Dialogue_Single_item> = VecDeque::new();
    let mut previous_dialogue_text = Dialogue_Helper{text: String::from("."), width: 0, height: 0};

    world.insert(movement_command);
    world.insert(world_clock);
    world.insert(player_command);
    world.insert(thegame);
    world.insert(dialogue_list);
    world.insert(previous_dialogue_text);
    world.register::<EntityAnimation>();


    let textures = [
        texture_creator.load_texture("assets/bardo.png")?,
        texture_creator.load_texture("assets/reaper.png")?,
        texture_creator.load_texture("assets/food.png")?,
        texture_creator.load_texture("assets/chest.png")?,
        texture_creator.load_texture("assets/dialogue_800x200.png")?,
        texture_creator.load_texture("assets/dialogue_800x400.png")?,
        texture_creator.load_texture("assets/dialogue_800x600.png")?,
    ];

    let mut texture_idx = HashMap::new(); // maybe unused? 
    texture_idx.insert("player".to_string(), 0);
    texture_idx.insert("reaper".to_string(), 1);
    texture_idx.insert("food".to_string(), 2);
    texture_idx.insert("chest".to_string(), 3);
    texture_idx.insert("dialogue_small".to_string(), 4);
    texture_idx.insert("dialogue_medium".to_string(), 5);
    texture_idx.insert("dialogue_large".to_string(), 6);

    let mut spawn_index = Spawner::Chests;

    add_player(&mut world)?;
    add_reaper(&mut world)?;

    world_clock = Some(Instant::now());

    // Gradient test
    let mut background_texture = texture_creator.
    create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
        .map_err(|e| e.to_string())?;
    // Create a red-green gradient
    background_texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        let perlin = Perlin::new();

        fn noise(x: f64, y: f64, per: &Perlin) -> f64 {
            let p: [f64; 2] = [x as f64, y as f64];
            per.get(p)
            //println!("x = {}, y = {}, out = {}", x, y, out);
        }

        for y in 0..256 {
            for x in 0..256 {
                let offset = y*pitch + x*3;
                let output = (noise(x as f64 /255.0, y as f64/255.0, &perlin)*100f64) as u8;
                //println!("{}", output);
                buffer[offset] = output;
                buffer[offset + 1] = output;
                buffer[offset + 2] = output;
            }
        }
    })?;

    let mut color: Color = Color::RGB(100, 255, 0);
    let mut i: i64 = 4;
    let mut going_up: i8 = 3;

    let mut event_pump = sdl_context.event_pump()?;


    'running: loop {
        if (i > 100) | (i < 3) {
            going_up *= -1;
        }
        i += going_up as i64;
        color.b = 100 + i as u8;
        let mut movement_command: VecDeque<Option<MovementCommand>> = VecDeque::new();
        let mut player_command: Option<PlayerCommands> = None;
        //println!("Gamestate before catching events: {:?}", thegame);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    repeat: false,
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(key),
                    ..
                } => {
                    match key {
                        // Player Actions
                        Keycode::Up => movement_command.push_back(Some(MovementCommand::Move(Direction::Up))),
                        Keycode::Down => movement_command.push_back(Some(MovementCommand::Move(Direction::Down))),
                        Keycode::Right => movement_command.push_back(Some(MovementCommand::Move(Direction::Right))),
                        Keycode::Left => movement_command.push_back(Some(MovementCommand::Move(Direction::Left))),
                        Keycode::Z => {player_command = Some(PlayerCommands::Interact)},


                        // Debugging
                        Keycode::F1 => draw_bounding_box = !draw_bounding_box,
                        Keycode::F2 => draw_interaction_zone = !draw_interaction_zone,
                        Keycode::Num1 => spawn_index = Spawner::Chests,
                        Keycode::Num2 => spawn_index = Spawner::Fruit,
                        Keycode::Kp0 => {
                            thegame = Gamestate::Pause;
                            *world.write_resource() = thegame;
                        }
                        Keycode::Kp1 => {
                            thegame = Gamestate::Running;
                            *world.write_resource() = thegame;
                            //println!("Setting Gamestate to Running");
                        },
                        _ => {}
                    }
                }

                Event::KeyUp {
                    keycode: Some(key),
                    repeat: false,
                    ..
                } => {
                    match key {
                        Keycode::Up => movement_command.push_back(Some(MovementCommand::Stop(Direction::Up))),
                        Keycode::Down => movement_command.push_back(Some(MovementCommand::Stop(Direction::Down))),
                        Keycode::Right => movement_command.push_back(Some(MovementCommand::Stop(Direction::Right))),
                        Keycode::Left => movement_command.push_back(Some(MovementCommand::Stop(Direction::Left))),
                        _ => {}
                    }
                }


                Event::MouseButtonDown{x, y, ..} => {
                    let (w, h) = canvas.output_size()?;
                    let w = w as i32;
                    let h = h as i32;
                    match spawn_index {
                        Spawner::Chests => spawn_chest(&mut world, x-w/2, y-h/2)?,
                        Spawner::Fruit => spawn_fruit(&mut world, x-w/2, y-h/2)?,
                    }

                },

                _ => {}
            }
        }
        //println!("Gamestate after catching events: {:?}", thegame);
        *world.write_resource() = movement_command;
        *world.write_resource() = world_clock;
        *world.write_resource() = player_command;
        //*world.write_resource() = thegame;

        // Update
        dispatcher.dispatch(&world);
        world.maintain();

        // Render
        renderer::render(&mut canvas,
            color,
            &textures,
            world.system_data(),
            draw_bounding_box,
            draw_interaction_zone,
            &background_texture,
        )?;
        // Time Management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
