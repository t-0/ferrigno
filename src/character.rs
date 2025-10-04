const CHARACTER_TYPE_NONE: u8 = 0x00;
const CHARACTER_TYPE_IDENTIFIER: u8 = 0x01;
const CHARACTER_TYPE_DIGIT_DECIMAL: u8 = 0x02;
const CHARACTER_TYPE_PRINTABLE: u8 = 0x04;
const CHARACTER_TYPE_WHITESPACE: u8 = 0x08;
const CHARACTER_TYPE_DIGIT_HEXADECIMAL: u8 = 0x10;
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
    pub fn is_alpha(&self) -> bool {
        self.is_lower() || self.is_upper()
    }
    pub fn is_digit(&self) -> bool {
        match self {
            | Character::Digit0
            | Character::Digit1
            | Character::Digit2
            | Character::Digit3
            | Character::Digit4
            | Character::Digit5
            | Character::Digit6
            | Character::Digit7
            | Character::Digit8
            | Character::Digit9 => true,
            | _ => false,
        }
    }
    pub fn from(input: i32) -> Self {
        match input {
            | 00 => Character::Null,
            | 01 => Character::StartOfHeader,
            | 02 => Character::StartOfText,
            | 03 => Character::EndOfText,
            | 04 => Character::EndOfTransmission,
            | 05 => Character::Enquiry,
            | 06 => Character::Acknowledge,
            | 07 => Character::Bell,
            | 08 => Character::Backspace,
            | 09 => Character::HorizontalTab,
            | 10 => Character::LineFeed,
            | 11 => Character::VerticalTab,
            | 12 => Character::FormFeed,
            | 13 => Character::CarriageReturn,
            | 14 => Character::ShiftOut,
            | 15 => Character::ShiftIn,
            | 16 => Character::DataLinkEscape,
            | 17 => Character::DeviceControl1,
            | 18 => Character::DeviceControl2,
            | 19 => Character::DeviceControl3,
            | 20 => Character::DeviceControl4,
            | 21 => Character::NegativeAcknowledge,
            | 22 => Character::Synchronize,
            | 23 => Character::EndOfTransmissionBlock,
            | 24 => Character::Cancel,
            | 25 => Character::EndOfMedium,
            | 26 => Character::Substitute,
            | 27 => Character::Escape,
            | 28 => Character::FileSeparator,
            | 29 => Character::GroupSeparator,
            | 30 => Character::RecordSeparator,
            | 31 => Character::UnitSeparator,
            | 32 => Character::Space,
            | 33 => Character::Exclamation,
            | 34 => Character::DoubleQuote,
            | 35 => Character::Octothorpe,
            | 36 => Character::Dollar,
            | 37 => Character::Percent,
            | 38 => Character::Ampersand,
            | 39 => Character::Quote,
            | 40 => Character::ParenthesisLeft,
            | 41 => Character::ParenthesisRight,
            | 42 => Character::Asterisk,
            | 43 => Character::Plus,
            | 44 => Character::Comma,
            | 45 => Character::Hyphen,
            | 46 => Character::Period,
            | 47 => Character::Solidus,
            | 48 => Character::Digit0,
            | 49 => Character::Digit1,
            | 50 => Character::Digit2,
            | 51 => Character::Digit3,
            | 52 => Character::Digit4,
            | 53 => Character::Digit5,
            | 54 => Character::Digit6,
            | 55 => Character::Digit7,
            | 56 => Character::Digit8,
            | 57 => Character::Digit9,
            | 58 => Character::Colon,
            | 59 => Character::Semicolon,
            | 60 => Character::AngleLeft,
            | 61 => Character::Equal,
            | 62 => Character::AngleRight,
            | 63 => Character::Question,
            | 64 => Character::At,
            | 65 => Character::UpperA,
            | 66 => Character::UpperB,
            | 67 => Character::UpperC,
            | 68 => Character::UpperD,
            | 69 => Character::UpperE,
            | 70 => Character::UpperF,
            | 71 => Character::UpperG,
            | 72 => Character::UpperH,
            | 73 => Character::UpperI,
            | 74 => Character::UpperJ,
            | 75 => Character::UpperK,
            | 76 => Character::UpperL,
            | 77 => Character::UpperM,
            | 78 => Character::UpperN,
            | 79 => Character::UpperO,
            | 80 => Character::UpperP,
            | 81 => Character::UpperQ,
            | 82 => Character::UpperR,
            | 83 => Character::UpperS,
            | 84 => Character::UpperT,
            | 85 => Character::UpperU,
            | 86 => Character::UpperV,
            | 87 => Character::UpperW,
            | 88 => Character::UpperX,
            | 89 => Character::UpperY,
            | 90 => Character::UpperZ,
            | 91 => Character::BracketLeft,
            | 92 => Character::Backslash,
            | 93 => Character::BracketRight,
            | 94 => Character::Caret,
            | 95 => Character::Underscore,
            | 96 => Character::Grave,
            | 97 => Character::LowerA,
            | 98 => Character::LowerB,
            | 99 => Character::LowerC,
            | 100 => Character::LowerD,
            | 101 => Character::LowerE,
            | 102 => Character::LowerF,
            | 103 => Character::LowerG,
            | 104 => Character::LowerH,
            | 105 => Character::LowerI,
            | 106 => Character::LowerJ,
            | 107 => Character::LowerK,
            | 108 => Character::LowerL,
            | 109 => Character::LowerM,
            | 110 => Character::LowerN,
            | 111 => Character::LowerO,
            | 112 => Character::LowerP,
            | 113 => Character::LowerQ,
            | 114 => Character::LowerR,
            | 115 => Character::LowerS,
            | 116 => Character::LowerT,
            | 117 => Character::LowerU,
            | 118 => Character::LowerV,
            | 119 => Character::LowerW,
            | 120 => Character::LowerX,
            | 121 => Character::LowerY,
            | 122 => Character::LowerZ,
            | 123 => Character::BraceLeft,
            | 124 => Character::Bar,
            | 125 => Character::BraceRight,
            | 126 => Character::Tilde,
            | 127 => Character::Delete,
            | _ => Character::Null,
        }
    }
    pub fn from2(input: i32) -> Option<Character> {
        match input {
            | -1 => None,
            | _ => Some(Character::from(input)),
        }
    }
    pub fn get_hexadecimal_digit_value(&self) -> u8 {
        match self {
            | Character::Digit0 => 0,
            | Character::Digit1 => 1,
            | Character::Digit2 => 2,
            | Character::Digit3 => 3,
            | Character::Digit4 => 4,
            | Character::Digit5 => 5,
            | Character::Digit6 => 6,
            | Character::Digit7 => 7,
            | Character::Digit8 => 8,
            | Character::Digit9 => 9,
            | Character::UpperA | Character::LowerA => 10,
            | Character::UpperB | Character::LowerB => 11,
            | Character::UpperC | Character::LowerC => 12,
            | Character::UpperD | Character::LowerD => 13,
            | Character::UpperE | Character::LowerE => 14,
            | Character::UpperF | Character::LowerF => 15,
            | _ => 0,
        }
    }
    pub fn get_character_type(&self) -> u8 {
        match self {
            | Character::Null
            | Character::StartOfHeader
            | Character::StartOfText
            | Character::EndOfText
            | Character::EndOfTransmission
            | Character::Enquiry
            | Character::Acknowledge
            | Character::Bell
            | Character::Backspace
            | Character::ShiftOut
            | Character::ShiftIn
            | Character::DataLinkEscape
            | Character::DeviceControl1
            | Character::DeviceControl2
            | Character::DeviceControl3
            | Character::DeviceControl4
            | Character::NegativeAcknowledge
            | Character::Synchronize
            | Character::EndOfTransmissionBlock
            | Character::Cancel
            | Character::EndOfMedium
            | Character::Substitute
            | Character::Escape
            | Character::FileSeparator
            | Character::GroupSeparator
            | Character::RecordSeparator
            | Character::UnitSeparator
            | Character::Delete => CHARACTER_TYPE_NONE,
            | Character::HorizontalTab
            | Character::LineFeed
            | Character::VerticalTab
            | Character::FormFeed
            | Character::CarriageReturn => CHARACTER_TYPE_WHITESPACE,
            | Character::Space => CHARACTER_TYPE_WHITESPACE | CHARACTER_TYPE_PRINTABLE,
            | Character::Exclamation
            | Character::DoubleQuote
            | Character::Octothorpe
            | Character::Dollar
            | Character::Percent
            | Character::Ampersand
            | Character::Quote
            | Character::ParenthesisLeft
            | Character::ParenthesisRight
            | Character::Asterisk
            | Character::Plus
            | Character::Comma
            | Character::Hyphen
            | Character::Period
            | Character::Solidus
            | Character::Colon
            | Character::Semicolon
            | Character::AngleLeft
            | Character::Equal
            | Character::AngleRight
            | Character::Question
            | Character::At
            | Character::BracketLeft
            | Character::Backslash
            | Character::BracketRight
            | Character::Caret
            | Character::Grave
            | Character::BraceLeft
            | Character::Bar
            | Character::BraceRight
            | Character::Tilde => CHARACTER_TYPE_PRINTABLE,
            | Character::Digit0
            | Character::Digit1
            | Character::Digit2
            | Character::Digit3
            | Character::Digit4
            | Character::Digit5
            | Character::Digit6
            | Character::Digit7
            | Character::Digit8
            | Character::Digit9 => CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_DIGIT_DECIMAL,
            | Character::UpperA
            | Character::UpperB
            | Character::UpperC
            | Character::UpperD
            | Character::UpperE
            | Character::UpperF
            | Character::LowerA
            | Character::LowerB
            | Character::LowerC
            | Character::LowerD
            | Character::LowerE
            | Character::LowerF => CHARACTER_TYPE_DIGIT_HEXADECIMAL | CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
            | Character::UpperG
            | Character::UpperH
            | Character::UpperI
            | Character::UpperJ
            | Character::UpperK
            | Character::UpperL
            | Character::UpperM
            | Character::UpperN
            | Character::UpperO
            | Character::UpperP
            | Character::UpperQ
            | Character::UpperR
            | Character::UpperS
            | Character::UpperT
            | Character::UpperU
            | Character::UpperV
            | Character::UpperW
            | Character::UpperX
            | Character::UpperY
            | Character::UpperZ
            | Character::LowerG
            | Character::LowerH
            | Character::LowerI
            | Character::LowerJ
            | Character::LowerK
            | Character::LowerL
            | Character::LowerM
            | Character::LowerN
            | Character::LowerO
            | Character::LowerP
            | Character::LowerQ
            | Character::LowerR
            | Character::LowerS
            | Character::LowerT
            | Character::LowerU
            | Character::LowerV
            | Character::LowerW
            | Character::LowerX
            | Character::LowerY
            | Character::LowerZ => CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
            | Character::Underscore => CHARACTER_TYPE_PRINTABLE | CHARACTER_TYPE_IDENTIFIER,
        }
    }
    pub fn is_lower(&self) -> bool {
        match self {
            | Character::LowerA
            | Character::LowerB
            | Character::LowerC
            | Character::LowerD
            | Character::LowerE
            | Character::LowerF
            | Character::LowerG
            | Character::LowerH
            | Character::LowerI
            | Character::LowerJ
            | Character::LowerK
            | Character::LowerL
            | Character::LowerM
            | Character::LowerN
            | Character::LowerO
            | Character::LowerP
            | Character::LowerQ
            | Character::LowerR
            | Character::LowerS
            | Character::LowerT
            | Character::LowerU
            | Character::LowerV
            | Character::LowerW
            | Character::LowerX
            | Character::LowerY
            | Character::LowerZ => true,
            | _ => false,
        }
    }
    pub fn is_upper(&self) -> bool {
        match self {
            | Character::UpperA
            | Character::UpperB
            | Character::UpperC
            | Character::UpperD
            | Character::UpperE
            | Character::UpperF
            | Character::UpperG
            | Character::UpperH
            | Character::UpperI
            | Character::UpperJ
            | Character::UpperK
            | Character::UpperL
            | Character::UpperM
            | Character::UpperN
            | Character::UpperO
            | Character::UpperP
            | Character::UpperQ
            | Character::UpperR
            | Character::UpperS
            | Character::UpperT
            | Character::UpperU
            | Character::UpperV
            | Character::UpperW
            | Character::UpperX
            | Character::UpperY
            | Character::UpperZ => true,
            | _ => false,
        }
    }
    pub fn is_whitespace(&self) -> bool {
        return self.get_character_type() & CHARACTER_TYPE_WHITESPACE != 0;
    }
    pub fn is_alphanumeric(&self) -> bool {
        return self.get_character_type() & (CHARACTER_TYPE_IDENTIFIER | CHARACTER_TYPE_DIGIT_DECIMAL) != 0;
    }
    pub fn is_printable(&self) -> bool {
        return self.get_character_type() & CHARACTER_TYPE_PRINTABLE != 0;
    }
    pub fn is_identifier(&self) -> bool {
        return self.get_character_type() & CHARACTER_TYPE_IDENTIFIER != 0;
    }
    pub fn is_digit_hexadecimal(&self) -> bool {
        return self.get_character_type() & CHARACTER_TYPE_DIGIT_HEXADECIMAL != 0;
    }
    pub fn is_digit_decimal(&self) -> bool {
        return self.get_character_type() & CHARACTER_TYPE_DIGIT_DECIMAL != 0;
    }
}
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
