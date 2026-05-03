//! Tokens C99 - Generado automáticamente
#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    KwAuto,
    KwBreak,
    KwCase,
    KwChar,
    KwConst,
    KwContinue,
    KwDefault,
    KwDo,
    KwDouble,
    KwElse,
    KwEnum,
    KwExtern,
    KwFloat,
    KwFor,
    KwGoto,
    KwIf,
    KwInline,
    KwInt,
    KwLong,
    KwRegister,
    KwRestrict,
    KwReturn,
    KwShort,
    KwSigned,
    KwSizeof,
    KwStatic,
    KwStruct,
    KwSwitch,
    KwTypedef,
    KwUnion,
    KwUnsigned,
    KwVoid,
    KwVolatile,
    KwWhile,
    KwBool,
    KwComplex,
    KwImaginary,

    // Operators & Punctuation
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    Bang,
    Eq,
    Lt,
    Gt,
    Question,
    Colon,
    Dot,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    PlusPlus,
    MinusMinus,
    Arrow,
    LtLt,
    GtGt,
    LtEq,
    GtEq,
    EqEq,
    BangEq,
    AmpAmp,
    PipePipe,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    AmpEq,
    PipeEq,
    CaretEq,
    LtLtEq,
    GtGtEq,
    Ellipsis,

    // Literals
    IntLit(i64),
    FloatLit(f64),
    CharLit(char),
    StringLit(String),
    
    // Identifier
    Ident(String),
    
    // Special
    Eof,
    Newline,
    Whitespace,
    Comment,
    
    // Preprocessor
    Hash,
    HashHash,
    Include,
    Define,
    Ifdef,
    Ifndef,
    Endif,
    Undef,
    Pragma,
    Error,
    Line,
}

impl Token {
    pub fn is_keyword(&self) -> bool {
        matches!(self, Token::KwAuto | Token::KwBreak | Token::KwCase | Token::KwChar | Token::KwConst | Token::KwContinue | Token::KwDefault | Token::KwDo | Token::KwDouble | Token::KwElse | Token::KwEnum | Token::KwExtern | Token::KwFloat | Token::KwFor | Token::KwGoto | Token::KwIf | Token::KwInline | Token::KwInt | Token::KwLong | Token::KwRegister | Token::KwRestrict | Token::KwReturn | Token::KwShort | Token::KwSigned | Token::KwSizeof | Token::KwStatic | Token::KwStruct | Token::KwSwitch | Token::KwTypedef | Token::KwUnion | Token::KwUnsigned | Token::KwVoid | Token::KwVolatile | Token::KwWhile | Token::KwBool | Token::KwComplex | Token::KwImaginary)
    }

    pub fn is_literal(&self) -> bool {
        matches!(self, Token::IntLit(_) | Token::FloatLit(_) | Token::CharLit(_) | Token::StringLit(_))
    }
    
    pub fn is_operator(&self) -> bool {
        !self.is_keyword() && !self.is_literal() && !matches!(self, Token::Ident(_) | Token::Eof)
    }
}
