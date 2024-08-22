use std::{
    cell::RefCell,
    fmt::{self, Debug},
    rc::Rc,
};

use crate::{env::Env, expr::Expression};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    /// !
    Not,
    /// !!
    NotNot,
    /// ~
    Tilde,
    /// %
    Percent,
    /// &
    And,
    /// &&
    AndAnd,
    /// *
    Mult,
    /// **
    Square,
    /// (
    LeftParen,
    /// )
    RightParen,
    /// -
    Minus,
    /// --
    Decr,
    /// ->
    Arrow,
    /// =>
    ArrowBig,
    /// _
    Underscore,
    /// +
    Plus,
    /// ++
    Increment,
    /// =
    Assign,
    /// ==
    Eq,
    /// !=
    NotEq,
    /// +=
    PlusEq,
    /// -=
    MinEq,
    /// *=
    MultEq,
    /// /=
    DivEq,
    /// {
    LeftBrace,
    /// }
    RightBrace,
    /// [
    LeftBracket,
    /// ]
    RightBracket,
    /// ;
    Semi,
    /// :
    Colon,
    /// ::
    DblColon,
    /// char
    CharLit,
    /// string
    StringLit,
    /// number
    NumberLit,
    /// true
    TrueLit,
    /// false
    FalseLit,
    /// null
    NullLit,
    /// array
    #[allow(dead_code)]
    ArrayLit,
    /// <
    Less,
    /// <=
    LessOrEq,
    /// >
    Greater,
    /// >=
    GreaterOrEq,
    /// ,
    Comma,
    /// .
    Dot,
    /// ..
    DotDot,
    /// /
    Divide,
    /// \
    Escape,
    /// \{
    StartParse,
    /// \}
    EndParse,
    /// ?
    Queston,
    /// |
    Pipe,
    /// ||
    Or,
    /// identifier
    Ident,
    /// end of file
    Eof,
    /// variable (let)
    Let,
    /// if
    If,
    /// else
    Else,
    /// else if
    ElseIf,
    /// return
    Return,
    /// while
    While,
    /// loop
    Loop,
    /// break
    Break,
    /// match
    Match,
    /// mod
    Mod,
    /// use
    Use,
    /// as
    As,
    /// from
    From,
    /// struct
    Struct,
    /// self
    Slf,
    /// impl
    Impl,
    /// enum
    Enum,
    /// async
    Async,
    /// await
    Await,
    /// pub
    Pub,
    /// mut
    Mut,
    /// function
    Func,
    /// number
    NumberIdent,
    /// string
    StringIdent,
    /// char
    CharIdent,
    /// bool
    BoolIdent,
    /// null
    NullIdent,
    /// void
    VoidIdent,
    /// array
    ArrayIdent,
    /// any
    AnyIdent,
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum FuncValueType {
    Func(FuncImpl),
    Std,
    Callback,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Base {
    Binary = 2,
    Octal = 8,
    Decimal = 10,
    Hexadecimal = 16,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralType {
    Number(f32),
    String(String),
    Char(char),
    Boolean(bool),
    Null,
    Void,
    Any,
    Array(Vec<Expression>),
    Func(FuncValueType),
    DeclrFunc(DeclrFuncType),
}

#[derive(Debug, Clone)]
pub struct DeclrFuncType {
    pub name: String,
    pub arity: usize,
    pub func: Rc<dyn FuncValType>,
}

pub trait FuncValType {
    fn call(&self, args: Vec<LiteralType>) -> LiteralType;
}

impl Debug for dyn FuncValType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FuncValType")
    }
}

impl PartialEq for DeclrFuncType {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.arity == other.arity && self.func.rc_eq(&other.func)
    }
}

pub trait RcFuncValType {
    fn rc_eq(&self, other: &Self) -> bool;
}

impl RcFuncValType for Rc<dyn FuncValType> {
    fn rc_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(self, other)
    }
}

pub struct Wrapper(pub Box<dyn Fn(&[LiteralType]) -> LiteralType>);

impl FuncValType for Wrapper {
    fn call(&self, args: Vec<LiteralType>) -> LiteralType {
        (self.0)(&args)
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum LiteralKind {
    Number { base: Base, value: f32 },
    String { value: String },
    Char { value: char },
    Bool { value: bool },
    Null,
}

#[derive(Clone, PartialEq, Debug)]
pub struct FuncImpl {
    pub name: String,
    pub value_type: Token,
    pub body: FuncBody,
    pub params: Vec<(Token, Token)>,
    pub is_async: bool,
    pub is_pub: bool,
    pub is_impl: bool,
    pub is_mut: bool,
    pub env: Rc<RefCell<Env>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token: TokenType,
    pub lexeme: String,
    pub value: Option<LiteralKind>,
    pub line: usize,
    pub pos: (usize, usize),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum CallType {
    Func,
    Var,
    Struct,
    OpenStruct,
    Method,
    Enum,
    Array,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Expression {
        expr: Expression,
    },
    Block {
        stmts: Vec<Statement>,
    },
    Var {
        names: Vec<Token>,
        value_type: Token,
        value: Option<Expression>,
        is_mut: bool,
        is_pub: bool,
        pub_names: Vec<Token>,
        is_func: bool,
    },
    Func {
        name: Token,
        value_type: Token,
        body: FuncBody,
        params: Vec<(Token, Token)>,
        is_async: bool,
        is_pub: bool,
        // if function is method (implemented)
        is_impl: bool,
        // if function contains `self` parameter
        is_mut: bool,
    },
    If {
        cond: Expression,
        body: Vec<Statement>,
        else_if_branches: Vec<(Expression, Vec<Statement>)>,
        else_branch: Option<Vec<Statement>>,
    },
    Return {
        expr: Expression,
    },
    While {
        cond: Expression,
        body: Vec<Statement>,
    },
    Loop {
        iter: Option<usize>,
        body: Vec<Statement>,
    },
    Break {},
    Match {
        cond: Expression,
        cases: Vec<(Expression, FuncBody)>,
        def_case: FuncBody,
    },
    Mod {
        src: String,
    },
    Use {
        src: String,
        names: Vec<(Token, Option<Token>)>,
    },
    Struct {
        name: Token,
        structs: Vec<(Token, TokenType, bool)>,
        is_pub: bool,
        methods: Vec<(Expression, bool)>,
    },
    Impl {
        name: Token,
        body: Vec<Statement>,
    },
    Enum {
        name: Token,
        enums: Vec<Token>,
        is_pub: bool,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum FuncBody {
    Statements(Vec<Statement>),
    Expression(Box<Expression>),
}
