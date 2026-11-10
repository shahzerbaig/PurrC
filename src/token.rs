use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Class,
    Const,
    Variable,
    Number,
    Let,
    Function,
    True,
    False,
    If,
    Else,
    Return,
    Ident,

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Slash,

    // Delimiters
    Comma,
    Semicolon,
    Colon,

    // Special tokens
    Illegal,
    EOF,
}

pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: impl Into<String>) -> Self {
        Self {
            kind,
            literal: literal.into(),
        }
    }
}

// Display funtion to show the type.
impl fmt::Display for TokenKind {
    fn fmt(&self, formater: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TokenKind::Class => "Class",
            TokenKind::Const => "Const",
            TokenKind::Variable => "Variable",
            TokenKind::Number => "Number",
            TokenKind::Let => "Let",
            TokenKind::Function => "Function",
            TokenKind::True => "True",
            TokenKind::False => "False",
            TokenKind::If => "If",
            TokenKind::Else => "Else",
            TokenKind::Return => "Return",
            TokenKind::Ident => "Ident",

            //Operators
            TokenKind::Assign => "Assign",
            TokenKind::Plus => "Plus",
            TokenKind::Minus => "Minus",
            TokenKind::Bang => "Bang",
            TokenKind::Slash => "Slash",

            //Delimeters
            TokenKind::Comma => "Comma",
            TokenKind::Semicolon => "Semicolon",
            TokenKind::Colon => "Colon",

            //Special Tokens
            TokenKind::Illegal => "Illegal",
            TokenKind::EOF => "EOF",
        };
        formater.write_str(s)
    }
}
// The lookup_ident() function distinguishes reserved keywords from normal identifiers. For example, let becomes TokenKind::Let, while age becomes TokenKind::Ident

pub fn lookup_ident(ident: &str) -> TokenKind {
    match ident {
        // ── Structure ──────────────────────────────
        "clowder" => TokenKind::Class, // a group of cats = a class
        // "colony" => TokenKind::Module, // feral cat colony = module/namespace
        // "litter" => TokenKind::Block,  // a litter = block/scope

        // ── Values & Bindings ──────────────────────
        "queen" => TokenKind::Const,   // queen = fixed/immutable
        "tom" => TokenKind::Variable,  // tom = mutable binding
        "kitten" => TokenKind::Number, // kitten = small literal
        // "meow" => TokenKind::String,   // meow = vocalization = string
        "purr" => TokenKind::True,  // purr = contentment = true
        "hiss" => TokenKind::False, // hiss = displeasure = false

        // ── Declaration ────────────────────────────
        "let" => TokenKind::Let,
        "fn" => TokenKind::Function,
        // "adopt" => TokenKind::Import, // adopt a cat = import
        // "groom" => TokenKind::Use,    // grooming = bringing in

        // ── Control Flow ───────────────────────────
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        // "pounce" => TokenKind::While,     // repeated pouncing = loop
        // "stalk" => TokenKind::For,        // stalking = iterate
        // "swat" => TokenKind::Break,       // swat = knock out of loop
        // "retract" => TokenKind::Continue, // retract claws = continue

        // ── Functions / Return ─────────────────────
        "return" => TokenKind::Return,
        // "yowl" => TokenKind::Throw,  // loud yowl = error/throw
        // "chirp" => TokenKind::Yield, // chirping = yield a value

        // ── Misc ───────────────────────────────────
        // "knead" => TokenKind::Await,      // kneading = waiting/working
        // "catnap" => TokenKind::Async,     // short sleep = async
        // "nine_lives" => TokenKind::Try,   // nine lives = fault tolerance
        // "territory" => TokenKind::Public, // marked territory = public
        // "den" => TokenKind::Private,      // hidden den = private
        _ => TokenKind::Ident,
    }
}

pub fn lookup_operator(ch: char) -> Option<TokenKind> {
    let kind = match ch {
        '=' => TokenKind::Assign,
        '+' => TokenKind::Plus,
        '-' => TokenKind::Minus,
        '!' => TokenKind::Bang,
        '/' => TokenKind::Slash,
        ',' => TokenKind::Comma,
        ';' => TokenKind::Semicolon,
        ':' => TokenKind::Colon,
        _ => return None,
    };
    Some(kind)
}
