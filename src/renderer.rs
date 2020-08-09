use specs::{ReadStorage, join::Join};
use crate::components::*;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
    ReadStorage<'a, CollisionBox>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    background: Color,
    textures: &[Texture],
    data: SystemData,
) -> Result<(), String> {

    canvas.set_draw_color(background);
    canvas.clear();

    let (width, height) = canvas.output_size()?;
    let origin = Point::new(width as i32 / 2, height as i32 /2);
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    
    for (pos, sprite, _) in (&data.0, &data.1, !&data.2).join() {
        let current_frame = sprite.region;
        let screen_coord = origin + pos.0;

        let screen_rect = Rect::from_center(screen_coord, current_frame.width(), current_frame.height());
        canvas.copy(&textures[sprite.spritesheet], current_frame, screen_rect)?;

    }

    for (pos, sprite, col) in (&data.0, &data.1, &data.2).join() {
        let current_frame = sprite.region;
        let screen_coord = origin + pos.0;

        let screen_rect = Rect::from_center(screen_coord, current_frame.width(), current_frame.height());
        canvas.copy(&textures[sprite.spritesheet], current_frame, screen_rect)?;
        canvas.draw_rect(Rect::from_center(screen_coord, col.width, col.height))?;

    }

    canvas.present();

    Ok(())
}
