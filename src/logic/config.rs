
pub const MIN_REQUIREMENTS: [Requirement; 14] = [
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK:15,
        MIN_UNITS_FOR_ATTACK: 5,
        MIN_UNITS_FOR_UPGRADE: 10,
        MIN_UNITS_FOR_DISTRIBUTION: 8,
        MAX_UNITS_FOR_SUPPLY: 4,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 30,
        MIN_UNITS_FOR_ATTACK: 10,
        MIN_UNITS_FOR_UPGRADE: 20,
        MIN_UNITS_FOR_DISTRIBUTION: 16,
        MAX_UNITS_FOR_SUPPLY: 8,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 65,
        MIN_UNITS_FOR_ATTACK: 20,
        MIN_UNITS_FOR_UPGRADE: 30,
        MIN_UNITS_FOR_DISTRIBUTION: 32,
        MAX_UNITS_FOR_SUPPLY: 16,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 80,
        MIN_UNITS_FOR_ATTACK: 25,
        MIN_UNITS_FOR_UPGRADE: 50,
        MIN_UNITS_FOR_DISTRIBUTION: 40,
        MAX_UNITS_FOR_SUPPLY: 20,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 165,
        MIN_UNITS_FOR_ATTACK: 60,
        MIN_UNITS_FOR_UPGRADE: 80,
        MIN_UNITS_FOR_DISTRIBUTION: 80,
        MAX_UNITS_FOR_SUPPLY: 40,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 240,
        MIN_UNITS_FOR_ATTACK: 80,
        MIN_UNITS_FOR_UPGRADE: 150,
        MIN_UNITS_FOR_DISTRIBUTION: 120,
        MAX_UNITS_FOR_SUPPLY: 60,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 316,
        MIN_UNITS_FOR_ATTACK: 100,
        MIN_UNITS_FOR_UPGRADE: 200,
        MIN_UNITS_FOR_DISTRIBUTION: 160,
        MAX_UNITS_FOR_SUPPLY: 80,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 388,
        MIN_UNITS_FOR_ATTACK: 120,
        MIN_UNITS_FOR_UPGRADE: 200,
        MIN_UNITS_FOR_DISTRIBUTION: 200,
        MAX_UNITS_FOR_SUPPLY: 100,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 456,
        MIN_UNITS_FOR_ATTACK: 150,
        MIN_UNITS_FOR_UPGRADE: 300,
        MIN_UNITS_FOR_DISTRIBUTION: 240,
        MAX_UNITS_FOR_SUPPLY: 120,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 520,
        MIN_UNITS_FOR_ATTACK: 275,
        MIN_UNITS_FOR_UPGRADE: 500,
        MIN_UNITS_FOR_DISTRIBUTION: 280,
        MAX_UNITS_FOR_SUPPLY: 140,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 440,
        MIN_UNITS_FOR_ATTACK: 300,
        MIN_UNITS_FOR_UPGRADE: 600,
        MIN_UNITS_FOR_DISTRIBUTION: 320,
        MAX_UNITS_FOR_SUPPLY: 160,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 380,
        MIN_UNITS_FOR_ATTACK: 350,
        MIN_UNITS_FOR_UPGRADE: 650,
        MIN_UNITS_FOR_DISTRIBUTION: 360,
        MAX_UNITS_FOR_SUPPLY: 180,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 300,
        MIN_UNITS_FOR_ATTACK: 375,
        MIN_UNITS_FOR_UPGRADE: 700,
        MIN_UNITS_FOR_DISTRIBUTION: 400,
        MAX_UNITS_FOR_SUPPLY: 200,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 500,
        MIN_UNITS_FOR_ATTACK: 500,
        MIN_UNITS_FOR_UPGRADE: 800,
        MIN_UNITS_FOR_DISTRIBUTION: 800,
        MAX_UNITS_FOR_SUPPLY: 400,
    },
];

pub struct Requirement {
    pub MIN_UNITS_AFTER_ATTACK: u32,
    pub MIN_UNITS_FOR_ATTACK: u32,
    pub MIN_UNITS_FOR_UPGRADE: u32,
    pub MIN_UNITS_FOR_DISTRIBUTION: u32,
    pub MAX_UNITS_FOR_SUPPLY: u32,
}

