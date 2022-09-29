use crate::token::NumericConstant;

#[derive(Clone)]
pub enum ComplexType {
    Procedure,
    Struct,
    Enum,
    Constant,
}

#[derive(Debug, Clone)]
pub enum ParsedType {
    Name(Vec<String>, String),
    Array(Box<ParsedType>, ParsedExpression),
}

impl PartialEq for ParsedType {
    fn eq(&self, other: &Self) -> bool {
        match other {
            ParsedType::Name(module_path, name) => {
                if let ParsedType::Name(other_module_path, other_name) = other {
                    return (module_path == other_module_path) && (name == other_name);
                }
                todo!("Implement arrays");
            },
            ParsedType::Array(_elem_type, _size) => {
                todo!("Implement arrays");
            }
        }
    }
}

/// An operator that only has one operand.
#[derive(Debug, Clone)]
pub enum UnaryOperator {
    PreIncrement,
    PostIncrement,
    PreDecrement,
    PostDecrement,
    LogicalNot,
    BitwiseNot,
    AddressOf,
    Dereference,
    TypeCast(Box<ParsedType>),
}

/// An operator that has two operands.
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Invalid,

    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    LogicalAnd,
    LogicalOr,
    LogicalXOR,

    NEQ,
    EQ,
    LT,
    GT,
    LEQ,
    GEQ,

    BitwiseAnd,
    BitwiseOr,
    BitwiseXOR,
    BitwiseLeftShift,
    BitwiseRightShift,

    Assign,

    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,

    AndAssign,
    OrAssign,
    XORAssign,

    LeftShiftAssign,
    RightShiftAssign,
}

impl BinaryOperator {
    pub fn is_assignment(&self) -> bool {
        match self {
            BinaryOperator::Assign          |
            BinaryOperator::AddAssign       |
            BinaryOperator::SubtractAssign  |
            BinaryOperator::MultiplyAssign  |
            BinaryOperator::DivideAssign    |
            BinaryOperator::ModuloAssign    |
            BinaryOperator::AndAssign       |
            BinaryOperator::OrAssign        |
            BinaryOperator::XORAssign       |
            BinaryOperator::LeftShiftAssign |
            BinaryOperator::RightShiftAssign => true,
            _ => false
        }
    }
}

/// A declaration of a variable. Either decalred with
/// `let` or implicitly as a parameter to a funciton.
#[derive(Debug, Clone)]
pub struct ParsedVarDecl {
    pub parsed_type: ParsedType,
    pub name: String,
    pub default_value: ParsedExpression,
}

/// A declaration of a procedure, otherwise known as a
/// function. These could either be methods of a struct
/// simple functions on their own.
#[derive(Debug, Clone)]
pub struct ParsedProcDecl {
    pub name: String,
    pub parameters: Vec<ParsedVarDecl>,
    pub parsed_return_type: ParsedType,
    pub body: ParsedBlock,
}

/// A call to a procedure
#[derive(Debug, Clone)]
pub struct ParsedProcCall {
    pub name: String,
    pub passed_parameters: Vec<ParsedVarDecl>,
}

/// The end bounds of a range expression. This indicates
/// whether or not a range's outer bounds are included or
/// excluded from the range itself.
#[derive(Debug, Clone)]
pub enum RangeExprBound {
    Inclusive,
    Exclusive,
}

/// The body of a `match` expression. It can either
/// consist of a single expression, or a block that
/// has a return expression of the same type.
#[derive(Debug, Clone)]
pub enum MatchExprBody {
    Expr(ParsedExpression),
    Block(ParsedBlock),
}

/// The case of a `match` expression that is to be matched
/// against.
#[derive(Debug, Clone)]
pub enum MatchExprCase {
    Expr(ParsedExpression, MatchExprBody),
    EnumVariant(String, MatchExprBody),
    Fallback(MatchExprBody),
}

/// An abstract representation of an expression, which is
/// any sequence of operators, operands, or data in
/// general that are used to express some meaningful value.
#[derive(Debug, Clone)]
pub enum ParsedExpression {
    Bool(bool),
    NumericConstant(NumericConstant),
    StringLiteral(String),
    CharLiteral(u8),
    Var(String),
    NamespacedVar(Vec<String>, String),
    Range(RangeExprBound, Box<ParsedExpression>, Box<ParsedExpression>, RangeExprBound),
    Match(Box<ParsedExpression>, Vec<MatchExprCase>),
    Operator(BinaryOperator),
    UnaryOperation(Box<ParsedExpression>, UnaryOperator),
    BinaryOperation(Box<ParsedExpression>, BinaryOperator, Box<ParsedExpression>),
    ProcCall(ParsedProcCall),
    Invalid,
}

impl ParsedExpression {
    pub fn priority(&self) -> i32 {
        match self {
            ParsedExpression::Operator(op) => {
                match op {
                    BinaryOperator::Invalid => -1,

                    BinaryOperator::Multiply |
                    BinaryOperator::Divide   |
                    BinaryOperator::Modulo => 12,

                    BinaryOperator::Add |
                    BinaryOperator::Subtract => 11,

                    BinaryOperator::BitwiseLeftShift |
                    BinaryOperator::BitwiseRightShift => 10,

                    BinaryOperator::LT  |
                    BinaryOperator::GT  |
                    BinaryOperator::LEQ |
                    BinaryOperator::GEQ => 9,

                    BinaryOperator::NEQ |
                    BinaryOperator::EQ => 8,

                    BinaryOperator::BitwiseAnd => 7,
                    BinaryOperator::BitwiseXOR => 6,
                    BinaryOperator::BitwiseOr => 5,

                    BinaryOperator::LogicalAnd => 4,
                    BinaryOperator::LogicalXOR => 3,
                    BinaryOperator::LogicalOr => 2,

                    BinaryOperator::Assign          |
                    BinaryOperator::AddAssign       |
                    BinaryOperator::SubtractAssign  |
                    BinaryOperator::MultiplyAssign  |
                    BinaryOperator::DivideAssign    |
                    BinaryOperator::ModuloAssign    |
                    BinaryOperator::AndAssign       |
                    BinaryOperator::OrAssign        |
                    BinaryOperator::XORAssign       |
                    BinaryOperator::LeftShiftAssign |
                    BinaryOperator::RightShiftAssign => 1,
                }
            }
            _ => -1
        }
    }

    pub fn is_assignable(&self) -> bool {
        match self {
            ParsedExpression::Var(_)              |
            ParsedExpression::NamespacedVar(_, _) |
            ParsedExpression::UnaryOperation(_, _) => true,
            _ => false
        }
    }
}

/// A statement that is to be acted upon, typically
/// contingent upon some sort of expression.
#[derive(Debug, Clone)]
pub enum ParsedStatement {
    Expr(ParsedExpression),
    VarDecl(ParsedVarDecl),
    VarAssign(String, BinaryOperator, ParsedExpression),
    If(ParsedExpression, ParsedBlock, Option<Box<ParsedStatement>>),
    Block(ParsedBlock),
    ForLoop(ParsedVarDecl, ParsedExpression, ParsedBlock),
    WhileLoop(ParsedExpression, ParsedBlock),
    InfiniteLoop(ParsedBlock),
    Continue,
    Break,
    Return(ParsedExpression),
}

#[derive(Debug, Clone)]
pub struct ParsedImport {
    pub current_module_path: String,
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct ParsedStructDecl {
    pub name: String,
    pub data_members: Vec<ParsedVarDecl>,
}

#[derive(Debug, Clone)]
pub enum ParsedEnumVariant {
    Untyped(String),
    UnlabeledTypes(String, Vec<ParsedType>),
    LabeledTypes(String, Vec<ParsedVarDecl>),
}

#[derive(Debug, Clone)]
pub struct ParsedEnumDecl {
    pub name: String,
    pub variants: Vec<ParsedEnumVariant>,
}

/// A block of statements, denoted by matching `{` and `}`.
/// These create a new lexical scope and describe the lifetime
/// of data.
#[derive(Debug, Clone)]
pub struct ParsedBlock {
    pub stmts: Vec<ParsedStatement>,
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
    pub procs: Vec<ParsedProcDecl>,
}

impl ParsedModule {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            imports: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            procs: Vec::new(),
        }
    }
}
