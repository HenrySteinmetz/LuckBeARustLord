use pix_engine::prelude::*;
use rand::Rng;
use std::process::exit;
mod slot_machine;
use slot_machine::calculate::*;
use slot_machine::Item;
use slot_machine::Rarities;
use slot_machine::State;

#[derive(Clone)]
pub struct GameState {
    items: Vec<Item>,
    draw_items: Vec<Item>,
    options: Vec<Item>,
    textures: Vec<TextureId>,
    money: i128,
    reroll_capsules: u8,
    removal_capsules: u8,
    floor: u8,
    rent_cycle: u16,
    spins_till_rent: u8,
    odds_multiplier: i8,
    spin_clicked: bool,
    resolution: usize,
    state: State,
    pause_state: Pause,
}
// State for the pause menu
#[derive(Clone)]
enum Pause {
    Main,
    Settings,
    Exit,
    Quit,
}

impl std::fmt::Display for Rarities {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn item_to_image(item: Item) -> Image {
    let str = item.to_text() + ".png";
    let path = "./textures/".to_string() + &str;
    println!("{}", path);
    Image::from_file(&path).unwrap()
}

fn image_alloc(img: Image, s: &mut PixState) -> PixResult<TextureId> {
    let texture_id = s.create_texture(120, 120, None)?;
    s.set_texture_target(texture_id)?;
    s.image(&img, point!(0, 0))?;
    s.clear_texture_target();
    Ok(texture_id)
}

impl GameState {
    fn get_options(rent_cycle: u16) -> Vec<Item> {
        // NOTE: this function doesn't take the luck multiplier into account
        // and allows duplicate selection
        let mut items = vec![];
        for _ in 0..3 {
            let rarity = match rent_cycle {
                1 => Rarities::Common,
                2 => match rand::thread_rng().gen_range(0..10) {
                    9 => Rarities::Uncommon,
                    _ => Rarities::Common,
                },
                3 => match rand::thread_rng().gen_range(0..100) {
                    99 => Rarities::Rare,
                    79..=98 => Rarities::Uncommon,
                    _ => Rarities::Common,
                },
                4 => match rand::thread_rng().gen_range(0..100) {
                    99 => Rarities::Rare,
                    74..=98 => Rarities::Uncommon,
                    _ => Rarities::Common,
                },
                5 => match rand::thread_rng().gen_range(0..200) {
                    0 => Rarities::VeryRare,
                    1..=3 => Rarities::Rare,
                    4..=62 => Rarities::Uncommon,
                    _ => Rarities::Common,
                },
                _ => match rand::thread_rng().gen_range(0..200) {
                    0 => Rarities::VeryRare,
                    1..=3 => Rarities::Rare,
                    4..=63 => Rarities::Uncommon,
                    _ => Rarities::Common,
                },
            };
            let item = match rarity {
                Rarities::Common => Item::get_common(),
                Rarities::Uncommon => Item::get_uncommon(),
                Rarities::Rare => Item::get_rare(),
                Rarities::VeryRare => Item::get_very_rare(),
                _ => panic!("Special or unknown item!"),
            };
            items.push(item);
        }
        items
    }

    pub fn new() -> Self {
        let mut items: Vec<Item> = vec![];
        let textures: Vec<TextureId> = Vec::with_capacity(23);
        for _ in 0..20 {
            items.push(Item::Empty);
        }
        let rent_cycle: u16 = 0;
        GameState {
            items: items.clone(),
            textures,
            draw_items: items,
            options: Self::get_options(rent_cycle),
            floor: 0,
            money: 0,
            reroll_capsules: 0,
            removal_capsules: 0,
            odds_multiplier: 1,
            rent_cycle,
            spins_till_rent: 5,
            spin_clicked: false,
            state: State::Selecting,
            pause_state: Pause::Main,
            resolution: 1,
        }
    }
    fn calculate(&mut self) -> i128 {
        let pre_rolled = remove_empty(self.items.clone());
        let temp = roll(pre_rolled);
        self.draw_items = temp.0;
        let removed = temp.1;
        let (temp_items, cards): (Vec<Item>, Vec<(u8, Item)>) =
            preprocessing(self.draw_items.clone()).unwrap_or((self.items.clone(), vec![]));
        let (val, its, funcs) = value_calc(temp_items);
        let temp_its = postprocessing(its, funcs);
        let mut idk_items = re_add_cards(temp_its, cards);
        match removed {
            Some(_) => {
                println!("{:?}", removed);
                for x in removed.unwrap() {
                    idk_items.push(x);
                }
                self.items = idk_items;
                val
            }
            None => {
                self.items = idk_items;
                val
            }
        }
    }
    fn vscale<I: Into<f32>>(self, i: I) -> f32 {
        let resolution: f32 = match self.resolution {
            0 => 720.0,
            1 => 1080.0,
            2 => 1440.0,
            _ => unreachable!("Invalid resolution")
        };
        let scale_factor: f32 = resolution / 1080.0;
        i.into() * scale_factor
    }
     fn hscale<I: Into<f32>>(self, i: I) -> f32 {
        let resolution: f32 = match self.resolution {
            0 => 1280.0,
            1 => 1920.0,
            2 => 2560.0,
            _ => unreachable!("Invalid resolution")
        };
        let scale_factor: f32 = resolution / 1920.0;
        i.into() * scale_factor
    }   
}

impl PixEngine for GameState {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        // Reusable texures
        s.blend_mode(BlendMode::Blend);
        for x in  0..3 {
            self.textures[x] = image_alloc( item_to_image(self.options[x]), s).unwrap();
        }
        for y in 3..23 {
            self.textures[y] = image_alloc(item_to_image(Item::Empty), s).unwrap();
        }
        println!("{:?}", self.textures);

        // Theme defintion
        let mut fonts = theme::Fonts::default();
        fonts.body = Font::INCONSOLATA;
        fonts.heading = Font::from_file("pixelated", "./fonts/pixelated.ttf");
        fonts.monospace = Font::INCONSOLATA;
        
        let mut colors = theme::Colors::dark();
        colors.on_background = color!(17, 17, 27);
        colors.primary = color!(30, 30, 46);
        colors.primary_variant = color!(108, 112, 134);
        colors.on_background = color!(166, 173, 200);
        colors.secondary = color!(205, 214, 244);
        colors.secondary_variant = color!(186, 194, 222);
        colors.error = color!(253, 139, 168);
        colors.surface = color!(49, 50, 68);
        colors.on_surface = color!(49, 50, 68);
        colors.on_primary = color!(205, 214, 244);
        colors.on_secondary = color!(116, 199, 236);
        colors.on_error = color!(253, 139, 168);
        // Theme assignment
        let mut catppuccin = Theme::builder().build();
        catppuccin.colors = colors;
        catppuccin.fonts = fonts;
        catppuccin.font_size = 30; // Needs to scale in the future
        catppuccin.styles = theme::FontStyles::default();
        catppuccin.spacing = theme::Spacing::builder().build();
        s.set_theme(catppuccin);
        Ok(())
    }
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.show_frame_rate(true);
        let tile_size = (3.0 / 48.0 * s.width().unwrap() as f32) as u32;
        s.clear()?;
        match self.state {
            // Doesn't scale with 1440p
            State::Normal => {
                let rect = rect!(
                    (s.width()? as f32 / 1920.0 * 659.0) as i32,
                    (s.height()? as f32 / 1080.0 * 239.0) as i32,
                    (tile_size * 5) as i32 - 1,
                    (tile_size * 4) as i32 - 1
                );
                s.fill(Color::rgb(49, 50, 68));
                s.rect(rect)?;
                for x in 0..self.draw_items.len() {
                    let p = point!(x % 5, (x as f32 / 5.0).floor() as usize);
                    let x_offset = rect.x() as usize;
                    let y_offset = rect.y() as usize;
                    let location = point!(
                        (p.x() * tile_size as usize + x_offset) as i32,
                        (p.y() * tile_size as usize + y_offset) as i32
                    );
                    let texture_id = s.create_texture(tile_size, tile_size, None)?;
                    let image = item_to_image(self.draw_items[x]);
                    s.image(&image, location)?;
                    s.set_texture_target(texture_id)?;
                    if s.hovered() {
                        let tooltip = self.draw_items[x].get_description();
                        s.tooltip(tooltip)?;
                    }
                    s.clear_texture_target();
                }
            }
            State::Paused => match self.pause_state {
                Pause::Main => {
                    s.font_size(30)?;
                    s.fill(color!(49, 50, 68));
                    if s.button("Settings")? {
                        self.pause_state = Pause::Settings;
                    }
                    if s.button("Exit")? {
                        self.pause_state = Pause::Exit;
                    }
                    if s.button("Quit")? {
                        self.pause_state = Pause::Quit;
                    }
                }
                Pause::Settings => {
                    s.font_size(46)?;
                    s.text("Settings")?;
                    s.font_size(30)?;
                    s.indent()?;
                    s.radio("1280x720", &mut self.resolution, 0)?;
                    s.indent()?;
                    s.radio("1920x1080", &mut self.resolution, 1)?;
                    s.indent()?;
                    s.radio("2560x1440", &mut self.resolution, 2)?;
                    if s.button("Save")? {
                        match self.resolution {
                            0 => s.set_window_dimensions((1280, 720))?,
                            1 => s.set_window_dimensions((1920, 1080))?,
                            2 => s.set_window_dimensions((2560, 1440))?,
                            _ => unreachable!("unknown resolution"),
                        }
                        self.pause_state = Pause::Main;
                        self.state = State::Normal;
                    }
                }
                Pause::Exit => {
                    self.pause_state = Pause::Main;
                    self.state = State::Normal;
                }
                Pause::Quit => exit(0),
            },
            State::Selecting => {
                let (w, h) = s.dimensions()?;
                // Will be replaced with the built in scale function s.scale()
                let vscale = self.clone().vscale(1 as u16);
                let hscale = self.clone().hscale(1 as u16);

                for z in 0..3 {
                    // Metadata
                    let item = self.options[z as usize];
                    let description = item.get_description();
                    let rarity = item.rarity().to_string();
                    let item_name = item.to_text();
                    let value = item.get_value();
                    let mut text = format!("\nRarity: {rarity}\nValue: {value}\nDescription:\n{description}\n");
                    
                    // Size caclulation for the three boxes
                    let height = ((h as f32 - (hscale * 100.0)) / 1.5) as u32;
                    let width = (w as f32 - (vscale * 100.0) / 3.0) as u32;
                    let x = ((z*600 + 80) as f32 * hscale) as i32;
                    let y = h + 200 - h;
                    let rect = rect!(0, 0, width as i32, height as i32);
                    
                    // Theme changes for drawing the boxes
                    s.theme_mut().spacing.frame_pad.set_x(50);
                    s.theme_mut().spacing.frame_pad.set_y(50);
                    s.fill(color!(30, 30, 46));
                    
                    // Creating the texture
                    let texture_id = s.create_texture(width, height, None)?;
                    s.set_texture_target(texture_id)?;
                    s.rect(rect)?;

                    s.fill(s.theme().colors.secondary_variant);
                    s.wrap(width+50);
                    s.heading(item_name)?;
                    s.text(&mut text)?;

                    s.clear_texture_target();

                    if s.clicked() {
                        self.items.push(item);
                        self.state = State::Normal;
                    }
                    s.blend_mode(BlendMode::Blend);
                    s.texture(texture_id, None, rect!(x, y as i32, width as i32, height as i32))?;
                    
                    // Display image of the item
                    let image_texture = self.textures[z as usize];
                    s.set_texture_target(image_texture)?;
                    s.texture(image_texture, None, rect!(x, (y as f32 -(y as f32 /3.0)) as i32, tile_size as i32, tile_size as i32))?;
                }
                s.text(format!("FPS: {}", s.avg_frame_rate()))?;
            }
            State::GameOver => {
                if s.button("Play again?")? {
                    *self = GameState::new(); // Nice pointer dereff
                }
            }
        }
        Ok(())
    }
    fn on_key_pressed(&mut self, _s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
        match event.key {
            Key::Escape if self.state == State::Normal => self.state = State::Paused,
            Key::Escape if self.state == State::Paused => {
                self.pause_state = Pause::Main;
                self.state = State::Normal;
            }
            Key::Space if self.state == State::Normal => {
                self.money += self.calculate();
                //println!("Items: {:?}", self.items);
                //println!("Display items: {:?}", self.draw_items);
                //println!("Items length: {:?}", self.items.len());
                //println!("Display items: {:?}", self.draw_items.len());
            }
            Key::Return if self.state == State::Normal => self.state = State::GameOver,
            _ => (),
        }
        Ok(false)
    }
}
