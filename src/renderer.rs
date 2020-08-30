use specs::{ReadStorage, join::Join, ReadExpect};
use crate::components::*;
use sdl2::render::{WindowCanvas, Texture, TextureCreator};
use sdl2::pixels::Color;
use sdl2::{video::WindowContext, rect::{Point, Rect}};
use std::collections::{HashMap, VecDeque};

pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
    ReadStorage<'a, CollisionBox>,
    ReadStorage<'a, InteractionZone>,
    ReadExpect<'a, Gamestate>,
    ReadStorage<'a, Dialogue>,
    ReadExpect<'a, VecDeque<Dialogue_Single_item>>
);

pub fn update_canvas (
        pos: &Position, 
        sprite: &Sprite, 
        origin: Point, 
        canvas: &mut WindowCanvas, 
        textures: &[Texture],
    ) -> Result<(), String> {
    let current_frame = sprite.region;
    let screen_coord = origin + pos.0;

    let screen_rect = Rect::from_center(screen_coord, current_frame.width(), current_frame.height());

    canvas.copy(&textures[sprite.spritesheet], current_frame, screen_rect)?;
    
    Ok(())
}

pub fn text_to_texture<'a>(texture_creator: &'a TextureCreator<WindowContext>, text: &str) -> Result<Texture<'a>, String> {

    // Load a font
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font = ttf_context.load_font("/usr/share/fonts/truetype/liberation/LiberationSerif-Regular.ttf", 128)?;
    
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    // render a surface, and convert it to a texture bound to the canvas
    let surface = font.render(text)
        .blended(Color::RGBA(255, 0, 0, 255))
        .map_err(|e| e.to_string())?;
    let texture = texture_creator.create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    Ok(texture)
}

pub fn render(
    canvas: &mut WindowCanvas,
    background: Color,
    textures: &[Texture],
    (
        position, 
        sprite,
        collision,
        interaction,
        gamestate,
        dialogue,
        dialogue_list,
    ): SystemData,
    draw_bounding_boxes: bool,
    draw_interaction_zone: bool,
    background_texture: &Texture,
) -> Result<(), String> {
    canvas.set_draw_color(background);
    canvas.clear();
    canvas.copy(&background_texture, None, Some(Rect::new(0, 0, 800, 600)))?;

    let (width, height) = canvas.output_size()?;
    let origin = Point::new(width as i32 / 2, height as i32 /2);
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    
    for (pos, sprite, col) in (&position, &sprite, (&collision).maybe()).join() {
        match col {
            Some(col) => {
                update_canvas(
                    pos,
                    sprite,
                    origin,
                    canvas,
                    textures,
                )?;
                if draw_bounding_boxes {
                    let screen_coord = origin + pos.0;
                    canvas.draw_rect(Rect::from_center(screen_coord, (*col).width, (*col).height))?;
                }
            },
            None => {
                update_canvas(
                    pos,
                    sprite,
                    origin,
                    canvas,
                    textures,
                    )?;
            },
        }
    }
    let dialogue_text: &str;
    let texture_creator = canvas.texture_creator();
    if *gamestate == Gamestate::Dialogue {

        if let Some(dialogue_item) = dialogue_list.front() {
            println!("{}",dialogue_item.dialogue_text);
            let txt_texture = text_to_texture(&texture_creator, &(dialogue_item.dialogue_text)).unwrap();
            canvas.copy(&txt_texture, None, Some(Rect::new(0,600-200,800,200)))?;

        }
        
    }

    // Debug function
    if draw_interaction_zone {
        for intzone in (&interaction).join() {
            
            let mut zone = intzone.rect;
            let screen_coord = origin + zone.center();
            zone.center_on(screen_coord);
            
            canvas.draw_rect(zone)?;
        }
    }

    //canvas.copy(&texture, None, Some(Rect::new(0,0,200,200)))?;
    canvas.present();

    Ok(())
}
