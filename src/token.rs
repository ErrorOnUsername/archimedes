#[derive(PartialEq, Debug)]
pub struct Span {
    pub file_id: u64,
    pub start: usize,
    pub end: usize,
}

#[derive(PartialEq, Debug)]
pub enum IntegerLiteralFormat {
    Binary,
    Octal,
    Decimal,
    Hexadecimal
}

#[derive(PartialEq, Debug)]
pub enum FloatingPointLiteralFormat {
    Standard,
    ENotation
}

#[derive(PartialEq, Debug)]
pub enum NumericConstant {
    Integer(String, IntegerLiteralFormat),
    FloatingPoint(String, FloatingPointLiteralFormat),
}

#[derive(PartialEq, Debug)]
pub enum PrimitiveType {
    Nothing,
    Bool,
    Char,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64
}

#[derive(PartialEq, Debug)]
pub enum Token {
    EOL(Span), // End-of-line
    EOF,       // End-of-file

    IdentName(Span, String),

    StringLiteral(Span, String), // ""
    CharLiteral(Span, u8),       // ''
    Number(Span, NumericConstant),

    LParen(Span),          // (
    RParen(Span),          // )
    LCurly(Span),          // {
    RCurly(Span),          // }
    LSquare(Span),         // [
    RSquare(Span),         // ]
    LAngle(Span),          // <
    RAngle(Span),          // >
    Assign(Span),          // =
    Colon(Span),           // :
    ColonAssign(Span),     // :=
    DoubleColon(Span),     // ::
    Semicolon(Span),       // ;
    ThinArrow(Span),       // ->
    ThiccArrow(Span),      // =>
    Dollar(Span),          // $
    Comma(Span),           // ,
    Dot(Span),             // .
    DotDot(Span),          // ..
    Hash(Span),            // #
    Bang(Span),            // !
    QuestionMark(Span),    // ?
    Tilde(Span),           // ~
    TildeAssign(Span),     // ~=
    Ampersand(Span),       // &
    AmpersandAssign(Span), // &=
    DoubleAmpersand(Span), // &&
    Pipe(Span),            // |
    PipeAssign(Span),      // |=
    DoublePipe(Span),      // ||
    Caret(Span),           // ^
    CaretAssign(Span),     // ^=
    DoubleCaret(Span),     // ^^

    RShift(Span),       // >>
    RShiftAssign(Span), // >>=
    LShift(Span),       // <<
    LShiftAssign(Span), // <<=
    LEQ(Span),          // <=
    GEQ(Span),          // >=
    EQ(Span),           // ==
    NEQ(Span),          // !=

    Minus(Span),         // -
    MinusMinus(Span),    // --
    MinusAssign(Span),   // -=
    Plus(Span),          // +
    PlusPlus(Span),      // ++
    PlusAssign(Span),    // +=
    Star(Span),          // *
    StarAssign(Span),    // *=
    Slash(Span),         // /
    SlashAssign(Span),   // /=
    Percent(Span),       // %
    PercentAssign(Span), // %=

    KeywordImport(Span),
    KeywordDecl(Span),
    KeywordLet(Span),
    KeywordMut(Span),

    KeywordStruct(Span),
    KeywordEnum(Span),

    KeywordMatch(Span),
    KeywordIf(Span),
    KeywordElse(Span),
    KeywordFor(Span),
    KeywordWhile(Span),
    KeywordContinue(Span),
    KeywordBreak(Span),
    KeywordReturn(Span),

    BuiltinType(Span, PrimitiveType),
}
