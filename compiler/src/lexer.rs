// Lexer for Prizm Language

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    If,
    Else,
    ElseIf,
    Loop,
    LoopUntil,
    Repeat,
    For,
    Define,
    Function,
    Return,
    Break,
    Var,
    Output,  // Built-in function
    Print,   // Built-in function
    
    // Type Keywords
    Int,
    Float,
    String,
    Boolean,
    Array,
    Object,
    
    // Boolean Literals
    True,
    False,
    
    // Headers
    File,
    Math,
    Http,
    Ui,
    Root,
    Data,
    Time,
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Colon,
    
    // Literals
    Identifier(String),
    Number(i64),
    Float(f64),
    String(String),
    
    // Special
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            self.skip_whitespace();
            
            if self.position >= self.input.len() {
                break;
            }

            match self.current_char() {
                '(' => {
                    tokens.push(Token::LeftParen);
                    self.position += 1;
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    self.position += 1;
                }
                '{' => {
                    tokens.push(Token::LeftBrace);
                    self.position += 1;
                }
                '}' => {
                    tokens.push(Token::RightBrace);
                    self.position += 1;
                }
                '[' => {
                    tokens.push(Token::LeftBracket);
                    self.position += 1;
                }
                ']' => {
                    tokens.push(Token::RightBracket);
                    self.position += 1;
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.position += 1;
                }
                '.' => {
                    tokens.push(Token::Dot);
                    self.position += 1;
                }
                ':' => {
                    tokens.push(Token::Colon);
                    self.position += 1;
                }
                '+' => {
                    tokens.push(Token::Plus);
                    self.position += 1;
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.position += 1;
                }
                '*' => {
                    tokens.push(Token::Star);
                    self.position += 1;
                }
                '/' => {
                    tokens.push(Token::Slash);
                    self.position += 1;
                }
                '%' => {
                    tokens.push(Token::Percent);
                    self.position += 1;
                }
                '=' => {
                    self.position += 1;
                    if self.position < self.input.len() && self.current_char() == '=' {
                        tokens.push(Token::EqualEqual);
                        self.position += 1;
                    } else {
                        tokens.push(Token::Equal);
                    }
                }
                '!' => {
                    self.position += 1;
                    if self.position < self.input.len() && self.current_char() == '=' {
                        tokens.push(Token::NotEqual);
                        self.position += 1;
                    } else {
                        tokens.push(Token::Not);
                    }
                }
                '<' => {
                    self.position += 1;
                    if self.position < self.input.len() && self.current_char() == '=' {
                        tokens.push(Token::LessEqual);
                        self.position += 1;
                    } else {
                        tokens.push(Token::Less);
                    }
                }
                '>' => {
                    self.position += 1;
                    if self.position < self.input.len() && self.current_char() == '=' {
                        tokens.push(Token::GreaterEqual);
                        self.position += 1;
                    } else {
                        tokens.push(Token::Greater);
                    }
                }
                '"' => {
                    self.position += 1;
                    let string = self.read_string();
                    tokens.push(Token::String(string));
                }
                _ if self.current_char().is_digit(10) => {
                    let number = self.read_number();
                    tokens.push(number);
                }
                _ if self.current_char().is_alphabetic() || self.current_char() == '_' => {
                    let identifier = self.read_identifier();
                    let token = self.keyword_or_identifier(&identifier);
                    tokens.push(token);
                }
                _ => {
                    self.position += 1;
                }
            }
        }

        tokens.push(Token::Eof);
        tokens
    }

    fn current_char(&self) -> char {
        self.input[self.position]
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while self.position < self.input.len() && 
              (self.input[self.position].is_alphanumeric() || self.input[self.position] == '_') {
            identifier.push(self.input[self.position]);
            self.position += 1;
        }
        identifier
    }

    fn read_number(&mut self) -> Token {
        let mut number_str = String::new();
        let mut is_float = false;

        while self.position < self.input.len() && 
              (self.input[self.position].is_digit(10) || self.input[self.position] == '.') {
            if self.input[self.position] == '.' {
                is_float = true;
            }
            number_str.push(self.input[self.position]);
            self.position += 1;
        }

        if is_float {
            Token::Float(number_str.parse().unwrap_or(0.0))
        } else {
            Token::Number(number_str.parse().unwrap_or(0))
        }
    }

    fn read_string(&mut self) -> String {
        let mut string = String::new();
        while self.position < self.input.len() && self.input[self.position] != '"' {
            string.push(self.input[self.position]);
            self.position += 1;
        }
        if self.position < self.input.len() {
            self.position += 1; // Skip closing quote
        }
        string
    }

    fn keyword_or_identifier(&self, word: &str) -> Token {
        match word {
            "if" => Token::If,
            "else" => Token::Else,
            "loop" => Token::Loop,
            "until" => Token::LoopUntil,
            "repeat" => Token::Repeat,
            "for" => Token::For,
            "define" => Token::Define,
            "function" => Token::Function,
            "return" => Token::Return,
            "break" => Token::Break,
            "var" => Token::Var,
            "output" => Token::Output,
            "print" => Token::Print,
            "int" => Token::Int,
            "float" => Token::Float,
            "string" => Token::String,
            "boolean" | "bool" => Token::Boolean,
            "array" => Token::Array,
            "object" => Token::Object,
            "true" => Token::True,
            "false" => Token::False,
            "file" => Token::File,
            "math" => Token::Math,
            "http" => Token::Http,
            "ui" => Token::Ui,
            "root" => Token::Root,
            "data" => Token::Data,
            "time" => Token::Time,
            _ => Token::Identifier(word.to_string()),
        }
    }
}