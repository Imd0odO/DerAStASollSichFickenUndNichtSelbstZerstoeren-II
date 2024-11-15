
const MIN_REQUIREMENTS: [Requirement; 14] = [
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK:15,
        MIN_UNITS_FOR_ATTACK: 5,
        MIN_UNITS_FOR_UPGRADE: 10,
        MIN_UNITS_FOR_DISTRIBUTION: 10,
        MAX_DISTANCE_FOR_ATTACK: 5,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 30,
        MIN_UNITS_FOR_ATTACK: 10,
        MIN_UNITS_FOR_UPGRADE: 20,
        MIN_UNITS_FOR_DISTRIBUTION: 0,
        MAX_DISTANCE_FOR_ATTACK: 100,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 65,
        MIN_UNITS_FOR_ATTACK: 20,
        MIN_UNITS_FOR_UPGRADE: 30,
        MIN_UNITS_FOR_DISTRIBUTION: 0,
        MAX_DISTANCE_FOR_ATTACK: 100,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 80,
        MIN_UNITS_FOR_ATTACK: 25,
        MIN_UNITS_FOR_UPGRADE: 50,
        MIN_UNITS_FOR_DISTRIBUTION: 0,
        MAX_DISTANCE_FOR_ATTACK: 100,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 165,
        MIN_UNITS_FOR_ATTACK: 60,
        MIN_UNITS_FOR_UPGRADE: 80,
        MIN_UNITS_FOR_DISTRIBUTION: 0,
        MAX_DISTANCE_FOR_ATTACK: 100,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 240,
        MIN_UNITS_FOR_ATTACK: 80,
        MIN_UNITS_FOR_UPGRADE: 150,
        MIN_UNITS_FOR_DISTRIBUTION: 0,
        MAX_DISTANCE_FOR_ATTACK: 100,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 316,
        MIN_UNITS_FOR_ATTACK: 100,
        MIN_UNITS_FOR_UPGRADE: 200,
        MIN_UNITS_FOR_DISTRIBUTION: 0,
        MAX_DISTANCE_FOR_ATTACK: 100,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 388,
        MIN_UNITS_FOR_ATTACK: 120,
        MIN_UNITS_FOR_UPGRADE: 200,
        MIN_UNITS_FOR_DISTRIBUTION: 0,
        MAX_DISTANCE_FOR_ATTACK: 100,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 456,
        MIN_UNITS_FOR_ATTACK: 150,
        MIN_UNITS_FOR_UPGRADE: 300,
        MIN_UNITS_FOR_DISTRIBUTION: 0,
        MAX_DISTANCE_FOR_ATTACK: 100,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 520,
        MIN_UNITS_FOR_ATTACK: 275,
        MIN_UNITS_FOR_UPGRADE: 500,
        MIN_UNITS_FOR_DISTRIBUTION: 0,
        MAX_DISTANCE_FOR_ATTACK: 100,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 440,
        MIN_UNITS_FOR_ATTACK: 300,
        MIN_UNITS_FOR_UPGRADE: 600,
        MIN_UNITS_FOR_DISTRIBUTION: 0,
        MAX_DISTANCE_FOR_ATTACK: 100,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 380,
        MIN_UNITS_FOR_ATTACK: 350,
        MIN_UNITS_FOR_UPGRADE: 650,
        MIN_UNITS_FOR_DISTRIBUTION: 0,
        MAX_DISTANCE_FOR_ATTACK: 100,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 300,
        MIN_UNITS_FOR_ATTACK: 375,
        MIN_UNITS_FOR_UPGRADE: 700,
        MIN_UNITS_FOR_DISTRIBUTION: 0,
        MAX_DISTANCE_FOR_ATTACK: 100,
    },
    // Level: n
    Requirement{
        MIN_UNITS_AFTER_ATTACK: 500,
        MIN_UNITS_FOR_ATTACK: 500,
        MIN_UNITS_FOR_UPGRADE: 800,
        MIN_UNITS_FOR_DISTRIBUTION: 0,
        MAX_DISTANCE_FOR_ATTACK: 100,
    },
];

struct Requirement {
    MIN_UNITS_AFTER_ATTACK: u32,
    MIN_UNITS_FOR_ATTACK: u32,
    MIN_UNITS_FOR_UPGRADE: u32,
    MIN_UNITS_FOR_DISTRIBUTION: u32,
    MAX_DISTANCE_FOR_ATTACK: u32,
}

