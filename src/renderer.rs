use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
use sdl2::rect::Point as SdlPoint;
use sdl2::surface::Surface;
use std::path::Path;
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
pub fn text_to_surface(text: String) -> Surface<'static> {
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let path = std::path::Path::new("fonts/Minecraft.ttf");
    let mut font = ttf_context.load_font(path, 100).unwrap();
    font.set_style(sdl2::ttf::FontStyle::NORMAL);
    let surface = font.render(&text.to_owned()).blended(Color::RGB(205,214,244)).map_err(|e| e.to_string()).unwrap();
    surface
}

impl Renderer {
    pub fn new(window: Window ) -> Result<Renderer, String> {
        let canvas = window.into_canvas().accelerated().build().map_err(|e| e.to_string())?;
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
    pub fn draw_slots(&mut self, items: Vec<Item>) -> Result<(), String> {
        for i in 0..items.len() {
            self.draw_item(items[i], index_to_point(i as u8))?;
        }
        Ok(())
    }
    pub fn draw_ui(&mut self, coins: i128) -> Result<(), String> {
        // Slot grid
        self.canvas.draw_rect(Rect::new(659, 239, 602, 482))?;
        for x in 0..5 {
            self.canvas.draw_line(SdlPoint::new(x*120+780, 240), SdlPoint::new(x*120+780, 720))?;
            self.canvas.draw_line(SdlPoint::new(x*120+780, 240), SdlPoint::new(x*120+780, 720))?;
        }
        // Points
        let texture_creator = self.canvas.texture_creator();
        let coins_text = text_to_surface(coins.to_string());
        self.canvas.copy(&texture_creator.create_texture_from_surface(coins_text).unwrap(), None, Some(Rect::new(1600, 50, 50, 50)))?;

        Ok(())
    }
    pub fn render(&mut self, items: Vec<Item>, coins: i128) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(30, 30, 46));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::WHITE);
        self.draw_slots(items)?;
        self.draw_ui(coins)?;
        self.canvas.present();
        Ok(())
    }
}
