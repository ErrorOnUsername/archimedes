pub struct ArchimedesSpan {
    pub file_id: u64,
    pub start: usize,
    pub end: usize,
}

pub enum Token {
    Trash(ArchimedesSpan),

    EOL(ArchimedesSpan), // End-of-line
    EOF(ArchimedesSpan), // End-of-file

    IdentName(ArchimedesSpan, String),

    StringLiteral(ArchimedesSpan, String), // ""
    CharLiteral(ArchimedesSpan, u8),       // ''

    LParen(ArchimedesSpan),          // (
    RParen(ArchimedesSpan),          // )
    LCurly(ArchimedesSpan),          // {
    RCurly(ArchimedesSpan),          // }
    LSquare(ArchimedesSpan),         // [
    RSquare(ArchimedesSpan),         // ]
    LAngle(ArchimedesSpan),          // <
    RAngle(ArchimedesSpan),          // >
    Assign(ArchimedesSpan),          // =
    Colon(ArchimedesSpan),           // :
    ColonAssign(ArchimedesSpan),     // :=
    DoubleColon(ArchimedesSpan),     // ::
    Semicolon(ArchimedesSpan),       // ;
    ThinArrow(ArchimedesSpan),       // ->
    ThiccArrow(ArchimedesSpan),      // =>
    Hash(ArchimedesSpan),            // #
    Bang(ArchimedesSpan),            // !
    Ampersand(ArchimedesSpan),       // &
    DoubleAmpersand(ArchimedesSpan), // &&
    Pipe(ArchimedesSpan),            // |
    DoublePipe(ArchimedesSpan),      // ||
    Caret(ArchimedesSpan),           // ^
    DoubleCaret(ArchimedesSpan),     // ^^

    RShift(ArchimedesSpan),       // >>
    RShiftAssign(ArchimedesSpan), // >>=
    LShift(ArchimedesSpan),       // <<
    LShiftAssign(ArchimedesSpan), // <<=
    LEQ(ArchimedesSpan),          // <=
    GEQ(ArchimedesSpan),          // >=
    EQ(ArchimedesSpan),           // ==
    NEQ(ArchimedesSpan),          // !=

    KeywordImport(ArchimedesSpan),
    KeywordDecl(ArchimedesSpan),
    KeywordLet(ArchimedesSpan),
    KeywordMut(ArchimedesSpan),

    KeywordStruct(ArchimedesSpan),
    KetwordEnum(ArchimedesSpan),

    KeywordMatch(ArchimedesSpan),
    KeywordIf(ArchimedesSpan),
    KeywordElse(ArchimedesSpan),
    KeywordFor(ArchimedesSpan),
    KeywordWhile(ArchimedesSpan),
    KeywordContinue(ArchimedesSpan),
    KeywordBreak(ArchimedesSpan),

    TypeLiteralNothing(ArchimedesSpan),
    TypeLiteralBool(ArchimedesSpan),
    TypeLiteralChar(ArchimedesSpan),
    TypeLiteralU8(ArchimedesSpan),
    TypeLiteralI8(ArchimedesSpan),
    TypeLiteralU16(ArchimedesSpan),
    TypeLiteralI16(ArchimedesSpan),
    TypeLiteralU32(ArchimedesSpan),
    TypeLiteralI32(ArchimedesSpan),
    TypeLiteralU64(ArchimedesSpan),
    TypeLiteralI64(ArchimedesSpan),
    TypeLiteralF32(ArchimedesSpan),
    TypeLiteralF64(ArchimedesSpan),
}
