pub const LENGTH: i32 = 13;
pub const MIN_LENGTH: i32 = 4;
pub const MAX_LENGTH: i32 = 32;

// [Letters,Digits, Symbols]
const CHAR: [str] = [
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
    "1234567890",
    "*;<>()[]{}#@$?!^~|_-",
];

struct UnitDict {
    ending: str,
    num: f32,
    plural: bool,
}

pub const UNITS_DICT: [UnitDict; 17] = [
    UnitDict {
        ending: "second",
        num: 1,
        plural: false,
    },
    UnitDict {
        ending: "minute",
        num: 60,
        plural: false,
    },
    UnitDict {
        ending: "hour",
        num: 60,
        plural: false,
    },
    UnitDict {
        ending: "day",
        num: 24,
        plural: false,
    },
    UnitDict {
        ending: "month",
        num: 30.5,
        plural: false,
    },
    UnitDict {
        ending: "year",
        num: 12,
        plural: false,
    },
    UnitDict {
        ending: "thousand years",
        num: 1000,
        plural: true,
    },
    UnitDict {
        ending: "million years",
        num: 1000,
        plural: true,
    },
    UnitDict {
        ending: "billion years",
        num: 1000,
        plural: true,
    },
    UnitDict {
        ending: "trillion years",
        num: 1000,
        plural: true,
    },
    UnitDict {
        ending: "quadrillion years",
        num: 1000,
        plural: true,
    },
    UnitDict {
        ending: "quintillion years",
        num: 1000,
        plural: true,
    },
    UnitDict {
        ending: "sextillion years",
        num: 1000,
        plural: true,
    },
    UnitDict {
        ending: "septillion years",
        num: 1000,
        plural: true,
    },
    UnitDict {
        ending: "octillion years",
        num: 1000,
        plural: true,
    },
    UnitDict {
        ending: "nonillion years",
        num: 1000,
        plural: true,
    },
    UnitDict {
        ending: "decillion years",
        num: 1000,
        plural: true,
    },
];
