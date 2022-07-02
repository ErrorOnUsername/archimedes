use crate::token::NumericConstant;
use crate::types::TypeID;

/// An operator that only has one operand.
pub enum UnaryOperator {
    PreIncrement,
    PostIncrement,
    PreDecrement,
    PostDecrement,
    ForceUnwrap,
    MakeOptional,
    LogicalNot,
    BitwiseNot,
    TypeCast(TypeID)
}

/// An operator that has two operands.
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    LogicalAnd,
    LogicalOr,
    LogicalXOR,
    BitwiseAnd,
    BitwiseOR,
    BitwiseXOR,
    BitwiseLeftShift,
    BitwiseRightShift
}

/// A declaration of a variable. Either decalred with
/// `let` or implicitly as a parameter to a funciton.
pub struct ParsedVarDecl {
    pub is_mutable: bool,
    pub is_reference: bool,
    pub type_id: TypeID,
    pub name: String,
}

/// A declaration of a procedure, otherwise known as a
/// function. These could either be methods of a struct
/// simple functions on their own.
pub struct ParsedProcDecl {
    pub name: String,
    pub is_macro: bool,
    pub parameters: Vec<ParsedVarDecl>,
    pub return_type_id: TypeID,
    pub body: ParsedBlock,
}

/// A call to a procedure
pub struct ParsedProcCall {
    pub signature: String,
    pub is_macro: bool,
    pub passed_parameters: Vec<ParsedVarDecl>,
}

/// The end bounds of a range expression. This indicates
/// whether or not a range's outer bounds are included or
/// excluded from the range itself.
pub enum RangeExprBound {
    Inclusive,
    Exclusive
}

/// The body of a `match` expression. It can either
/// consist of a single expression, or a block that
/// has a return expression of the same type.
pub enum MatchExprBody {
    Expr(ParsedExpression),
    Block(ParsedBlock)
}

/// The case of a `match` expression that is to be matched
/// against.
pub enum MatchExprCase {
    Expr(ParsedExpression, MatchExprBody),
    EnumVariant(String, MatchExprBody),
    All(MatchExprBody)
}

/// An abstract representation of an expression, which is
/// any sequence of operators, operands, or data in
/// general that are used to express some meaningful value.
pub enum ParsedExpression {
    Bool(bool),
    NumericConstant(NumericConstant),
    StringLiteral(String),
    CharLiteral(String),
    Var(String, Vec<String>),
    Range(RangeExprBound, Box<ParsedExpression>, Box<ParsedExpression>, RangeExprBound),
    Match(Box<ParsedExpression>, Vec<MatchExprCase>),
    UnaryOperation(Box<ParsedExpression>, UnaryOperator),
    BinaryOperation(Box<ParsedExpression>, BinaryOperator, Box<ParsedExpression>),
    ProcCall(Box<ParsedExpression>, ParsedProcCall),
    OptionalSome(Box<ParsedExpression>),
    OptionalNone
}

/// A statement that is to be acted upon, typically
/// contingent upon some sort of expression.
pub enum ParsedStatement {
    Expr(ParsedExpression),
    VarDecl(ParsedVarDecl, ParsedExpression),
    If(ParsedExpression, ParsedBlock, Option<Box<ParsedStatement>>),
    Block(ParsedBlock),
    ForLoop(ParsedExpression, ParsedBlock),
    WhileLoop(ParsedExpression, ParsedBlock),
    Continue,
    Break,
    Return(ParsedExpression),
}

pub struct ParsedImport {
    pub current_module_path: String,
    pub path: String,
}

pub struct ParsedStructDecl {
    pub name: String,
    pub data_members: Vec<ParsedVarDecl>,
    pub methods: Vec<ParsedProcDecl>,
}

pub enum ParsedEnumVariant {
    Untyped(String),
    UnlabeledTypes(String, Vec<TypeID>),
    LabeledTypes(String, Vec<ParsedVarDecl>)
}

pub struct ParsedEnumDecl {
    pub name: String,
    pub variants: Vec<ParsedEnumVariant>,
}

/// A block of statements, denoted by matching `{` and `}`.
/// These create a new lexical scope and describe the lifetime
/// of data.
pub struct ParsedBlock {
    pub stmts: Vec<ParsedStatement>
}

impl ParsedBlock {
    pub fn new() -> Self {
        Self { stmts: Vec::new() }
    }
}

/// A module, which is really just a file.
pub struct ParsedModule {
    pub name: String,
    pub imports: Vec<ParsedImport>,
    pub structs: Vec<ParsedStructDecl>,
    pub enums: Vec<ParsedEnumDecl>,
    pub procs: Vec<ParsedProcDecl>
}

impl ParsedModule {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            imports: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            procs: Vec::new()
        }
    }
}
