#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ExpressionKind {
    Void = 0,
    Nil = 1,
    True = 2,
    False = 3,
    Constant = 4,
    ConstantNumber = 5,
    ConstantInteger = 6,
    ConstantString = 7,
    Nonrelocatable = 8,
    Local = 9,
    UpValue = 10,
    Constant2 = 11,
    Indexed = 12,
    IndexUpValue = 13,
    IndexInteger = 14,
    Field = 15,
    Jump = 16,
    Relocatable = 17,
    Call = 18,
    VariableArguments = 19,
    Global = 20,
    VarargVariable = 21,
    VarargIndex = 22,
}
impl ExpressionKind {
    pub fn is_index(&self) -> bool {
        match self {
            ExpressionKind::Indexed
            | ExpressionKind::VarargIndex
            | ExpressionKind::IndexUpValue
            | ExpressionKind::IndexInteger
            | ExpressionKind::Field => true,
            _ => false,
        }
    }
    pub fn is_index_plus(&self) -> bool {
        match self {
            ExpressionKind::Local
            | ExpressionKind::VarargVariable
            | ExpressionKind::UpValue
            | ExpressionKind::Constant2
            | ExpressionKind::Indexed
            | ExpressionKind::VarargIndex
            | ExpressionKind::IndexUpValue
            | ExpressionKind::IndexInteger
            | ExpressionKind::Field => true,
            _ => false,
        }
    }
}
