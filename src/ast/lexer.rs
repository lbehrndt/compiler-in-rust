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

    /// Gets the kind of token.
    pub fn kind(&self) -> &TokenKind {
        &self._kind
    }

    /// Gets the position of the token in the source file.
    pub fn position(&self) -> usize {
        self._position
    }

    /// Gets the text representation of the token.
    pub fn text(&self) -> &str {
        self._text
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

    /// Gets the current character at the lexer's position.
    fn current(&self) -> char {
        if self._position >= self._text.len() {
            return '\0';
        }
        return self._text.chars().nth(self._position).unwrap();
    }

    /// Moves the lexer to the next position and returns the new position.
    fn next(&mut self) -> usize {
        self._position += 1;
        self._position
    }

    /// Lexes the next token from the input text.
    pub fn next_token(&mut self) {}
}
