pub mod calculate;
use rand::Rng;

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
pub enum Symbol {
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
    Crow(u8),
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
    GoldenArrow(Direction),
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
    MatryoshkaDoll,
    MatryoshkaDollTwo,
    MatryoshkaDollThree,
    MatryoshkaDollFour,
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
    Pirate(u16),
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
    Watermelon,
    WealthyCapsule,
    Wildcard,
    Wine,
    Witch,
    Wolf,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum State {
    Selecting,
    Paused,
    Normal,
    GameOver,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Rarities {
    Common,
    Uncommon,
    Rare,
    VeryRare,
    Special,
}
impl Symbol {
    pub fn rarity(self) -> Rarities {
        match self {
            Symbol::Anchor
            | Symbol::Banana
            | Symbol::BananaPeel
            | Symbol::Bee
            | Symbol::Beer
            | Symbol::BountyHunter
            | Symbol::Bubble(_)
            | Symbol::Candy
            | Symbol::Cat
            | Symbol::Cheese
            | Symbol::Cherry
            | Symbol::Coal(_)
            | Symbol::Coin
            | Symbol::Crab
            | Symbol::Crow(_)
            | Symbol::Cultist
            | Symbol::Dog
            | Symbol::Dwarf
            | Symbol::Egg
            | Symbol::Flower
            | Symbol::Gambler(_)
            | Symbol::Goldfish
            | Symbol::Goose
            | Symbol::Key
            | Symbol::LightBulb(_)
            | Symbol::Lockbox
            | Symbol::Magpie(_)
            | Symbol::Milk
            | Symbol::Miner
            | Symbol::Monkey
            | Symbol::Mouse
            | Symbol::Ore
            | Symbol::Owl(_)
            | Symbol::Oyster
            | Symbol::Pearl
            | Symbol::Present(_)
            | Symbol::Seed
            | Symbol::ShinyPebble
            | Symbol::Snail(_)
            | Symbol::ThreeSidedDie
            | Symbol::Toddler
            | Symbol::Turtle(_)
            | Symbol::Urn => Rarities::Common,

            Symbol::BarOfSoap(_)
            | Symbol::Bear
            | Symbol::BigOre
            | Symbol::BigUrn
            | Symbol::Billionaire
            | Symbol::BronzeArrow(_)
            | Symbol::BuffingCapsule
            | Symbol::ChemicalSeven
            | Symbol::Chick
            | Symbol::Clubs
            | Symbol::Coconut
            | Symbol::CoconutHalf
            | Symbol::Diamonds
            | Symbol::EssenceCapsule
            | Symbol::FiveSidedDie
            | Symbol::Golem(_)
            | Symbol::Hearts
            | Symbol::HexOfDestruction
            | Symbol::HexOfDraining
            | Symbol::HexOfEmptiness
            | Symbol::HexOfHoarding
            | Symbol::HexOfMidas
            | Symbol::HexOfTedium
            | Symbol::HexOfThievery
            | Symbol::Hooligan
            | Symbol::HustlingCapsule
            | Symbol::ItemCapsule
            | Symbol::Jellyfish
            | Symbol::LuckyCapsule
            | Symbol::MatryoshkaDoll
            | Symbol::Ninja
            | Symbol::Orange
            | Symbol::Peach
            | Symbol::Pinata
            | Symbol::Pufferfish
            | Symbol::Rabbit(_)
            | Symbol::RabbitFluff
            | Symbol::Rain
            | Symbol::RemovalCapsule
            | Symbol::RerollCapsule
            | Symbol::Safe
            | Symbol::SandDollar
            | Symbol::Sapphire
            | Symbol::Sloth(_)
            | Symbol::Spades
            | Symbol::Target
            | Symbol::TediumCapsule
            | Symbol::Thief(_)
            | Symbol::TimeCapsule
            | Symbol::VoidCreature
            | Symbol::VoidFruit
            | Symbol::VoidStone
            | Symbol::WealthyCapsule
            | Symbol::Wine
            | Symbol::Wolf => Rarities::Uncommon,

            Symbol::Amethyst(_)
            | Symbol::Apple
            | Symbol::Bartender
            | Symbol::Beastmaster
            | Symbol::Beehive
            | Symbol::CardShark
            | Symbol::Chef
            | Symbol::Chicken
            | Symbol::Comedian
            | Symbol::Cow
            | Symbol::Dame
            | Symbol::Diver
            | Symbol::Dove
            | Symbol::Emerald
            | Symbol::Farmer
            | Symbol::FrozenFossil(_)
            | Symbol::GeneralZaroff
            | Symbol::Geologist(_)
            | Symbol::GoldenEgg
            | Symbol::Honey
            | Symbol::Joker
            | Symbol::KingMidas
            | Symbol::MagicKey
            | Symbol::Martini
            | Symbol::Mine(_)
            | Symbol::Moon
            | Symbol::MrsFruit
            | Symbol::Omelette
            | Symbol::Pear(_)
            | Symbol::RobinHood(_)
            | Symbol::Ruby
            | Symbol::SilverArrow(_)
            | Symbol::Spirit(_)
            | Symbol::Strawberry
            | Symbol::Sun
            | Symbol::Tomb
            | Symbol::TreasureChest
            | Symbol::Witch => Rarities::Rare,

            Symbol::Diamond
            | Symbol::EldritchCreature
            | Symbol::GoldenArrow(_)
            | Symbol::Highlander
            | Symbol::MegaChest
            | Symbol::MidasBomb
            | Symbol::Pirate(_)
            | Symbol::Watermelon
            | Symbol::Wildcard => Rarities::VeryRare,
            _ => Rarities::Special,
        }
    }

    pub fn get_description(self) -> String {
        let description = match self {
            Symbol::Amethyst(_) => "Whenever another symbol makes this symbol give additional coins, this symbol permanently gives 1 more coin.",
            Symbol::Anchor =>"Gives 4 mor coins when in a corner.",
            Symbol::Apple => "",
            Symbol::Banana => "Adds a banana peel when destroyed.",
            Symbol::BananaPeel => "Destroys adjacent thieves. Destroys itself afterwards.",
            Symbol::BarOfSoap(_) => "Adds a bubble each spin. Destroys itself after giving coins three times.",
            Symbol::Bartender => "Has a 10% chance of adding alkohol.",
            Symbol::Bear => "Destroys adjacent honey. Gives 40 coins for each honey destroyed.",
            Symbol::Beastmaster => "Adjacent animals give 2x more coins.",
            Symbol::Bee => "Adjacent flowers, beehives and honey give 2x more coins. Gives a coin more for each adjacent flower, beehive or honey",
            Symbol::Beehive => "Has a 10% chance of adding honey.",
            Symbol::Beer => "",
            Symbol::BigOre => "Adds 2 gemstones when destroyed.",
            Symbol::BigUrn => "Adds two spirits when destroyed.",
            Symbol::Billionaire => "Adjacent cheese and whine give 2x more coins. Gives 39 coins when destroyed.",
            Symbol::BountyHunter => "Destroys adjacent thieves. Gives 20 coins for each thief destroyed.",
            Symbol::BronzeArrow(_) => "Points a random direction. Symbols that are pointed to give 2x more coins. Destroys targets that are pointed to.",
            Symbol::Bubble(_) => "Destroys itself after giving coins 3 times.",
            Symbol::BuffingCapsule => "Destroys itself. Adjacent symbols give 2x more coins",
            Symbol::Candy => "",
            Symbol::CardShark => "Adjacent suits are wildcards.",
            Symbol::Cat => "Destroys adjacent milk. Gives 9 coins for each milk destroyed.",
            Symbol::Cheese => "",
            Symbol::Chef => "Adjacent food and trinks give 2x more coins.",
            Symbol::ChemicalSeven => "Destroys itself. Gives seven coins and adds one crazy seven item when destoryed.",
            Symbol::Cherry => "",
            Symbol::Chick => "Has a 10% chane to grow into a chicken.",
            Symbol::Chicken => "Has a 5% chance of adding an egg. Has a 1% chance of adding a golden egg.",
            Symbol::Clubs => "Adjacent clubs and spades give one coin more. Gives one more coin if there are at least three suits.",
            Symbol::Coal(_) => "Transforms into a diamond after 20 spins.",
            Symbol::Coconut => "Adds two coconut halves when destroyed.",
            Symbol::CoconutHalf => "",
            Symbol::Coin => "",
            Symbol::Comedian => "Adjacent bananas, banana peels, dogs, monkeys, toddlers and jokers give 3x more coins.",
            Symbol::Cow => "Has a 15% chance of adding milk",
            Symbol::Crab => "Gives 3 more coins for each other crab in the same row.",
            Symbol::Crow(_) => "Gives -3 coins every 4 spins.",
            Symbol::Cultist => "Gives one more coin for each other cultist. Gives one mire coin if there are at least 3 culstists.",
            Symbol::Dame => "Adjacent gemstones give 2x more coins. Destroys adjacent martinis. Gives 40 coins for each martini destoyed.",
            Symbol::Diamond => "Gives one more coin for each other diamond",
            Symbol::Diamonds => "Adjacent diamonds and hearts give one more coin. Gives one more coin if there are at least 3 suits.",
            Symbol::Diver => "Removes adjacent see related objects. Permanently gives one more coin for each symbol removed.",
            Symbol::Dog => "Gives two more coins if adjacent to a human.",
            Symbol::Dove => "If an adjacent symbol would be destroyed, instead it ins't, and this symbol permanently gives one more coin.",
            Symbol::Dud => "Destroys itself after 33 spins. Cannot be removed.",
            Symbol::Dwarf => "Destroys adjacent beer and whine. Gives coins equivalent to 10x the value of the symbols destroyed this way.",
            Symbol::Egg => "Has a 10% chance to transform into a chick.",
            Symbol::EldritchCreature => "Destroys adjacent cultists, witches and hexes. Gives one more coin for each symbol destroyed this way.",
            Symbol::Emerald => "Gives one more coin if there are at least 2 emeralds.",
            Symbol::Empty => "",
            Symbol::EssenceCapsule => "Destroys itself. Gives one essence token when destroyed.",
            Symbol::Farmer => "Adjacent plants fruits and animals give 2x more coins. Adjacent seeds are 50% more likely to grow.",
            Symbol::FiveSidedDie => "Gives between 1 and 5 coins randomly.",
            Symbol::Flower => "",
            Symbol::FrozenFossil(_) => "Destroyes itself after 20 spins. The amount of spins needed is reduced by 5 for each cultist, witch or hex destroyed or removed this game. Adds an eldritch creature when destroyed.",
            Symbol::Gambler(_) => "Gives ? coins when destroyed. '?' increases by 2 coins each spins. Destroys itself when a five or three sided die roll one.",
            Symbol::GeneralZaroff => "Destroys adjacent humans. Gives 25 coins for each symbol removed.",
            Symbol::Geologist(_) => "Destroyes adjacent ores, large ores, pearls, shiny pebbles and sapphires. Permanently gives one more coin for each symbol destroyed.",
            Symbol::GoldenArrow(_) => "Points a random direction. Symbols that are pointed to give 4x more coins. Destroys targets that are pointed to.",
            Symbol::GoldenEgg => "",
            Symbol::Goldfish => "Destroys adjacent bubbles. Gives 15 coins for each bubble destroyed.",
            Symbol::Golem(_) => "Destroyes itself after five spins. Add five ores when destroyed.",
            Symbol::Goose => "Has a 1% chance of adding a golden egg.",
            Symbol::Hearts => "Adjacent diamonds and hearts give one more coin. Gives one more coin if there are atleast three suits.",
            Symbol::HexOfDestruction => "Has a 30% chance to destory an adjacent symbol.",
            Symbol::HexOfDraining => "Has a 30% chance to make a symbol give zero coins.",
            Symbol::HexOfEmptiness => "Has a 30% of forcing you to skip the symbols you can add after a spin.",
            Symbol::HexOfHoarding => "Has a 30% chance of forcing you to add a symbol after this spin.",
            Symbol::HexOfMidas => "Has a 30% chance of adding a coin.",
            Symbol::HexOfTedium => "You are 1.3x less likely to find uncommon, rare and very rare symbols.",
            Symbol::HexOfThievery => "Heas a 30% chance to take 6 coins.",
            Symbol::Highlander => "There can be only one highlander.",
            Symbol::Honey => "",
            Symbol::Hooligan => "Destroys adjacent urns, big urns, and graves. Gives six gold for each symbol destroyed.",
            Symbol::HustlingCapsule => "Destroys itself. Adds a pool ball item when destroyed.",
            Symbol::ItemCapsule => "Destroys itself. Adds one common item when destroyed.",
            Symbol::Jellyfish => "Gives one removal token when removed.",
            Symbol::Joker => "Adjacent suits give 2x more coins.",
            Symbol::Key => "Destroys adjacent lock boxes and chests. Destroys itself afterwards.",
            Symbol::KingMidas => "Adds a coin each spin. Adjacent coins give 3x more coins.",
            Symbol::LightBulb(_) => "Adjacent gemsones give 2x more coins. Destroys itself after making other symbols gain coins five times.",
            Symbol::Lockbox => "Gives 15 coins when destroyed.",
            Symbol::LuckyCapsule => "Destroys itself. At least one of the symbols to add after this spin will be rare or better.",
            Symbol::MagicKey => "Destroys adjacent lock boces and chests. Symbols destroyed this way give 3x more coins. Destroys itself afterwards.",
            Symbol::Magpie(_) => "Gives nine coins every four spins.",
            Symbol::Martini => "",
            Symbol::MatryoshkaDoll => "Destroys itself after 3 spins. Adds a MatryoshkaDollTwo when destroyed.",
            Symbol::MatryoshkaDollTwo => "Destroys itself after 5 spins. Add a MatryoshkaDollThree when destroyed.",
            Symbol::MatryoshkaDollThree => "Destroys itself after 7 spins. Adds MatryoshkaDollFour when destroyed.",
            Symbol::MatryoshkaDollFour => "Destroys itself after 9 spins. Adds MatryoshkaDollFive when destroyed.",
            Symbol::MatryoshkaDollFive => "",
            Symbol::MegaChest => "Gives 100 coins when destroyed.",
            Symbol::MidasBomb => "Destroys itself and all adjacent symbols. Symbols destroyed this way give coins equal to 7x their value.",
            Symbol::Milk => "",
            Symbol::Mine(_) => "Add an ore each spin. Destroys itself after giving coins four times. Adds one pickaxe item when destroyed.",
            Symbol::Miner => "Destroys adjacent ores and big ores. Gives 20 coins for each symbol destroyed.",
            Symbol::Monkey => "Destroys adjacent bananas, coconuts and coconut halves. Gives coins equal to 6x the value of symbols destroyed.",
            Symbol::Moon => "Adjacent owls, rabbits and wolves give 3x more coins.",
            Symbol::Mouse => "Destroys adjacent cheese. Gives 20 coins for each cheese destoryed.",
            Symbol::MrsFruit => "Destroys adjacent fruit. Permanently gives one more coin for each symbol destoryed.",
            Symbol::Ninja => "Gives one less coin for each other ninja.",
            Symbol::Omelette => "Gives two more if adjacent to cheese, eggs, milk, golden eggs or omelettes.",
            Symbol::Orange => "",
            Symbol::Ore => "Adds a gemstone when destroyed.",
            Symbol::Owl(_) => "Gives one more coin every 3 spins.",
            Symbol::Oyster => "Has a 20% chance of adding a pearl. Add a pearl when removed.",
            Symbol::Peach => "Adds a seed when destroyed.",
            Symbol::Pear(_) => "Whenever another symbol makes this symbol give additional coins, this symbol permanently gives one coin more.",
            Symbol::Pearl => "",
            Symbol::Pirate(_) => "Destroys adjacent anchors, beer, coins, lockboxes, safes, oranges and chests. Permanently gives one more coin for each symbol destroyed.",
            Symbol::Pinata => "Adds 7 candy when destroyed.",
            Symbol::Present(_) => "Destroys itself after 12 spins. Gives ten coins when destroyed.",
            Symbol::Pufferfish => "Gives one reroll token when removed.",
            Symbol::Rabbit(_) => "Permanently gives two more coins after giving coins ten times.",
            Symbol::RabbitFluff => "You are 1.2x more likely to find common, uncommon, rare and very rare syblols.",
            Symbol::Rain => "Adjacent flowers give 2x more coins. Adjacent seeds are 50% more likely to grow.",
            Symbol::RemovalCapsule => "Destroys itself. Gives one removal token when destroyed.",
            Symbol::RerollCapsule => "Destroys itself. Gives one reroll token when destroyed.",
            Symbol::RobinHood(_) => "Gives 25 coins every four spins. Adjacent thieves and arrows give 3 coins more. Destroys adjacent billionaires, targets and apples. Gives 15 coins for each sybol destroyed.",
            Symbol::Ruby => "Gives one more coin if there are at least two rubies.",
            Symbol::Safe => "Gives 30 coins when destroyed.",
            Symbol::SandDollar => "Gives ten coins when removed.",
            Symbol::Sapphire => "",
            Symbol::Seed => "Has a 25% chance to grow into any fruit.",
            Symbol::ShinyPebble => "You are 1.1x more likely to find uncommon, rare and very rare symbols.",
            Symbol::SilverArrow(_) => "Points a random direction. Symbols taht are pointed to give 3x more coins. Destroys targets that are pointed to.",
            Symbol::Sloth(_) => "Gives four every two spins.",
            Symbol::Snail(_) => "Gives five every four spins.",
            Symbol::Spades => "Adjacent clubs and spades give one more coin. Gives one more coin if there are at least three suits.",
            Symbol::Spirit(_) => "Destroys itself after giveing coins four times.",
            Symbol::Strawberry => "Givs one more if ther are at least two Strawberries.",
            Symbol::Sun => "Adjacent flowers give 5x more coins. Adjacent seeds are 50% more likely to grow.",
            Symbol::Target => "Gives ten coins when destroyed.",
            Symbol::TediumCapsule => "Destroys itself. Gives five coins when destroyed. At least one of the symbols to add after this spin will be common.",
            Symbol::Thief(_) => "Gives ? coins when destroyed. ? increases by 4 coins ever spin.",
            Symbol::ThreeSidedDie => "Gives between 1 and 3 coins randomly,",
            Symbol::TimeCapsule => "Destroys itself. Add one symbol that was destroyed this game when destroyed. Cannot add a time capsule.",
            Symbol::Toddler => "Destroys adjacent presents, candy, pinatas and bubbles. Gives six coins for each symbol destroyed.",
            Symbol::Tomb => "Has a 6% chance of adding a spirit. Add four spirits when destroyed.",
            Symbol::TreasureChest => "Gives 50 coins when destroyed.",
            Symbol::Turtle(_) => "Gives four coins every three spins.",
            Symbol::Urn => "Adds a spirit when destroyed.",
            Symbol::VoidCreature => "Adjacent empties give one more coin. Destroys itself if adjacent to zero empties. Gives eight coins when destroyed.",
            Symbol::VoidFruit => "Adjacent empties give one more coin. Destroys itself if adjacent to zero empties. Gives eight coins when destroyed.",
            Symbol::VoidStone => "Adjacent empties give one more coin. Destroys itself if adjacent to zero empties. Gives eight coins when destroyed.",
            Symbol::Watermelon => "Gives one more coin for each other watermelon.",
            Symbol::WealthyCapsule => "Destroys itself. Gives ten coins when destroyed.",
            Symbol::Wildcard => "Gives coins equal to the highest value among adjacent symbols.",
            Symbol::Wine => "Permanently gives one more coin adter giving coins eight times.",
            Symbol::Witch => "Adjacent cats, owls, crows, apples, eldritch creatures, spirits and hexes give 2x more coins.",
            Symbol::Wolf => "",
        };
        description.to_owned()
    }
    pub fn get_value(self) -> String {
        let val = match self {
            Self::Amethyst(_) => "1",
            Self::Anchor => "1",
            Self::Apple => "3",
            Self::Banana => "1",
            Self::BananaPeel => "1",
            Self::BarOfSoap(_) => "1",
            Self::Bartender => "3",
            Self::Bear => "2",
            Self::Beastmaster => "2",
            Self::Bee => "1",
            Self::Beehive => "3",
            Self::Beer => "1",
            Self::BigOre => "2",
            Self::BigUrn => "2",
            Self::Billionaire => "0",
            Self::BountyHunter => "1",
            Self::BronzeArrow(_) => "0",
            Self::Bubble(_) => "2",
            Self::BuffingCapsule => "0",
            Self::Candy => "1",
            Self::CardShark => "3",
            Self::Cat => "1",
            Self::Cheese => "1",
            Self::Chef => "2",
            Self::ChemicalSeven => "0",
            Self::Cherry => "1",
            Self::Chick => "1",
            Self::Chicken => "2",
            Self::Clubs => "1",
            Self::Coal(_) => "0",
            Self::Coconut => "1",
            Self::CoconutHalf => "2",
            Self::Coin => "1",
            Self::Comedian => "3",
            Self::Cow => "3",
            Self::Crab => "1",
            Self::Crow(_) => "2",
            Self::Cultist => "0",
            Self::Dame => "2",
            Self::Diamond => "5",
            Self::Diamonds => "1",
            Self::Diver => "2",
            Self::Dog => "1",
            Self::Dove => "2",
            Self::Dud => "0",
            Self::Dwarf => "1",
            Self::Egg => "1",
            Self::EldritchCreature => "4",
            Self::Emerald => "3",
            Self::Empty => "0",
            Self::EssenceCapsule => "-12",
            Self::Farmer => "2",
            Self::FiveSidedDie => "?",
            Self::Flower => "1",
            Self::FrozenFossil(_) => "0",
            Self::Gambler(_) => "1",
            Self::GeneralZaroff => "1",
            Self::Geologist(_) => "2",
            Self::GoldenArrow(_) => "0",
            Self::GoldenEgg => "4",
            Self::Goldfish => "1",
            Self::Golem(_) => "0",
            Self::Goose => "1",
            Self::Hearts => "1",
            Self::HexOfDestruction => "3",
            Self::HexOfDraining => "3",
            Self::HexOfEmptiness => "3",
            Self::HexOfHoarding => "3",
            Self::HexOfMidas => "3",
            Self::HexOfTedium => "3",
            Self::HexOfThievery => "3",
            Self::Highlander => "6",
            Self::Honey => "3",
            Self::Hooligan => "2",
            Self::HustlingCapsule => "-7",
            Self::ItemCapsule => "0",
            Self::Jellyfish => "2",
            Self::Joker => "3",
            Self::Key => "1",
            Self::KingMidas => "1",
            Self::LightBulb(_) => "1",
            Self::Lockbox => "1",
            Self::LuckyCapsule => "0",
            Self::MagicKey => "2",
            Self::Magpie(_) => "-1",
            Self::Martini => "3",
            Self::MatryoshkaDoll => "0",
            Self::MatryoshkaDollTwo => "1",
            Self::MatryoshkaDollThree => "2",
            Self::MatryoshkaDollFour => "3",
            Self::MatryoshkaDollFive => "4",
            Self::MegaChest => "3",
            Self::MidasBomb => "0",
            Self::Milk => "1",
            Self::Mine(_) => "4",
            Self::Miner => "1",
            Self::Monkey => "1",
            Self::Moon => "3",
            Self::Mouse => "1",
            Self::MrsFruit => "2",
            Self::Ninja => "2",
            Self::Omelette => "3",
            Self::Orange => "2",
            Self::Ore => "1",
            Self::Owl(_) => "1",
            Self::Oyster => "1",
            Self::Peach => "2",
            Self::Pear(_) => "1",
            Self::Pearl => "1",
            Self::Pirate(_) => "2",
            Self::Pinata => "1",
            Self::Present(_) => "0",
            Self::Pufferfish => "2",
            Self::Rabbit(_) => "1",
            Self::RabbitFluff => "2",
            Self::Rain => "2",
            Self::RemovalCapsule => "0",
            Self::RerollCapsule => "0",
            Self::RobinHood(_) => "-4",
            Self::Ruby => "3",
            Self::Safe => "1",
            Self::SandDollar => "2",
            Self::Sapphire => "2",
            Self::Seed => "1",
            Self::ShinyPebble => "1",
            Self::SilverArrow(_) => "0",
            Self::Sloth(_) => "0",
            Self::Snail(_) => "0",
            Self::Spades => "1",
            Self::Spirit(_) => "6",
            Self::Strawberry => "3",
            Self::Sun => "3",
            Self::Target => "2",
            Self::TediumCapsule => "0",
            Self::Thief(_) => "-1",
            Self::ThreeSidedDie => "?",
            Self::TimeCapsule => "0",
            Self::Toddler => "1",
            Self::Tomb => "3",
            Self::TreasureChest => "2",
            Self::Turtle(_) => "0",
            Self::Urn => "1",
            Self::VoidCreature => "0",
            Self::VoidFruit => "0",
            Self::VoidStone => "0",
            Self::Watermelon => "4",
            Self::WealthyCapsule => "0",
            Self::Wildcard => "?",
            Self::Wine => "2",
            Self::Witch => "2",
            Self::Wolf => "2",
            _ => unreachable!("Unrecognised symbol!"),
        };
        val.to_owned()
    }
    pub fn get_common() -> Symbol {
        let commons = vec![
            Symbol::Anchor,
            Symbol::Banana,
            Symbol::BananaPeel,
            Symbol::Bee,
            Symbol::Beer,
            Symbol::BountyHunter,
            Symbol::Bubble(3),
            Symbol::Candy,
            Symbol::Cat,
            Symbol::Cheese,
            Symbol::Cherry,
            Symbol::Coal(20),
            Symbol::Coin,
            Symbol::Crab,
            Symbol::Crow(0),
            Symbol::Cultist,
            Symbol::Dog,
            Symbol::Dwarf,
            Symbol::Egg,
            Symbol::Flower,
            Symbol::Gambler(0),
            Symbol::Goldfish,
            Symbol::Goose,
            Symbol::Key,
            Symbol::LightBulb(5),
            Symbol::Lockbox,
            Symbol::Magpie(0),
            Symbol::Milk,
            Symbol::Miner,
            Symbol::Monkey,
            Symbol::Mouse,
            Symbol::Ore,
            Symbol::Owl(0),
            Symbol::Oyster,
            Symbol::Pearl,
            Symbol::Present(12),
            Symbol::Seed,
            Symbol::ShinyPebble,
            Symbol::Snail(0),
            Symbol::ThreeSidedDie,
            Symbol::Toddler,
            Symbol::Turtle(0),
            Symbol::Urn,
        ];
        commons[rand::thread_rng().gen_range(0..commons.len()) as usize]
    }
    pub fn get_uncommon() -> Symbol {
        let uncommons = vec![
            Symbol::BarOfSoap(3),
            Symbol::Bear,
            Symbol::BigOre,
            Symbol::BigUrn,
            Symbol::Billionaire,
            Symbol::BronzeArrow(Direction::North),
            Symbol::BuffingCapsule,
            Symbol::ChemicalSeven,
            Symbol::Chick,
            Symbol::Clubs,
            Symbol::Coconut,
            Symbol::CoconutHalf,
            Symbol::Diamonds,
            Symbol::EssenceCapsule,
            Symbol::FiveSidedDie,
            Symbol::Golem(5),
            Symbol::Hearts,
            Symbol::HexOfDestruction,
            Symbol::HexOfDraining,
            Symbol::HexOfEmptiness,
            Symbol::HexOfHoarding,
            Symbol::HexOfMidas,
            Symbol::HexOfTedium,
            Symbol::HexOfThievery,
            Symbol::Hooligan,
            Symbol::HustlingCapsule,
            Symbol::ItemCapsule,
            Symbol::Jellyfish,
            Symbol::LuckyCapsule,
            Symbol::MatryoshkaDoll,
            Symbol::Ninja,
            Symbol::Orange,
            Symbol::Peach,
            Symbol::Pinata,
            Symbol::Pufferfish,
            Symbol::Rabbit(0),
            Symbol::RabbitFluff,
            Symbol::Rain,
            Symbol::RemovalCapsule,
            Symbol::RerollCapsule,
            Symbol::Safe,
            Symbol::SandDollar,
            Symbol::Sapphire,
            Symbol::Sloth(0),
            Symbol::Spades,
            Symbol::Target,
            Symbol::TediumCapsule,
            Symbol::Thief(0),
            Symbol::TimeCapsule,
            Symbol::VoidCreature,
            Symbol::VoidFruit,
            Symbol::VoidStone,
            Symbol::WealthyCapsule,
            Symbol::Wine,
            Symbol::Wolf,
        ];
        uncommons[rand::thread_rng().gen_range(0..uncommons.len()) as usize]
    }
    pub fn get_rare() -> Symbol {
        let rares = vec![
            Symbol::Amethyst(1),
            Symbol::Apple,
            Symbol::Bartender,
            Symbol::Beastmaster,
            Symbol::Beehive,
            Symbol::CardShark,
            Symbol::Chef,
            Symbol::Chicken,
            Symbol::Comedian,
            Symbol::Cow,
            Symbol::Dame,
            Symbol::Diver,
            Symbol::Dove,
            Symbol::Emerald,
            Symbol::Farmer,
            Symbol::FrozenFossil(20),
            Symbol::GeneralZaroff,
            Symbol::Geologist(2),
            Symbol::GoldenEgg,
            Symbol::Honey,
            Symbol::Joker,
            Symbol::KingMidas,
            Symbol::MagicKey,
            Symbol::Martini,
            Symbol::Mine(4),
            Symbol::Moon,
            Symbol::MrsFruit,
            Symbol::Omelette,
            Symbol::Pear(0),
            Symbol::RobinHood(0),
            Symbol::Ruby,
            Symbol::SilverArrow(Direction::North),
            Symbol::Spirit(4),
            Symbol::Strawberry,
            Symbol::Sun,
            Symbol::Tomb,
            Symbol::TreasureChest,
            Symbol::Witch,
        ];
        rares[rand::thread_rng().gen_range(0..rares.len()) as usize]
    }
    pub fn get_very_rare() -> Symbol {
        let very_rares = vec![
            Symbol::Diamond,
            Symbol::EldritchCreature,
            Symbol::GoldenArrow(Direction::North),
            Symbol::Highlander,
            Symbol::MegaChest,
            Symbol::MidasBomb,
            Symbol::Pirate(2),
            Symbol::Watermelon,
            Symbol::Wildcard,
        ];
        very_rares[rand::thread_rng().gen_range(0..very_rares.len()) as usize]
    }
    // Note:: the following function makes any future items that contain the word North non
    // displayable and would need a check
    pub fn to_text(self) -> String {
        self.to_string().replace("North", "").replace(&['(', ')', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'][..], "")
    }
}
