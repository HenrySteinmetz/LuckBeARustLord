use pix_engine::prelude::*;
use rand::Rng;
use std::process::exit;
mod slot_machine;
use slot_machine::calculate::*;
use slot_machine::Item;
use slot_machine::State;

pub struct GameState {
    items: Vec<Item>,
    draw_items: Vec<Item>,
    money: i128,
    odds_multiplier: i8,
    spin_clicked: bool,
    resolution: usize,
    state: State,
    pause_state: Pause,
}
// State for the pause menu
enum Pause {
    Main,
    Settings,
    Exit,
    Quit,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Rarities {
    Common,
    Uncommon,
    Rare,
    VeryRare,
}

impl AsRef<str> for Rarities {
    fn as_ref(&self) -> &str {
        match self {
            Self::Common => "Common",
            Self::Uncommon => "Uncommon",
            Self::Rare => "Rare",
            Self::VeryRare => "Very rare",
        }
    }
}

impl AsRef<str> for Pause {
    fn as_ref(&self) -> &str {
        match self {
            Self::Main => "Main",
            Self::Settings => "Settings",
            Self::Exit => "Exit",
            Self::Quit => "Quit",
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn item_to_image(item: Item) -> Image {
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
        GameState {
            items: items.clone(),
            draw_items: items,
            money: 0,
            odds_multiplier: 1,
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
}

impl PixEngine for GameState {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        // Todo draw selection menu
        s.background(Color::rgb(30, 30, 46));
        Ok(())
    }
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
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

        let mut catppuccin = Theme::builder().build();
        catppuccin.colors = colors;
        catppuccin.fonts = fonts;
        catppuccin.font_size = 30;
        catppuccin.styles = theme::FontStyles::default();
        catppuccin.spacing = theme::Spacing::builder().build();
        let tile_size = (3.0 / 48.0 * s.width().unwrap() as f32) as u32;
        s.clear()?;
        s.set_theme(catppuccin);
        match self.state {
            // Doesn't scale with 1440p
            State::Normal => {
                let rect = rect!(
                    (s.width()? as f32 / 1920.0 * 659.0) as i32,
                    (s.height()? as f32 / 1080.0 * 239.0) as i32,
                    (tile_size * 5) as i32,
                    (tile_size * 4) as i32
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
                        // Todo add tooltips for all symbols
                        let mut tooltip = "";
                        match self.items[x] {
                            Item::Empty => tooltip = "Does nothing",
                            _ => (),
                        }
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
                let commons = vec![
                    Item::Anchor,
                    Item::Banana,
                    Item::BananaPeel,
                    Item::Bee,
                    Item::Beer,
                    Item::BountyHunter,
                    Item::Bubble(3),
                    Item::Candy,
                    Item::Cat,
                    Item::Cheese,
                    Item::Cherry,
                    Item::Coal(20),
                    Item::Coin,
                    Item::Crab,
                    Item::Crow(0),
                    Item::Cultist,
                    Item::Dog,
                    Item::Dwarf,
                    Item::Egg,
                    Item::Flower,
                    Item::Gambler(0),
                    Item::Goldfish,
                    Item::Goose,
                    Item::Key,
                    Item::LightBulb(5),
                    Item::Lockbox,
                    Item::Magpie(0),
                    Item::Milk,
                    Item::Miner,
                    Item::Monkey,
                    Item::Mouse,
                    Item::Ore,
                    Item::Owl(0),
                    Item::Oyster,
                    Item::Pearl,
                    Item::Present(12),
                    Item::Seed,
                    Item::ShinyPebble,
                    Item::Snail(0),
                    Item::ThreeSidedDie,
                    Item::Toddler,
                    Item::Turtle(0),
                    Item::Urn,
                ];
                let uncommons = vec![
                    Item::BarOfSoap(3),
                    Item::Bear,
                    Item::BigOre,
                    Item::BigUrn,
                    Item::Billionaire,
                    Item::BronzeArrow(slot_machine::Direction::North),
                    Item::BuffingCapsule,
                    Item::ChemicalSeven,
                    Item::Chick,
                    Item::Clubs,
                    Item::Coconut,
                    Item::CoconutHalf,
                    Item::Diamonds,
                    Item::EssenceCapsule,
                    Item::FiveSidedDie,
                    Item::Golem(5),
                    Item::Hearts,
                    Item::HexOfDestruction,
                    Item::HexOfDraining,
                    Item::HexOfEmptiness,
                    Item::HexOfHoarding,
                    Item::HexOfMidas,
                    Item::HexOfTedium,
                    Item::HexOfThievery,
                    Item::Hooligan,
                    Item::HustlingCapsule,
                    Item::ItemCapsule,
                    Item::Jellyfish,
                    Item::LuckyCapsule,
                    Item::MatryoshkaDoll,
                    Item::Ninja,
                    Item::Orange,
                    Item::Peach,
                    Item::Pinata,
                    Item::Pufferfish,
                    Item::Rabbit(0),
                    Item::RabbitFluff,
                    Item::Rain,
                    Item::RemovalCapsule,
                    Item::RerollCapsule,
                    Item::Safe,
                    Item::SandDollar,
                    Item::Sapphire,
                    Item::Sloth(0),
                    Item::Spades,
                    Item::Target,
                    Item::TediumCapsule,
                    Item::Thief(0),
                    Item::TimeCapsule,
                    Item::VoidCreature,
                    Item::VoidFruit,
                    Item::VoidStone,
                    Item::WealthyCapsule,
                    Item::Wine,
                    Item::Wolf,
                ];
                let rares = vec![
                    Item::Amethyst(1),
                    Item::Apple,
                    Item::Bartender,
                    Item::Beastmaster,
                    Item::Beehive,
                    Item::CardShark,
                    Item::Chef,
                    Item::Chicken,
                    Item::Comedian,
                    Item::Cow,
                    Item::Dame,
                    Item::Diver,
                    Item::Dove,
                    Item::Emerald,
                    Item::Farmer,
                    Item::FrozenFossil(20),
                    Item::GeneralZaroff,
                    Item::Geologist(2),
                    Item::GoldenEgg,
                    Item::Honey,
                    Item::Joker,
                    Item::KingMidas,
                    Item::MagicKey,
                    Item::Martini,
                    Item::Mine(4),
                    Item::Moon,
                    Item::MrsFruit,
                    Item::Omelette,
                    Item::Pear(0),
                    Item::RobinHood(0),
                    Item::Ruby,
                    Item::SilverArrow(slot_machine::Direction::North),
                    Item::Spirit(4),
                    Item::Strawberry,
                    Item::Sun,
                    Item::Tomb,
                    Item::TreasureChest,
                    Item::Witch,
                ];
                let very_rares = vec![
                    Item::Diamond,
                    Item::EldritchCreature,
                    Item::GoldenArrow(slot_machine::Direction::North),
                    Item::Highlander,
                    Item::MegaChest,
                    Item::MidasBomb,
                    Item::Pirate(2),
                    Item::Watermelon,
                    Item::Wildcard,
                ];
                let mut items: Vec<Item> = vec![];
                // Every rarity decreases the chances of beeing choosen by 2 multiplicatively
                // NOTE: the above is the default state and doens't take modifiers into account
                for x in 0..3 {
                    let rarity = match rand::thread_rng().gen_range(0..20) {
                        0|1|2|3|4|5|6|7|8|9 => Rarities::Common,
                        10|11|12|13|14 => Rarities::Uncommon,
                        15|16|17 => Rarities::Rare,
                        18|19 => Rarities::VeryRare,
                        _ => unreachable!("rng out of range"),
                    };
                    let item = match rarity {
                        Rarities::Common => commons[rand::thread_rng().gen_range(0..commons.len()) as usize],
                        Rarities::Uncommon => uncommons[rand::thread_rng().gen_range(0..uncommons.len()) as usize],
                        Rarities::Rare => rares[rand::thread_rng().gen_range(0..rares.len()) as usize],
                        Rarities::VeryRare => very_rares[rand::thread_rng().gen_range(0..very_rares.len()) as usize],
                    };
                    items.push(item);
                    let description = item.get_description();
                    let rarity = item.rarity();
                    let value = item.get_value();
                    let (w, h) = s.dimensions()?;
                    let height = h - (h as f32/1080.0 * 100.0) as u32;
                    let width = (w as f32 / 3.0) as u32 - (w as f32 / 1920.0 * 100.0) as u32;
                }


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
            }
            Key::Return if self.state == State::Normal => self.state = State::GameOver,
            _ => (),
        }
        Ok(false)
    }
}
