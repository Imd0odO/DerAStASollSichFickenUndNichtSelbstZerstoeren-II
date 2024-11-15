
pub const MIN_REQUIREMENTS: [Requirement; 14] = [
    // Level: 20
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 3,
        MIN_UNITS_FOR_ATTACK: 5,
        MIN_UNITS_FOR_UPGRADE: 10,
        MIN_UNITS_FOR_DISTRIBUTION: 8,
        MAX_UNITS_FOR_SUPPLY: 5,
    },
    // Level: 40
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 6,
        MIN_UNITS_FOR_ATTACK: 10,
        MIN_UNITS_FOR_UPGRADE: 20,
        MIN_UNITS_FOR_DISTRIBUTION: 16,
        MAX_UNITS_FOR_SUPPLY: 10,
    },
    // Level: 80
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 15,
        MIN_UNITS_FOR_ATTACK: 25,
        MIN_UNITS_FOR_UPGRADE: 40,
        MIN_UNITS_FOR_DISTRIBUTION: 32,
        MAX_UNITS_FOR_SUPPLY: 20,
    },
    // Level: 100
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 25,
        MIN_UNITS_FOR_ATTACK: 40,
        MIN_UNITS_FOR_UPGRADE: 50,
        MIN_UNITS_FOR_DISTRIBUTION: 40,
        MAX_UNITS_FOR_SUPPLY: 24,
    },
    // Level: 200
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 35,
        MIN_UNITS_FOR_ATTACK: 80,
        MIN_UNITS_FOR_UPGRADE: 100,
        MIN_UNITS_FOR_DISTRIBUTION: 80,
        MAX_UNITS_FOR_SUPPLY: 50,
    },
    // Level: 300
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 60,
        MIN_UNITS_FOR_ATTACK: 100,
        MIN_UNITS_FOR_UPGRADE: 150,
        MIN_UNITS_FOR_DISTRIBUTION: 120,
        MAX_UNITS_FOR_SUPPLY: 60,
    },
    // Level: 400
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 150,
        MIN_UNITS_FOR_ATTACK: 200,
        MIN_UNITS_FOR_UPGRADE: 200,
        MIN_UNITS_FOR_DISTRIBUTION: 160,
        MAX_UNITS_FOR_SUPPLY: 80,
    },
    // Level: 500
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 100,
        MIN_UNITS_FOR_ATTACK: 120,
        MIN_UNITS_FOR_UPGRADE: 250,
        MIN_UNITS_FOR_DISTRIBUTION: 250,
        MAX_UNITS_FOR_SUPPLY: 150,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 100,
        MIN_UNITS_FOR_ATTACK: 120,
        MIN_UNITS_FOR_UPGRADE: 300,
        MIN_UNITS_FOR_DISTRIBUTION: 250,
        MAX_UNITS_FOR_SUPPLY: 150,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 100,
        MIN_UNITS_FOR_ATTACK: 120,
        MIN_UNITS_FOR_UPGRADE: 450,
        MIN_UNITS_FOR_DISTRIBUTION: 250,
        MAX_UNITS_FOR_SUPPLY: 150,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 100,
        MIN_UNITS_FOR_ATTACK: 120,
        MIN_UNITS_FOR_UPGRADE: 500,
        MIN_UNITS_FOR_DISTRIBUTION: 250,
        MAX_UNITS_FOR_SUPPLY: 150,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 100,
        MIN_UNITS_FOR_ATTACK: 120,
        MIN_UNITS_FOR_UPGRADE: 550,
        MIN_UNITS_FOR_DISTRIBUTION: 250,
        MAX_UNITS_FOR_SUPPLY: 150,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 100,
        MIN_UNITS_FOR_ATTACK: 120,
        MIN_UNITS_FOR_UPGRADE: 600,
        MIN_UNITS_FOR_DISTRIBUTION: 250,
        MAX_UNITS_FOR_SUPPLY: 150,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 100,
        MIN_UNITS_FOR_ATTACK: 120,
        MIN_UNITS_FOR_UPGRADE: 999_999_999,
        MIN_UNITS_FOR_DISTRIBUTION: 250,
        MAX_UNITS_FOR_SUPPLY: 150,
    },
];

pub struct Requirement {
    pub MIN_UNITS_AFTER_ATTACK: u32,
    pub MIN_UNITS_FOR_ATTACK: u32,
    pub MIN_UNITS_FOR_UPGRADE: u32,
    pub MIN_UNITS_FOR_DISTRIBUTION: u32,
    pub MAX_UNITS_FOR_SUPPLY: u32,
}

