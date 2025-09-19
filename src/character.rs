#![allow(unused)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(i32)]
pub enum Character {
    Null = 00,
    StartOfHeader = 01,
    StartOfText = 02,
    EndOfText = 03,
    EndOfTransmission = 04,
    Enquiry = 05,
    Acknowledge = 06,
    Bell = 07,
    Backspace = 08,
    HorizontalTab = 09,
    LineFeed = 10,
    VerticalTab = 11,
    FormFeed = 12,
    CarriageReturn = 13,
    ShiftOut = 14,
    ShiftIn = 15,
    DataLinkEscape = 16,
    DeviceControl1 = 17,
    DeviceControl2 = 18,
    DeviceControl3 = 19,
    DeviceControl4 = 20,
    NegativeAcknowledge = 21,
    Synchronize = 22,
    EndOfTransmissionBlock = 23,
    Cancel = 24,
    EndOfMedium = 25,
    Substitute = 26,
    Escape = 27,
    FileSeparator = 28,
    GroupSeparator = 29,
    RecordSeparator = 30,
    UnitSeparator = 31,
    Space = 32,
    Exclamation = 33,
    DoubleQuote = 34,
    Octothorpe = 35,
    Dollar = 36,
    Percent = 37,
    Ampersand = 38,
    Quote = 39,
    ParenthesisLeft = 40,
    ParenthesisRight = 41,
    Asterisk = 42,
    Plus = 43,
    Comma = 44,
    Hyphen = 45,
    Period = 46,
    Solidus = 47,
    Digit0 = 48,
    Digit1 = 49,
    Digit2 = 50,
    Digit3 = 51,
    Digit4 = 52,
    Digit5 = 53,
    Digit6 = 54,
    Digit7 = 55,
    Digit8 = 56,
    Digit9 = 57,
    Colon = 58,
    Semicolon = 59,
    AngleLeft = 60,
    Equal = 61,
    AngleRight = 62,
    Question = 63,
    At = 64,
    UpperA = 65,
    UpperB = 66,
    UpperC = 67,
    UpperD = 68,
    UpperE = 69,
    UpperF = 70,
    UpperG = 71,
    UpperH = 72,
    UpperI = 73,
    UpperJ = 74,
    UpperK = 75,
    UpperL = 76,
    UpperM = 77,
    UpperN = 78,
    UpperO = 79,
    UpperP = 80,
    UpperQ = 81,
    UpperR = 82,
    UpperS = 83,
    UpperT = 84,
    UpperU = 85,
    UpperV = 86,
    UpperW = 87,
    UpperX = 88,
    UpperY = 89,
    UpperZ = 90,
    BracketLeft = 91,
    Backslash = 92,
    BracketRight = 93,
    Caret = 94,
    Underscore = 95,
    Grave = 96,
    LowerA = 97,
    LowerB = 98,
    LowerC = 99,
    LowerD = 100,
    LowerE = 101,
    LowerF = 102,
    LowerG = 103,
    LowerH = 104,
    LowerI = 105,
    LowerJ = 106,
    LowerK = 107,
    LowerL = 108,
    LowerM = 109,
    LowerN = 110,
    LowerO = 111,
    LowerP = 112,
    LowerQ = 113,
    LowerR = 114,
    LowerS = 115,
    LowerT = 116,
    LowerU = 117,
    LowerV = 118,
    LowerW = 119,
    LowerX = 120,
    LowerY = 121,
    LowerZ = 122,
    BraceLeft = 123,
    Bar = 124,
    BraceRight = 125,
    Tilde = 126,
    Delete = 127,
}
impl Character {
    pub fn from(input: i32) -> Self {
        match (input) {
            00 => Character::Null,
            01 => Character::StartOfHeader,
            02 => Character::StartOfText,
            03 => Character::EndOfText,
            04 => Character::EndOfTransmission,
            05 => Character::Enquiry,
            06 => Character::Acknowledge,
            07 => Character::Bell,
            08 => Character::Backspace,
            09 => Character::HorizontalTab,
            10 => Character::LineFeed,
            11 => Character::VerticalTab,
            12 => Character::FormFeed,
            13 => Character::CarriageReturn,
            14 => Character::ShiftOut,
            15 => Character::ShiftIn,
            16 => Character::DataLinkEscape,
            17 => Character::DeviceControl1,
            18 => Character::DeviceControl2,
            19 => Character::DeviceControl3,
            20 => Character::DeviceControl4,
            21 => Character::NegativeAcknowledge,
            22 => Character::Synchronize,
            23 => Character::EndOfTransmissionBlock,
            24 => Character::Cancel,
            25 => Character::EndOfMedium,
            26 => Character::Substitute,
            27 => Character::Escape,
            28 => Character::FileSeparator,
            29 => Character::GroupSeparator,
            30 => Character::RecordSeparator,
            31 => Character::UnitSeparator,
            32 => Character::Space,
            33 => Character::Exclamation,
            34 => Character::DoubleQuote,
            35 => Character::Octothorpe,
            36 => Character::Dollar,
            37 => Character::Percent,
            38 => Character::Ampersand,
            39 => Character::Quote,
            40 => Character::ParenthesisLeft,
            41 => Character::ParenthesisRight,
            42 => Character::Asterisk,
            43 => Character::Plus,
            44 => Character::Comma,
            45 => Character::Hyphen,
            46 => Character::Period,
            47 => Character::Solidus,
            48 => Character::Digit0,
            49 => Character::Digit1,
            50 => Character::Digit2,
            51 => Character::Digit3,
            52 => Character::Digit4,
            53 => Character::Digit5,
            54 => Character::Digit6,
            55 => Character::Digit7,
            56 => Character::Digit8,
            57 => Character::Digit9,
            58 => Character::Colon,
            59 => Character::Semicolon,
            60 => Character::AngleLeft,
            61 => Character::Equal,
            62 => Character::AngleRight,
            63 => Character::Question,
            64 => Character::At,
            65 => Character::UpperA,
            66 => Character::UpperB,
            67 => Character::UpperC,
            68 => Character::UpperD,
            69 => Character::UpperE,
            70 => Character::UpperF,
            71 => Character::UpperG,
            72 => Character::UpperH,
            73 => Character::UpperI,
            74 => Character::UpperJ,
            75 => Character::UpperK,
            76 => Character::UpperL,
            77 => Character::UpperM,
            78 => Character::UpperN,
            79 => Character::UpperO,
            80 => Character::UpperP,
            81 => Character::UpperQ,
            82 => Character::UpperR,
            83 => Character::UpperS,
            84 => Character::UpperT,
            85 => Character::UpperU,
            86 => Character::UpperV,
            87 => Character::UpperW,
            88 => Character::UpperX,
            89 => Character::UpperY,
            90 => Character::UpperZ,
            91 => Character::BracketLeft,
            92 => Character::Backslash,
            93 => Character::BracketRight,
            94 => Character::Caret,
            95 => Character::Underscore,
            96 => Character::Grave,
            97 => Character::LowerA,
            98 => Character::LowerB,
            99 => Character::LowerC,
            100 => Character::LowerD,
            101 => Character::LowerE,
            102 => Character::LowerF,
            103 => Character::LowerG,
            104 => Character::LowerH,
            105 => Character::LowerI,
            106 => Character::LowerJ,
            107 => Character::LowerK,
            108 => Character::LowerL,
            109 => Character::LowerM,
            110 => Character::LowerN,
            111 => Character::LowerO,
            112 => Character::LowerP,
            113 => Character::LowerQ,
            114 => Character::LowerR,
            115 => Character::LowerS,
            116 => Character::LowerT,
            117 => Character::LowerU,
            118 => Character::LowerV,
            119 => Character::LowerW,
            120 => Character::LowerX,
            121 => Character::LowerY,
            122 => Character::LowerZ,
            123 => Character::BraceLeft,
            124 => Character::Bar,
            125 => Character::BraceRight,
            126 => Character::Tilde,
            127 => Character::Delete,
            _ => Character::Null,
        }
    }
}
pub const CHARACTER_HT: i32 = Character::HorizontalTab as i32;
pub const CHARACTER_LF: i32 = Character::LineFeed as i32;
pub const CHARACTER_VT: i32 = Character::VerticalTab as i32;
pub const CHARACTER_FF: i32 = Character::FormFeed as i32;
pub const CHARACTER_CR: i32 = Character::CarriageReturn as i32;
pub const CHARACTER_SPACE: i32 = Character::Space as i32;
pub const CHARACTER_EXCLAMATION: i32 = Character::Exclamation as i32;
pub const CHARACTER_DOUBLEQUOTE: i32 = Character::DoubleQuote as i32;
pub const CHARACTER_DOLLAR: i32 = Character::Dollar as i32;
pub const CHARACTER_PERCENT: i32 = Character::Percent as i32;
pub const CHARACTER_QUOTE: i32 = Character::Quote as i32;
pub const CHARACTER_PARENTHESIS_LEFT: i32 = Character::ParenthesisLeft as i32;
pub const CHARACTER_PARENTHESIS_RIGHT: i32 = Character::ParenthesisRight as i32;
pub const CHARACTER_COMMA: i32 = Character::Comma as i32;
pub const CHARACTER_HYPHEN: i32 = Character::Hyphen as i32;
pub const CHARACTER_PERIOD: i32 = Character::Period as i32;
pub const CHARACTER_SOLIDUS: i32 = Character::Solidus as i32;
pub const CHARACTER_0: i32 = Character::Digit0 as i32;
pub const CHARACTER_1: i32 = Character::Digit1 as i32;
pub const CHARACTER_2: i32 = Character::Digit2 as i32;
pub const CHARACTER_3: i32 = Character::Digit3 as i32;
pub const CHARACTER_4: i32 = Character::Digit4 as i32;
pub const CHARACTER_5: i32 = Character::Digit5 as i32;
pub const CHARACTER_6: i32 = Character::Digit6 as i32;
pub const CHARACTER_7: i32 = Character::Digit7 as i32;
pub const CHARACTER_8: i32 = Character::Digit8 as i32;
pub const CHARACTER_9: i32 = Character::Digit9 as i32;
pub const CHARACTER_COLON: i32 = Character::Colon as i32;
pub const CHARACTER_SEMICOLON: i32 = Character::Semicolon as i32;
pub const CHARACTER_ANGLE_LEFT: i32 = Character::AngleLeft as i32;
pub const CHARACTER_EQUAL: i32 = Character::Equal as i32;
pub const CHARACTER_ANGLE_RIGHT: i32 = Character::AngleRight as i32;
pub const CHARACTER_UPPER_A: i32 = Character::UpperA  as i32;
pub const CHARACTER_UPPER_B: i32 = Character::UpperB  as i32;
pub const CHARACTER_UPPER_C: i32 = Character::UpperC  as i32;
pub const CHARACTER_UPPER_D: i32 = Character::UpperD  as i32;
pub const CHARACTER_UPPER_E: i32 = Character::UpperE  as i32;
pub const CHARACTER_UPPER_F: i32 = Character::UpperF  as i32;
pub const CHARACTER_UPPER_G: i32 = Character::UpperG  as i32;
pub const CHARACTER_UPPER_H: i32 = Character::UpperH  as i32;
pub const CHARACTER_UPPER_I: i32 = Character::UpperI  as i32;
pub const CHARACTER_UPPER_J: i32 = Character::UpperJ  as i32;
pub const CHARACTER_UPPER_K: i32 = Character::UpperK  as i32;
pub const CHARACTER_UPPER_L: i32 = Character::UpperL  as i32;
pub const CHARACTER_UPPER_M: i32 = Character::UpperM  as i32;
pub const CHARACTER_UPPER_N: i32 = Character::UpperN  as i32;
pub const CHARACTER_UPPER_O: i32 = Character::UpperO  as i32;
pub const CHARACTER_UPPER_P: i32 = Character::UpperP  as i32;
pub const CHARACTER_UPPER_Q: i32 = Character::UpperQ  as i32;
pub const CHARACTER_UPPER_R: i32 = Character::UpperR  as i32;
pub const CHARACTER_UPPER_S: i32 = Character::UpperS  as i32;
pub const CHARACTER_UPPER_T: i32 = Character::UpperT  as i32;
pub const CHARACTER_UPPER_U: i32 = Character::UpperU  as i32;
pub const CHARACTER_UPPER_V: i32 = Character::UpperV  as i32;
pub const CHARACTER_UPPER_W: i32 = Character::UpperW  as i32;
pub const CHARACTER_UPPER_X: i32 = Character::UpperX  as i32;
pub const CHARACTER_UPPER_Y: i32 = Character::UpperY  as i32;
pub const CHARACTER_UPPER_Z: i32 = Character::UpperZ  as i32;
pub const CHARACTER_BRACKET_LEFT: i32 = Character::BracketLeft as i32;
pub const CHARACTER_BACKSLASH: i32 = Character::Backslash as i32;
pub const CHARACTER_BRACKET_RIGHT: i32 = Character::BracketRight as i32;
pub const CHARACTER_CARET: i32 = Character::Caret as i32;
pub const CHARACTER_UNDERSCORE: i32 = Character::Underscore as i32;
pub const CHARACTER_GRAVE: i32 = Character::Grave as i32;
pub const CHARACTER_LOWER_A: i32 =Character::LowerA as i32;
pub const CHARACTER_LOWER_B: i32 =Character::LowerB as i32;
pub const CHARACTER_LOWER_C: i32 =Character::LowerC as i32;
pub const CHARACTER_LOWER_D: i32 = Character::LowerD as i32;
pub const CHARACTER_LOWER_E: i32 = Character::LowerE as i32;
pub const CHARACTER_LOWER_F: i32 = Character::LowerF as i32;
pub const CHARACTER_LOWER_G: i32 = Character::LowerG as i32;
pub const CHARACTER_LOWER_H: i32 = Character::LowerH as i32;
pub const CHARACTER_LOWER_I: i32 = Character::LowerI as i32;
pub const CHARACTER_LOWER_J: i32 = Character::LowerJ as i32;
pub const CHARACTER_LOWER_K: i32 = Character::LowerK as i32;
pub const CHARACTER_LOWER_L: i32 = Character::LowerL as i32;
pub const CHARACTER_LOWER_M: i32 = Character::LowerM as i32;
pub const CHARACTER_LOWER_N: i32 = Character::LowerN as i32;
pub const CHARACTER_LOWER_O: i32 = Character::LowerO as i32;
pub const CHARACTER_LOWER_P: i32 = Character::LowerP as i32;
pub const CHARACTER_LOWER_Q: i32 = Character::LowerQ as i32;
pub const CHARACTER_LOWER_R: i32 = Character::LowerR as i32;
pub const CHARACTER_LOWER_S: i32 = Character::LowerS as i32;
pub const CHARACTER_LOWER_T: i32 = Character::LowerT as i32;
pub const CHARACTER_LOWER_U: i32 = Character::LowerU as i32;
pub const CHARACTER_LOWER_V: i32 = Character::LowerV as i32;
pub const CHARACTER_LOWER_W: i32 = Character::LowerW as i32;
pub const CHARACTER_LOWER_X: i32 = Character::LowerX as i32;
pub const CHARACTER_LOWER_Z: i32 = Character::LowerZ as i32;
pub const CHARACTER_BRACE_LEFT: i32 = Character::BraceLeft as i32;
pub const CHARACTER_TILDE: i32 = Character::Tilde as i32;
pub unsafe fn get_hexadecimal_digit_value(ch: i32) -> i32 {
    match ch {
        CHARACTER_0 => 0,
        CHARACTER_1 => 1,
        CHARACTER_2 => 2,
        CHARACTER_3 => 3,
        CHARACTER_4 => 4,
        CHARACTER_5 => 5,
        CHARACTER_6 => 6,
        CHARACTER_7 => 7,
        CHARACTER_8 => 8,
        CHARACTER_9 => 9,
        CHARACTER_UPPER_A | CHARACTER_LOWER_A => 10,
        CHARACTER_UPPER_B | CHARACTER_LOWER_B => 11,
        CHARACTER_UPPER_C | CHARACTER_LOWER_C => 12,
        CHARACTER_UPPER_D | CHARACTER_LOWER_D => 13,
        CHARACTER_UPPER_E | CHARACTER_LOWER_E => 14,
        CHARACTER_UPPER_F | CHARACTER_LOWER_F => 15,
        _ => 0,
    }
}
pub const CHARACTER_TYPE_NONE: u8 = 0x00;
pub const CHARACTER_TYPE_IDENTIFIER: u8 = 0x01;
pub const CHARACTER_TYPE_DIGIT_DECIMAL: u8 = 0x02;
pub const CHARACTER_TYPE_PRINTABLE: u8 = 0x04;
pub const CHARACTER_TYPE_WHITESPACE: u8 = 0x08;
pub const CHARACTER_TYPE_DIGIT_HEXADECIMAL: u8 = 0x10;
pub fn get_character_type(ch: i32) -> u8 {
    return CHARACTER_TYPE[ch as usize];
}
pub fn is_whitespace(ch: i32) -> bool {
    return get_character_type(ch) & CHARACTER_TYPE_WHITESPACE != 0;
}
pub fn is_alphanumeric(ch: i32) -> bool {
    return get_character_type(ch) & (CHARACTER_TYPE_IDENTIFIER | CHARACTER_TYPE_DIGIT_DECIMAL) != 0;
}
pub fn is_printable(ch: i32) -> bool {
    return get_character_type(ch) & CHARACTER_TYPE_PRINTABLE != 0;
}
pub fn is_identifier(ch: i32) -> bool {
    return get_character_type(ch) & CHARACTER_TYPE_IDENTIFIER != 0;
}
pub fn is_digit_hexadecimal(ch: i32) -> bool {
    return get_character_type(ch) & CHARACTER_TYPE_DIGIT_HEXADECIMAL != 0;
}
pub fn is_digit_decimal(ch: i32) -> bool {
    return get_character_type(ch) & CHARACTER_TYPE_DIGIT_DECIMAL != 0;
}
const CHARACTER_TYPE: [u8; 257] = [
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_WHITESPACE,
    CHARACTER_TYPE_WHITESPACE,
    CHARACTER_TYPE_WHITESPACE,
    CHARACTER_TYPE_WHITESPACE,
    CHARACTER_TYPE_WHITESPACE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_WHITESPACE | CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_DIGIT_DECIMAL,
    CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_DIGIT_DECIMAL,
    CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_DIGIT_DECIMAL,
    CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_DIGIT_DECIMAL,
    CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_DIGIT_DECIMAL,
    CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_DIGIT_DECIMAL,
    CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_DIGIT_DECIMAL,
    CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_DIGIT_DECIMAL,
    CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_DIGIT_DECIMAL,
    CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_DIGIT_DECIMAL,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_PRINTABLE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
    CHARACTER_TYPE_NONE,
];
pub unsafe fn luao_utf8esc(buffer: *mut i8, mut x: usize) -> i32 {
    unsafe {
        let mut n: i32 = 1;
        if x < 0x80 {
            *buffer.offset((8 - 1) as isize) = x as i8;
        } else {
            let mut mfb: usize = 0x3f;
            loop {
                let fresh9 = n;
                n += 1;
                *buffer.offset((8 - fresh9) as isize) = (0x80 as usize | x & 0x3f as usize) as i8;
                x >>= 6;
                mfb >>= 1;
                if !(x > mfb) {
                    break;
                }
            }
            *buffer.offset((8 - n) as isize) = ((!mfb << 1) as usize | x) as i8;
        }
        return n;
    }
}
pub fn is_digit(ch: i32) -> bool {
    match ch {
        CHARACTER_0 | CHARACTER_1 | CHARACTER_2 | CHARACTER_3 | CHARACTER_4 | CHARACTER_5 | CHARACTER_6 | CHARACTER_7 | CHARACTER_8 | CHARACTER_9 => true,
        _ => false,
    }
}
