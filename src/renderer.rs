use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
use std::path::Path;
use sdl2::render::Texture;
use crate::Item;

const TILE_SIZE_IN_PXS: i32 = 120;
// Temporary 
//const SCREEN_WIDTH_IN_PXS: i32 = 800;
//const SCREEN_HEIGHT_IN_PXS: i32 = 600;

pub struct Renderer {canvas: WindowCanvas}
#[derive(Debug)]
pub struct Point (pub u16, pub u16);


impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn item_to_file_name(item: Item) -> String {
    let str = item.to_string() + ".png";
    "textures/".to_string() + &str
}

pub fn index_to_point(index: u8) -> Point {
    let i = index as u16;
    match index {
        0|1|2|3|4 => Point(i, 0),
        5|6|7|8|9 => Point(i-5, 1),
        10|11|12|13|14 => Point(i-10, 2),
        15|16|17|18|19 => Point(i-15, 3),
        _ => panic!("Index out of range!"),
    }
}
impl Renderer {
    pub fn new(window: Window ) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer{canvas})
    }
    pub fn draw_item(&mut self, item: Item, point: Point) -> Result<(), String>{
        let x_offset = point.0 as i32 * TILE_SIZE_IN_PXS + 660;
        let y_offset = point.1 as i32 * TILE_SIZE_IN_PXS + 240;
        let file_name = item_to_file_name(item);
        let file_path = Path::new(&file_name);
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator.load_texture(file_path)?;
        let draw_rect = Rect::new(x_offset, y_offset, 120, 120);
        self.canvas.copy(&texture, None, Some(draw_rect))?;
        Ok(())
    }
    pub fn text_to_texture(&mut self, text: &str) -> Box<Texture> {
        let mut font = ttf_context.load_font("fonts/pixelated.ttf", 20).unwrap();
        font.set_style(sdl2::ttf::FontStyle::NORMAL);
        let surface = font
            .render(text)
            .blended(Color::RGBA(255, 0, 0, 255))
            .map_err(|e| e.to_string()).unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string()).unwrap();
        Box::new(texture)
    }
    pub fn draw_slots(&mut self, items: Vec<Item>) -> Result<(), String> {
        for i in 0..items.len() {
            self.draw_item(items[i], index_to_point(i as u8))?;
        }
        Ok(())
    }
    pub fn draw_ui(&mut self) -> Result<(), String> {
        Ok(())
    }
    pub fn render(&mut self, items: Vec<Item>) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.draw_rect(Rect::new(660, 240, 600, 480))?;
        self.draw_ui()?;
        self.draw_slots(items)?;
        self.canvas.present();
        Ok(())
    }
}
