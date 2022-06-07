pub struct Span {
    pub file_id: u64,
    pub start: usize,
    pub end: usize,
}

pub enum NumericConstant {
    Integer(i128),
    FloatingPoint(f64),
}

pub enum Token {
    Trash(Span),

    EOL(Span), // End-of-line
    EOF(Span), // End-of-file

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
    Hash(Span),            // #
    Bang(Span),            // !
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

    Star(Span),               // *
    StarAssign(Span),         // *=
    ForwardSlash(Span),       // /
    ForwardSlashAssign(Span), // /=
    Percent(Span),            // %
    PercentAssign(Span),      // %=

    KeywordImport(Span),
    KeywordDecl(Span),
    KeywordLet(Span),
    KeywordMut(Span),

    KeywordStruct(Span),
    KetwordEnum(Span),

    KeywordMatch(Span),
    KeywordIf(Span),
    KeywordElse(Span),
    KeywordFor(Span),
    KeywordWhile(Span),
    KeywordContinue(Span),
    KeywordBreak(Span),
    KeywordReturn(Span),

    TypeLiteralNothing(Span),
    TypeLiteralBool(Span),
    TypeLiteralChar(Span),
    TypeLiteralU8(Span),
    TypeLiteralI8(Span),
    TypeLiteralU16(Span),
    TypeLiteralI16(Span),
    TypeLiteralU32(Span),
    TypeLiteralI32(Span),
    TypeLiteralU64(Span),
    TypeLiteralI64(Span),
    TypeLiteralF32(Span),
    TypeLiteralF64(Span),
}
