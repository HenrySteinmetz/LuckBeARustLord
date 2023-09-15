pub mod calculate;
use calculate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,     // Oben
    South,     // Unten
    East,      // Rechts
    West,      // Links
    NorthEast, // Oben rechts
    NorthWest, // Oben links
    SouthEast, // Unten rechts
    SouthWest, // Unten links
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    Amethyst(u16),
    Anchor,
    Apple,
    Banana,
    BananaPeel,
    BarOfSoap(u8),
    Bartender,
    Bear,
    Beastmaster,
    Bee,
    Beehive,
    Beer,
    BigOre,
    BigUrn,
    Billionaire,
    BountyHunter,
    BronzeArrow(Direction),
    Bubble(u8),
    BuffingCapsule,
    Candy,
    CardShark,
    Cat,
    Cheese,
    Chef,
    ChemicalSeven,
    Cherry,
    Chick,
    Chicken,
    Clubs,
    Coal(u8),
    Coconut,
    CoconutHalf,
    Coin,
    Comedian,
    Cow,
    Crab,
    Crow,
    Cultist,
    Dame,
    Diamond,
    Diamonds,
    Diver,
    Dog,
    Dove,
    Dud,
    Dwarf,
    Egg,
    EldritchCreature,
    Emerald,
    Empty,
    EssenceCapsule,
    Farmer,
    FiveSidedDie,
    Flower,
    FrozenFossil(u8),
    Gambler(u16),
    GeneralZaroff,
    Geologist(u8),
    GoldArrow(Direction),
    GoldenEgg,
    Goldfish,
    Golem(u8),
    Goose,
    Hearts,
    HexOfDestruction,
    HexOfDraining,
    HexOfEmptiness,
    HexOfHoarding,
    HexOfMidas,
    HexOfTedium,
    HexOfThievery,
    Highlander,
    Honey,
    Hooligan,
    HustlingCapsule,
    ItemCapsule,
    Jellyfish,
    Joker,
    Key,
    KingMidas,
    LightBulb(u8),
    Lockbox,
    LuckyCapsule,
    MagicKey,
    Magpie(u8),
    Martini,
    MatryoshkaDoll(u8),
    MatryoshkaDollTwo(u8),
    MatryoshkaDollThree(u8),
    MatryoshkaDollFour(u8),
    MatryoshkaDollFive,
    MegaChest,
    MidasBomb,
    Milk,
    Mine(u8),
    Miner,
    Monkey,
    Moon,
    Mouse,
    MrsFruit,
    Ninja,
    Omelette,
    Orange,
    Ore,
    Owl(u8),
    Oyster,
    Peach,
    Pear(u16),
    Pearl,
    Pirate,
    Pinata,
    Present(u8),
    Pufferfish,
    Rabbit(u8),
    RabbitFluff,
    Rain,
    RemovalCapsule,
    RerollCapsule,
    RobinHood(u8),
    Ruby,
    Safe,
    SandDollar,
    Sapphire,
    Seed,
    ShinyPebble,
    SilverArrow(Direction),
    Sloth(u8),
    Snail(u8),
    Spades,
    Spirit(u8),
    Strawberry,
    Sun,
    Target,
    TediumCapsule,
    Thief(u16),
    ThreeSidedDie,
    TimeCapsule,
    Toddler,
    Tomb,
    TreasureChest,
    Turtle(u8),
    Urn,
    VoidCreature,
    VoidFruit,
    VoidStone,
    Watermelone,
    WealthyCapsule,
    Wildcard,
    Wine,
    Witch,
    Wolf,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    Selecting,
    Paused,
    Normal,
}

pub struct SlotMachine {
    state: State,
}

impl SlotMachine {
    pub fn new() -> (SlotMachine, Vec<Item>) {
        let mut items: Vec<Item> = vec![];
        for _ in 0..20 {
            items.push(Item::Empty);
        }
        (
            SlotMachine {
                state: State::Selecting,
            },
            items,
        )
    }
    pub fn calculate(&mut self, items: Vec<Item>) -> (i128, Vec<Item>, SlotMachine) {
        let (temp_items, cards): (Vec<Item>, Vec<(u8, Item)>) =
            preprocessing(items.clone()).unwrap_or((items, vec![]));
        let (val, its, funcs) = value_calc(temp_items);
        
        let ret_items = postprocessing(its, funcs);
        return (
            val,
            re_add_cards(ret_items, cards),
            SlotMachine {state: State::Normal,},
        );
    }
}
