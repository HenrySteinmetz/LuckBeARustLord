use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use crate::Item;

const TILE_SIZE_IN_PXS: i32 = 120;
// Temporary 
const SCREEN_WIDTH_IN_PXS: i32 = 800;
const SCREEN_HEIGHT_IN_PXS: i32 = 600;

pub struct Renderer {canvas: WindowCanvas}
pub struct Point (pub u16, pub u16);

impl Renderer {
    pub fn new(window: Window ) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer{canvas})
    }

    pub fn draw_item(&mut self, item: Item, point: &Point) -> Result<(), String>{
        let x_offset = point.0 as i32 * TILE_SIZE_IN_PXS;
        let y_offset = point.1 as i32 * TILE_SIZE_IN_PXS;

        match item {
            Item::Empty => {
                
                Ok(())
            },
            _ => Ok(()),
        }
    }

    pub fn draw_tile(&mut self, point: &Point, item: Item) -> Result<(), String> {
        let Point(x,y) = point;
        // Background color for the slotmachine 
        self.canvas.set_draw_color(Color::RGB(69, 71, 90));
        self.canvas.fill_rect(Rect::new(
            *x as i32 * TILE_SIZE_IN_PXS as i32,
            *y as i32 * TILE_SIZE_IN_PXS as i32,
            TILE_SIZE_IN_PXS as u32,
            TILE_SIZE_IN_PXS as u32,
        ))?;
        
        self.draw_item(item, point);

        self.canvas.present();
        Ok(())
    }
    
    pub fn draw_ui(&mut self) -> Result<(), String> {
        
        
        Ok(())
    }

    pub fn render(&mut self, items: Vec<Item>) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(30, 30, 46));
        self.canvas.clear();
        
        self.canvas.present();
        Ok(())
    }
}
