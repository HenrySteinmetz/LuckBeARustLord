pub mod calculate;

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

#[derive(Debug, PartialEq, Eq)]
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
impl Item {
    pub fn rarity(self) -> Rarities {
        match self {
            Item::Anchor |
            Item::Banana |
            Item::BananaPeel |
            Item::Bee |
            Item::Beer |
            Item::BountyHunter|
            Item::Bubble(_)|
            Item::Candy|
            Item::Cat|
            Item::Cheese|
            Item::Cherry|
            Item::Coal(_)|
            Item::Coin|
            Item::Crab|
            Item::Crow(_)|
            Item::Cultist|
            Item::Dog|
            Item::Dwarf|
            Item::Egg|
            Item::Flower|
            Item::Gambler(_)|
            Item::Goldfish|
            Item::Goose|
            Item::Key|
            Item::LightBulb(_)|
            Item::Lockbox|
            Item::Magpie(_)|
            Item::Milk|
            Item::Miner|
            Item::Monkey|
            Item::Mouse|
            Item::Ore|
            Item::Owl(_)|
            Item::Oyster|
            Item::Pearl|
            Item::Present(_)|
            Item::Seed|
            Item::ShinyPebble|
            Item::Snail(_)|
            Item::ThreeSidedDie|
            Item::Toddler|
            Item::Turtle(_)|
            Item::Urn => Rarities::Common,
            
            Item::BarOfSoap(_)|
            Item::Bear|
            Item::BigOre|
            Item::BigUrn|
            Item::Billionaire|
            Item::BronzeArrow(_)|
            Item::BuffingCapsule|
            Item::ChemicalSeven|
            Item::Chick|
            Item::Clubs|
            Item::Coconut|
            Item::CoconutHalf|
            Item::Diamonds|
            Item::EssenceCapsule|
            Item::FiveSidedDie|
            Item::Golem(_)|
            Item::Hearts|
            Item::HexOfDestruction|
            Item::HexOfDraining|
            Item::HexOfEmptiness|
            Item::HexOfHoarding|
            Item::HexOfMidas|
            Item::HexOfTedium|
            Item::HexOfThievery|
            Item::Hooligan|
            Item::HustlingCapsule|
            Item::ItemCapsule|
            Item::Jellyfish|
            Item::LuckyCapsule|
            Item::MatryoshkaDoll|
            Item::Ninja|
            Item::Orange|
            Item::Peach|
            Item::Pinata|
            Item::Pufferfish|
            Item::Rabbit(_)|
            Item::RabbitFluff|
            Item::Rain|
            Item::RemovalCapsule|
            Item::RerollCapsule|
            Item::Safe|
            Item::SandDollar|
            Item::Sapphire|
            Item::Sloth(_)|
            Item::Spades|
            Item::Target|
            Item::TediumCapsule|
            Item::Thief(_)|
            Item::TimeCapsule|
            Item::VoidCreature|
            Item::VoidFruit|
            Item::VoidStone|
            Item::WealthyCapsule|
            Item::Wine|
            Item::Wolf => Rarities::Uncommon,

            Item::Amethyst(_)|
            Item::Apple|
            Item::Bartender|
            Item::Beastmaster|
            Item::Beehive|
            Item::CardShark|
            Item::Chef|
            Item::Chicken|
            Item::Comedian|
            Item::Cow|
            Item::Dame|
            Item::Diver|
            Item::Dove|
            Item::Emerald|
            Item::Farmer|
            Item::FrozenFossil(_)|
            Item::GeneralZaroff|
            Item::Geologist(_)|
            Item::GoldenEgg|
            Item::Honey|
            Item::Joker|
            Item::KingMidas|
            Item::MagicKey|
            Item::Martini|
            Item::Mine(_)|
            Item::Moon|
            Item::MrsFruit|
            Item::Omelette|
            Item::Pear(_)|
            Item::RobinHood(_)|
            Item::Ruby|
            Item::SilverArrow(_)|
            Item::Spirit(_)|
            Item::Strawberry|
            Item::Sun|
            Item::Tomb|
            Item::TreasureChest|
            Item::Witch => Rarities::Rare,

            Item::Diamond|
            Item::EldritchCreature|
            Item::GoldenArrow(_)|
            Item::Highlander|
            Item::MegaChest|
            Item::MidasBomb|
            Item::Pirate(_)|
            Item::Watermelon|
            Item::Wildcard => Rarities::VeryRare,
            _ => Rarities::Special,
        }
    }

    pub fn get_description(self) -> String {
        let description = match self {
            Item::Amethyst(_) => "Whenever another symbol makes this symbol give additional coins, this symbol permanently gives 1 more coin.",
            Item::Anchor =>"Gives 4 mor coins when in a corner.",
            Item::Apple => "",
            Item::Banana => "Adds a banana peel when destroyed.",
            Item::BananaPeel => "Destroys adjacent thieves. Destroys itself afterwards.",
            Item::BarOfSoap(_) => "Adds a bubble each spin. Destroys itself after giving coins three times.",
            Item::Bartender => "Has a 10% chance of adding alkohol.",
            Item::Bear => "Destroys adjacent honey. Gives 40 coins for each honey destroyed.",
            Item::Beastmaster => "Adjacent animals give 2x more coins.",
            Item::Bee => "Adjacent flowers, beehives and honey give 2x more coins. Gives a coin more for each adjacent flower, beehive or honey",
            Item::Beehive => "Has a 10% chance of adding honey.",
            Item::Beer => "",
            Item::BigOre => "Adds 2 gemstones when destroyed.",
            Item::BigUrn => "Adds two spirits when destroyed.",
            Item::Billionaire => "Adjacent cheese and whine give 2x more coins. Gives 39 coins when destroyed.",
            Item::BountyHunter => "Destroys adjacent thieves. Gives 20 coins for each thief destroyed.",
            Item::BronzeArrow(_) => "Points a random direction. Symbols that are pointed to give 2x more coins. Destroys targets that are pointed to.",
            Item::Bubble(_) => "Destroys itself after giving coins 3 times.",
            Item::BuffingCapsule => "Destroys itself. Adjacent symbols give 2x more coins",
            Item::Candy => "",
            Item::CardShark => "Adjacent suits are wildcards.",
            Item::Cat => "Destroys adjacent milk. Gives 9 coins for each milk destroyed.",
            Item::Cheese => "",
            Item::Chef => "Adjacent food and trinks give 2x more coins.",
            Item::ChemicalSeven => "Destroys itself. Gives seven coins and adds one crazy seven item when destoryed.",
            Item::Cherry => "",
            Item::Chick => "Has a 10% chane to grow into a chicken.",
            Item::Chicken => "Has a 5% chance of adding an egg. Has a 1% chance of adding a golden egg.",
            Item::Clubs => "Adjacent clubs and spades give one coin more. Gives one more coin if there are at least three suits.",
            Item::Coal(_) => "Transforms into a diamond after 20 spins.",
            Item::Coconut => "Adds two coconut halves when destroyed.",
            Item::CoconutHalf => "",
            Item::Coin => "",
            Item::Comedian => "Adjacent bananas, banana peels, dogs, monkeys, toddlers and jokers give 3x more coins.",
            Item::Cow => "Has a 15% chance of adding milk",
            Item::Crab => "Gives 3 more coins for each other crab in the same row.",
            Item::Crow(_) => "Gives -3 coins every 4 spins.",
            Item::Cultist => "Gives one more coin for each other cultist. Gives one mire coin if there are at least 3 culstists.",
            Item::Dame => "Adjacent gemstones give 2x more coins. Destroys adjacent martinis. Gives 40 coins for each martini destoyed.",
            Item::Diamond => "Gives one more coin for each other diamond",
            Item::Diamonds => "Adjacent diamonds and hearts give one more coin. Gives one more coin if there are at least 3 suits.",
            Item::Diver => "Removes adjacent see related objects. Permanently gives one more coin for each symbol removed.",
            Item::Dog => "Gives two more coins if adjacent to a human.",
            Item::Dove => "If an adjacent symbol would be destroyed, instead it ins't, and this symbol permanently gives one more coin.",
            Item::Dud => "Destroys itself after 33 spins. Cannot be removed.",
            Item::Dwarf => "Destroys adjacent beer and whine. Gives coins equivalent to 10x the value of the symbols destroyed this way.",
            Item::Egg => "Has a 10% chance to transform into a chick.",
            Item::EldritchCreature => "Destroys adjacent cultists, witches and hexes. Gives one more coin for each symbol destroyed this way.",
            Item::Emerald => "Gives one more coin if there are at least 2 emeralds.",
            Item::Empty => "",
            Item::EssenceCapsule => "Destroys itself. Gives one essence token when destroyed.",
            Item::Farmer => "Adjacent plants fruits and animals give 2x more coins. Adjacent seeds are 50% more likely to grow.",
            Item::FiveSidedDie => "Gives between 1 and 5 coins randomly.",
            Item::Flower => "",
            Item::FrozenFossil(_) => "Destroyes itself after 20 spins. The amount of spins needed is reduced by 5 for each cultist, witch or hex destroyed or removed this game. Adds an eldritch creature when destroyed.",
            Item::Gambler(_) => "Gives ? coins when destroyed. '?' increases by 2 coins each spins. Destroys itself when a five or three sided die roll one.",
            Item::GeneralZaroff => "Destroys adjacent humans. Gives 25 coins for each symbol removed.",
            Item::Geologist(_) => "Destroyes adjacent ores, large ores, pearls, shiny pebbles and sapphires. Permanently gives one more coin for each symbol destroyed.",
            Item::GoldenArrow(_) => "Points a random direction. Symbols that are pointed to give 4x more coins. Destroys targets that are pointed to.",
            Item::GoldenEgg => "",
            Item::Goldfish => "Destroys adjacent bubbles. Gives 15 coins for each bubble destroyed.",
            Item::Golem(_) => "Destroyes itself after five spins. Add five ores when destroyed.",
            Item::Goose => "Has a 1% chance of adding a golden egg.",
            Item::Hearts => "Adjacent diamonds and hearts give one more coin. Gives one more coin if there are atleast three suits.",
            Item::HexOfDestruction => "Has a 30% chance to destory an adjacent symbol.",
            Item::HexOfDraining => "Has a 30% chance to make a symbol give zero coins.",
            Item::HexOfEmptiness => "Has a 30% of forcing you to skip the symbols you can add after a spin.",
            Item::HexOfHoarding => "Has a 30% chance of forcing you to add a symbol after this spin.",
            Item::HexOfMidas => "Has a 30% chance of adding a coin.",
            Item::HexOfTedium => "You are 1.3x less likely to find uncommon, rare and very rare symbols.",
            Item::HexOfThievery => "Heas a 30% chance to take 6 coins.",
            Item::Highlander => "There can be only one highlander.",
            Item::Honey => "",
            Item::Hooligan => "Destroys adjacent urns, big urns, and graves. Gives six gold for each symbol destroyed.",
            Item::HustlingCapsule => "Destroys itself. Adds a pool ball item when destroyed.",
            Item::ItemCapsule => "Destroys itself. Adds one common item when destroyed.",
            Item::Jellyfish => "Gives one removal token when removed.",
            Item::Joker => "Adjacent suits give 2x more coins.",
            Item::Key => "Destroys adjacent lock boxes and chests. Destroys itself afterwards.",
            Item::KingMidas => "Adds a coin each spin. Adjacent coins give 3x more coins.",
            Item::LightBulb(_) => "Adjacent gemsones give 2x more coins. Destroys itself after making other symbols gain coins five times.",
            Item::Lockbox => "Gives 15 coins when destroyed.",
            Item::LuckyCapsule => "Destroys itself. At least one of the symbols to add after this spin will be rare or better.",
            Item::MagicKey => "Destroys adjacent lock boces and chests. Symbols destroyed this way give 3x more coins. Destroys itself afterwards.",
            Item::Magpie(_) => "Gives nine coins every four spins.",
            Item::Martini => "",
            Item::MatryoshkaDoll => "Destroys itself after 3 spins. Adds a MatryoshkaDollTwo when destroyed.",
            Item::MatryoshkaDollTwo => "Destroys itself after 5 spins. Add a MatryoshkaDollThree when destroyed.",
            Item::MatryoshkaDollThree => "Destroys itself after 7 spins. Adds MatryoshkaDollFour when destroyed.",
            Item::MatryoshkaDollFour => "Destroys itself after 9 spins. Adds MatryoshkaDollFive when destroyed.",
            Item::MatryoshkaDollFive => "",
            Item::MegaChest => "Gives 100 coins when destroyed.",
            Item::MidasBomb => "Destroys itself and all adjacent symbols. Symbols destroyed this way give coins equal to 7x their value.",
            Item::Milk => "",
            Item::Mine(_) => "Add an ore each spin. Destroys itself after giving coins four times. Adds one pickaxe item when destroyed.",
            Item::Miner => "Destroys adjacent ores and big ores. Gives 20 coins for each symbol destroyed.",
            Item::Monkey => "Destroys adjacent bananas, coconuts and coconut halves. Gives coins equal to 6x the value of symbols destroyed.",
            Item::Moon => "Adjacent owls, rabbits and wolves give 3x more coins.",
            Item::Mouse => "Destroys adjacent cheese. Gives 20 coins for each cheese destoryed.",
            Item::MrsFruit => "Destroys adjacent fruit. Permanently gives one more coin for each symbol destoryed.",
            Item::Ninja => "Gives one less coin for each other ninja.",
            Item::Omelette => "Gives two more if adjacent to cheese, eggs, milk, golden eggs or omelettes.",
            Item::Orange => "",
            Item::Ore => "Adds a gemstone when destroyed.",
            Item::Owl(_) => "Gives one more coin every 3 spins.",
            Item::Oyster => "Has a 20% chance of adding a pearl. Add a pearl when removed.",
            Item::Peach => "Adds a seed when destroyed.",
            Item::Pear(_) => "Whenever another symbol makes this symbol give additional coins, this symbol permanently gives one coin more.",
            Item::Pearl => "",
            Item::Pirate(_) => "Destroys adjacent anchors, beer, coins, lockboxes, safes, oranges and chests. Permanently gives one more coin for each symbol destroyed.",
            Item::Pinata => "Adds 7 candy when destroyed.",
            Item::Present(_) => "Destroys itself after 12 spins. Gives ten coins when destroyed.",
            Item::Pufferfish => "Gives one reroll token when removed.",
            Item::Rabbit(_) => "Permanently gives two more coins after giving coins ten times.",
            Item::RabbitFluff => "You are 1.2x more likely to find common, uncommon, rare and very rare syblols.",
            Item::Rain => "Adjacent flowers give 2x more coins. Adjacent seeds are 50% more likely to grow.",
            Item::RemovalCapsule => "Destroys itself. Gives one removal token when destroyed.",
            Item::RerollCapsule => "Destroys itself. Gives one reroll token when destroyed.",
            Item::RobinHood(_) => "Gives 25 coins every four spins. Adjacent thieves and arrows give 3 coins more. Destroys adjacent billionaires, targets and apples. Gives 15 coins for each sybol destroyed.",
            Item::Ruby => "Gives one more coin if there are at least two rubies.",
            Item::Safe => "Gives 30 coins when destroyed.",
            Item::SandDollar => "Gives ten coins when removed.",
            Item::Sapphire => "",
            Item::Seed => "Has a 25% chance to grow into any fruit.",
            Item::ShinyPebble => "You are 1.1x more likely to find uncommon, rare and very rare symbols.",
            Item::SilverArrow(_) => "Points a random direction. Symbols taht are pointed to give 3x more coins. Destroys targets that are pointed to.",
            Item::Sloth(_) => "Gives four every two spins.",
            Item::Snail(_) => "Gives five every four spins.",
            Item::Spades => "Adjacent clubs and spades give one more coin. Gives one more coin if there are at least three suits.",
            Item::Spirit(_) => "Destroys itself after giveing coins four times.",
            Item::Strawberry => "Givs one more if ther are at least two Strawberries.",
            Item::Sun => "Adjacent flowers give 5x more coins. Adjacent seeds are 50% more likely to grow.",
            Item::Target => "Gives ten coins when destroyed.",
            Item::TediumCapsule => "Destroys itself. Gives five coins when destroyed. At least one of the symbols to add after this spin will be common.",
            Item::Thief(_) => "Gives ? coins when destroyed. ? increases by 4 coins ever spin.",
            Item::ThreeSidedDie => "Gives between 1 and 3 coins randomly,",
            Item::TimeCapsule => "Destroys itself. Add one symbol that was destroyed this game when destroyed. Cannot add a time capsule.",
            Item::Toddler => "Destroys adjacent presents, candy, pinatas and bubbles. Gives six coins for each symbol destroyed.",
            Item::Tomb => "Has a 6% chance of adding a spirit. Add four spirits when destroyed.",
            Item::TreasureChest => "Gives 50 coins when destroyed.",
            Item::Turtle(_) => "Gives four coins every three spins.",
            Item::Urn => "Adds a spirit when destroyed.",
            Item::VoidCreature => "Adjacent empties give one more coin. Destroys itself if adjacent to zero empties. Gives eight coins when destroyed.",
            Item::VoidFruit => "Adjacent empties give one more coin. Destroys itself if adjacent to zero empties. Gives eight coins when destroyed.",
            Item::VoidStone => "Adjacent empties give one more coin. Destroys itself if adjacent to zero empties. Gives eight coins when destroyed.",
            Item::Watermelon => "Gives one more coin for each other watermelon.",
            Item::WealthyCapsule => "Destroys itself. Gives ten coins when destroyed.",
            Item::Wildcard => "Gives coins equal to the highest value among adjacent symbols.",
            Item::Wine => "Permanently gives one more coin adter giving coins eight times.",
            Item::Witch => "Adjacent cats, owls, crows, apples, eldritch creatures, spirits and hexes give 2x more coins.",
            Item::Wolf => "",
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
            Self::Spades => "6",
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
            _ => unreachable!("Unrecognised symbol!")
        };
        val.to_owned()
    }
}
