use specs::{ReadStorage, join::Join, ReadExpect};
use crate::components::*;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::collections::HashMap;

pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
    ReadStorage<'a, CollisionBox>,
    ReadStorage<'a, InteractionZone>,
    ReadExpect<'a, Gamestate>,
    ReadStorage<'a, Dialogue>,
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
    ): SystemData,
    draw_bounding_boxes: bool,
    draw_interaction_zone: bool,
    background_texture: &Texture,
) -> Result<(), String> {
    println!("Renderer: GS = {:?}", *gamestate);

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

    if *gamestate == Gamestate::Dialogue {
        for dial in (&dialogue).join() {
            //update_canvas(pos.0, dial.spritesheet, Point::new(0, 0), canvas, textures)?;
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

    // Load a font
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font = ttf_context.load_font("/usr/share/fonts/truetype/liberation/LiberationSerif-Regular.ttf", 128)?;
    
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    // render a surface, and convert it to a texture bound to the canvas
    let texture_creator = canvas.texture_creator();
    let surface = font.render("Hello Rust!")
        .blended(Color::RGBA(255, 0, 0, 255))
        .map_err(|e| e.to_string())?;
    let texture = texture_creator.create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    canvas.copy(&texture, None, Some(Rect::new(0,0,200,200)))?;
    canvas.present();

    Ok(())
}
