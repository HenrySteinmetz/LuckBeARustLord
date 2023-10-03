use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
use sdl2::rect::Point as SdlPoint;
use sdl2::surface::Surface;
use std::path::Path;
use crate::Item;
use crate::State;

pub struct Renderer {
    canvas: WindowCanvas,
    screen_width: i32,
    screen_height: i32,
}
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
    
    pub fn new(window: Window, screen_width: i32, screen_height: i32 ) -> Result<Renderer, String> {
        let canvas = window.into_canvas().accelerated().build().map_err(|e| e.to_string())?;
        Ok(Renderer{canvas, screen_width, screen_height})
    }

    pub fn draw_item(&mut self, item: Item, point: Point) -> Result<(), String> {
        // Random fractions I got by simplifying the scale for 1080p
        let tile_size = 3.0/48.0 * self.screen_width as f32;
        let x_temp = 33.0/96.0 * self.screen_width as f32;
        let y_temp = 2.0/9.0 * self.screen_height as f32; 
        let x_offset = point.0 as i32 * tile_size as i32 + x_temp as i32;
        let y_offset = point.1 as i32 * tile_size as i32 + y_temp as i32;

        let file_name = item_to_file_name(item);
        let file_path = Path::new(&file_name);
        
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator.load_texture(file_path)?;
        
        let draw_rect = Rect::new(x_offset, y_offset, tile_size as u32, tile_size as u32);
        
        self.canvas.copy(&texture, None, Some(draw_rect))?;
        Ok(())
    }
    
    pub fn draw_slots(&mut self, items: Vec<Item>) -> Result<(), String> {
        for i in 0..items.len() {
            self.draw_item(items[i], index_to_point(i as u8))?;
        }
        Ok(())
    }
    
    pub fn render_pause_menu(&mut self) -> Result<(), String> {
        // Dimm background
        self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 7));
        self.canvas.clear();
        self.canvas.present();
        Ok(())
    }

    pub fn draw_ui(&mut self, coins: i128) -> Result<(), String> {
        let tile_size = 3.0/48.0 * self.screen_width as f32;

        // Slot grid
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.draw_rect(Rect::new(
            (self.screen_width as f32/1920.0 * 659.0) as i32,
            (self.screen_height as f32/1080.0 * 239.0) as i32,
            (self.screen_width as f32/1920.0 * 602.0) as u32,
            (self.screen_height as f32/1080.0 * 482.0) as u32))?;
        for x in 0..5 {
            let start_x = x * tile_size as i32 + (self.screen_width as f32 * 0.40625) as i32;
            let end_y = (self.screen_width as f32 * 0.375) as i32;
            let temp = 4 * tile_size as i32;
            let start_y = end_y - temp;
            self.canvas.draw_line(SdlPoint::new(start_x, start_y), SdlPoint::new(start_x, end_y))?;
        }

        // Texture creator for the entire function
        let texture_creator = self.canvas.texture_creator();

        // Money
        let coins_text = text_to_surface(coins.to_string());
        let x_pos = 5.0/6.0 * self.screen_height as f32;
        let tile_fraction: u32 = 5/12  * tile_size as u32;
        self.canvas.set_draw_color(Color::RGB(30, 30, 46));
        self.canvas.fill_rect(Rect::new(x_pos as i32, 108/5 * self.screen_height, tile_fraction, tile_fraction))?;
        self.canvas.copy(&texture_creator.create_texture_from_surface(coins_text).unwrap(), None, Some(Rect::new(x_pos as i32, tile_fraction as i32, tile_fraction, tile_fraction)))?;

        // Spin button
        let spin = text_to_surface(String::from("Spin"));
        let offset = 5/96 * self.screen_width;
        let spin_rect = Rect::new(self.screen_width/2 -offset, (25.0/36.0 * self.screen_height as f32) as i32, (offset*2) as u32, offset as u32);
        self.canvas.set_draw_color(Color::RGB(69, 71, 90));
        self.canvas.fill_rect(spin_rect)?;
        self.canvas.copy(&texture_creator.create_texture_from_surface(spin).unwrap(), None, Some(spin_rect))?;

        Ok(())
    }

    pub fn render(&mut self, items: Vec<Item>, coins: i128, state: State) -> Result<(), String> {
        match state {
            State::Normal => {
                // Background Color
                self.canvas.set_draw_color(Color::RGB(30, 30, 46));
                self.canvas.clear();

                self.draw_slots(items)?;
                self.draw_ui(coins)?;
            }
            State::Paused => {
                self.render_pause_menu()?;
            }
            State::Selecting => {

            }
        }
        self.canvas.present();
        Ok(())
    }
}
