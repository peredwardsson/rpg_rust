use specs::{ReadStorage, join::Join, ReadExpect};
use crate::components::*;
use sdl2::render::{WindowCanvas, Texture, TextureCreator, TextureQuery};
use sdl2::pixels::Color;
use sdl2::{video::WindowContext, rect::{Point, Rect}};
use std::collections::{VecDeque};
use shred::WriteExpect;

const FONT_SIZE_DIALOGUE: u16 = 20;
const STD_FONT_COLOR: Color = Color::RGBA(90, 170, 230, 230);

pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
    ReadStorage<'a, CollisionBox>,
    ReadStorage<'a, InteractionZone>,
    ReadExpect<'a, Gamestate>,
    ReadExpect<'a, VecDeque<Dialogue_Single_item>>,
    WriteExpect<'a, Dialogue_Helper>,
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
    let mut font = ttf_context.load_font("/usr/share/fonts/opentype/LandasansUltraLight.otf", FONT_SIZE_DIALOGUE)?;
    
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    // render a surface, and convert it to a texture bound to the canvas
    //println!("{}", text);
    let surface = font.render(text.trim())
        .blended(STD_FONT_COLOR)
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
        dialogue_list,
        mut previous_dialogue,
    ): SystemData,
    draw_bounding_boxes: bool,
    draw_interaction_zone: bool,
    background_texture: &Texture,
) -> Result<(), String> {
    canvas.set_draw_color(background);
    canvas.clear();
    //canvas.copy(&background_texture, None, Some(Rect::new(0, 0, 800, 600)))?;

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
    
    let texture_creator = canvas.texture_creator();
    let mut txt_texture = text_to_texture(&texture_creator, &previous_dialogue.text).unwrap(); 

    if *gamestate == Gamestate::Dialogue {
        if let Some(dialogue_item) = dialogue_list.front() {

            if dialogue_item.dialogue_text != previous_dialogue.text {
                // Update the texture if there's new dialogue
                txt_texture = text_to_texture(&texture_creator, &(dialogue_item.dialogue_text)).unwrap();
                let TextureQuery { width, height, .. } = txt_texture.query();
                previous_dialogue.text = dialogue_item.dialogue_text.clone();
                previous_dialogue.width = width;
                previous_dialogue.height = height;
            }
            canvas.copy(&txt_texture, None, Some(Rect::new(0,600-200, previous_dialogue.width, previous_dialogue.height)))?;
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
