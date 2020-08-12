use specs::{ReadStorage, join::Join};
use crate::components::*;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
    ReadStorage<'a, CollisionBox>,
    ReadStorage<'a, InteractionZone>,
    ReadStorage<'a, Interactable>,
);

pub fn update_canvas<T: Into<Option<CollisionBox>>>(
        pos: &Position, 
        sprite: &Sprite, 
        origin: Point, 
        canvas: &mut WindowCanvas, 
        textures: &[Texture],
        col: T,
        draw_bounding_boxes: bool,
    ) -> Result<(), String> {
    let current_frame = sprite.region;
    let screen_coord = origin + pos.0;

    let screen_rect = Rect::from_center(screen_coord, current_frame.width(), current_frame.height());
    //println!("Updating canvas");
    canvas.copy(&textures[sprite.spritesheet], current_frame, screen_rect)?;
    
    if draw_bounding_boxes {
        let c = match col.into() {
            Some(m) => {m}, 
            _ => return Ok(())
        };
        canvas.draw_rect(Rect::from_center(screen_coord, c.width, c.height))?;
    }
    Ok(())
}

pub fn render(
    canvas: &mut WindowCanvas,
    background: Color,
    textures: &[Texture],
    (position, 
    sprite,
    collision,
    interaction,
    interactable): SystemData,
    draw_bounding_boxes: bool,
    draw_interaction_zone: bool,
) -> Result<(), String> {

    canvas.set_draw_color(background);
    canvas.clear();

    let (width, height) = canvas.output_size()?;
    let origin = Point::new(width as i32 / 2, height as i32 /2);
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    
    for (pos, sprite, _, _) in (&position, &sprite, &interactable, !&collision).join() {

        update_canvas(
            pos,
            sprite,
            origin,
            canvas,
            textures,
            None,
            draw_bounding_boxes,
        )?;
        //canvas.copy(&textures[sprite.spritesheet], current_frame, screen_rect)?;
    }

    for (pos, sprite, col,) in (&position, &sprite, &collision).join() {
        update_canvas(
            pos,
            sprite,
            origin,
            canvas,
            textures,
            (*col).clone(),
            draw_bounding_boxes,
        )?;
        let current_frame = sprite.region;
        let screen_coord = origin + pos.0;

        let screen_rect = Rect::from_center(screen_coord, current_frame.width(), current_frame.height());
        canvas.copy(&textures[sprite.spritesheet], current_frame, screen_rect)?;

        // Debug function
        if draw_bounding_boxes {
            canvas.draw_rect(Rect::from_center(screen_coord, col.width, col.height))?;
        }
    }


    // Debug function
    if draw_interaction_zone {
        for (pos, intzone) in (&position, &interaction).join() {
            
            let mut zone = intzone.rect;
            let screen_coord = origin + zone.center();
            zone.center_on(screen_coord);
            
            canvas.draw_rect(zone)?;
        }
    }

    canvas.present();

    Ok(())
}
