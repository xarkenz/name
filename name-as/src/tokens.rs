use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    #[regex(r"\.[a-zA-Z_]+")]
    Directive,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*:")]
    Label,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex(r"\$[a-zA-Z0-9]+")]
    Register,

    #[regex(r"-?0x[0-9a-fA-F]+")]
    HexNumber,

    #[regex(r"-?0b[01]+")]
    BinaryNumber,

    #[regex(r"-?0[0-7]+")]
    OctalNumber,

    #[regex(r"-?[0-9]+")]
    DecimalNumber,

    #[regex(r#""([^"\\]*(?:\\.[^"\\]*)*)""#)]
    DoubleQuote,

    #[token(":")]
    Colon,

    #[token(",", logos::skip)]
    Comma,

    #[token("(", logos::skip)]
    LParen,

    #[token(")", logos::skip)]
    RParen,

    #[regex(r"#.*", logos::skip)]
    Comment,
}