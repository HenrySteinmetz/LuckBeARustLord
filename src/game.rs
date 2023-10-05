use pix_engine::prelude::*;
mod slot_machine;
use slot_machine::Item;
use slot_machine::State;
use slot_machine::calculate::roll;
use slot_machine::calculate;

pub struct GameState {
    items: Vec<Item>,
    slots_textures: Vec<Image>,
    money: u128,
    spin_clicked: bool,
    state: State,
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn item_to_image(item: Item) -> Image {
    let str = item.to_string() + ".png";
    let path = "./textures/".to_string() + &str;
    Image::from_file(&path).unwrap()
}

impl GameState {
    pub fn new() -> Self {
        let mut images: Vec<Image> = vec![];
        let mut items: Vec<Item> = vec![];
        for _ in 0..20 {
            items.push(Item::Empty);
            images.push(item_to_image(Item::Empty));
        }
        GameState { items, slots_textures: images, money: 0, spin_clicked: false, state: State::Normal }
    }
}

impl PixEngine for GameState {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        // Todo draw selection menu
        s.background(Color::rgb(30, 30, 46));
        Ok(())
    }
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        let tile_size = (3.0/48.0 * s.width().unwrap() as f32) as u32;
        s.clear()?;
        match self.state {
            State::Normal => {
                s.background(Color::rgb(30, 30, 46));
                s.fill(Color::rgb(49, 50, 68));
                let rect = rect!(                    
                    (s.width().unwrap() as f32/1920.0 * 659.0) as i32,
                    (s.height().unwrap() as f32/1080.0 * 239.0) as i32,
                    (s.width().unwrap() as f32/1920.0 * 602.0) as i32,
                    (s.height().unwrap() as f32/1080.0 * 482.0) as i32,);
                s.rect(rect)?;
                for x in 0..self.items.len() {
                    let p = match x {
                        0|1|2|3|4 => Point::new([x, 0]),
                        5|6|7|8|9 => Point::new([x-5, 1]),
                        10|11|12|13|14 => Point::new([x-10, 2]),
                        15|16|17|18|19 => Point::new([x-15, 3]),
                        _ => panic!("Index out of range!")};
                    let temp_x = (s.width().unwrap() as f32 * (11.0/32.0)) as i32;
                    let temp_y = (s.height().unwrap() as f32 * (2.0/9.0)) as i32;
                    let location = rect![
                        temp_x,
                        temp_y,
                        p.x() as i32 * tile_size as i32 + temp_x,
                        p.y() as i32 * tile_size as i32 + temp_y];
                    s.clear_texture_target();
                    let texture_id = s.create_texture(tile_size, tile_size, None)?;
                    let image = item_to_image(self.items[x]);
                    s.update_texture(texture_id, rect, image.as_bytes(), image.width() as usize)?;
                    s.texture(texture_id, None, location)?;
                }
            }
            State::Paused => {

            }
            State::Selecting => {

            }
            State::GameOver => {

            }
        }
        Ok(())
    }
    fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
        match event.key {
            Key::Escape if self.state == State::Normal => {
                self.state = State::Paused;
            }
            Key::Escape if self.state == State::Paused => {
                self.state = State::Normal;
            }
            Key::Space if self.state == State::Normal => {
                self.items = roll(self.items.clone());

            }
            _ => (),
        }
        Ok(false)
    }
}
