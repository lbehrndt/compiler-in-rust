/// Represents the kind of token in the lexer.
pub enum TokenKind {
    /// A numeric token.
    Number(i64),
    /// Addition operator.
    Plus,
    /// Subtraction operator.
    Minus,
    /// Multiplication operator.
    Asterisk,
    /// Division operator.
    Slash,
    /// Left parenthesis.
    LeftParenthesis,
    /// Right parenthesis.
    RightParenthesis,
    /// Represents a bad token.
    Bad,
    /// End of file token.
    Eof,
}

/// Represents a span of text in the source file.
pub struct TextSpan {
    /// The start position of the text span.
    start: usize,
    /// The end position of the text span.
    end: usize,
    /// The literal text covered by the text span.
    literal: String,
}

/// Represents a syntax token in the lexer.
pub struct SyntaxToken<'a> {
    /// The kind of token.
    _kind: TokenKind,
    /// The position of the token in the source file.
    _position: usize,
    /// The text representation of the token.
    _text: &'a str,
}

impl<'a> SyntaxToken<'a> {
    /// Creates a new instance of SyntaxToken.
    pub fn new(kind: TokenKind, position: usize, text: &'a str) -> Self {
        Self {
            _kind: kind,
            _position: position,
            _text: text,
        }
    }
}

/// Transforms an input string into a sequence of tokens.
pub struct Lexer<'a> {
    /// The input text to be lexed.
    _text: &'a str,
    /// The current position of the lexer.
    _position: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new instance of Lexer.
    pub fn new(text: &'a str) -> Self {
        Self {
            _text: text,
            _position: 0,
        }
    }
}
