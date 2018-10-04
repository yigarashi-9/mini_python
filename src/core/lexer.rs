use std::iter::Peekable;
use std::str::Chars;
use token::Token;

pub struct LexingError {
    line: usize,
    row: usize,
    msg: String,
}

impl LexingError {
    fn new(line: usize, row: usize, msg: String) -> LexingError {
        LexingError { line: line, row: row, msg: msg }
    }

    pub fn to_string(self) -> String {
        format!("Lexing Error: line {}, row {}, {}", self.line, self.row, self.msg)
    }
}

fn symbol_to_token(ch: char) -> Token {
    match ch {
        '+' => Token::Plus,
        '<' => Token::Lt,
        '(' => Token::LParen,
        ')' => Token::RParen,
        '[' => Token::LBracket,
        ']' => Token::RBracket,
        '{' => Token::LBrace,
        '}' => Token::RBrace,
        ':' => Token::Colon,
        ',' => Token::Comma,
        '.' => Token::Dot,
        _   => panic!("Invalid symbol"),
    }
}

fn ident_to_token(s: String) -> Token {
    match &s[..] {
        "None" => Token::None,
        "True" => Token::True,
        "False" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "while" => Token::While,
        "for" => Token::For,
        "in" => Token::In,
        "break" => Token::Break,
        "continue" => Token::Continue,
        "def" => Token::Def,
        "return" => Token::Return,
        "assert" => Token::Assert,
        "class" => Token::Class,
        _ => Token::Ident(s),
    }
}

fn is_number(ch: char) -> bool {
    match ch {
        '0' ... '9' => true,
        _ => false
    }
}

fn is_alphabet(ch: char) -> bool {
    match ch {
        '_' | 'a' ... 'z' | 'A' ... 'Z' => true,
        _ => false
    }
}

fn is_alphanumeric(ch: char) -> bool {
    is_number(ch) || is_alphabet(ch)
}

fn is_whitespace(ch: char) -> bool {
    ch == ' '
}

fn is_not_quote(ch: char) -> bool {
    ch != '\''
}

fn is_not_dquote(ch: char) -> bool {
    ch != '"'
}

struct Lexer<'a> {
    it: Peekable<Chars<'a>>,
    line: usize,
    row: usize,
    stack: Vec<usize>,
    is_line_head: bool,
    tokens: Vec<Token>
}

impl <'a>Lexer<'a> {
    fn new(s: &'a String) -> Lexer<'a> {
        Lexer { it: s.chars().peekable(), line: 1, row: 1, stack: vec![0],
                is_line_head: true, tokens: vec![] }
    }

    fn next(&mut self) -> Option<char> {
        match self.it.next() {
            Some('\n') => {
                self.line += 1;
                self.row = 0;
                Some('\n')
            },
            Some(c) => {
                self.row += 1;
                Some(c)
            },
            None => None,
        }
    }

    fn calc_indent(&mut self, indent_level: usize) -> Result<(), LexingError> {
        let mut last_indent_level = *(self.stack.last().unwrap());
        if indent_level > last_indent_level {
            self.stack.push(indent_level);
            self.tokens.push(Token::Indent);
        } else if indent_level < last_indent_level {
            loop {
                self.stack.pop();
                self.tokens.push(Token::Dedent);
                last_indent_level = *(self.stack.last().unwrap());
                if indent_level == last_indent_level {
                    break;
                } else if indent_level > last_indent_level {
                    return Err(self.error("Invalid indentation".to_string()))
                }
            }
        };
        Ok(())
    }

    fn consume(&mut self, c1: char) -> Option<char> {
        match self.next() {
            Some(c2) if c1 == c2 => {
                Some(c1)
            },
            _ => None,
        }
    }

    fn consume_while<X>(&mut self, f: X) -> Vec<char>
    where X: Fn(char) -> bool {
        let mut v: Vec<char> = vec![];
        while let Some(&ch) = self.it.peek() {
            if f(ch) {
                self.next(); v.push(ch)
            } else {
                break;
            }
        }
        v
    }
    fn error(&self, s: String) -> LexingError {
        LexingError::new(self.line, self.row, s)
    }
}

pub fn tokenize(s: String) -> Result<Vec<Token>, LexingError> {
    let mut lexer = Lexer::new(&s);
    loop {
        // consume blank lines
        if lexer.is_line_head {
            let indent_level = lexer.consume_while(is_whitespace).len();
            match lexer.it.peek() {
                Some('\n') => {
                    lexer.next();
                    continue
                },
                Some(_) => {
                    try!(lexer.calc_indent(indent_level));
                    lexer.is_line_head = false;
                },
                _ => break,
            }
        };

        let mut ch = '0';
        match lexer.it.peek() {
            Some(&ch_) => { ch = ch_ },
            None => break
        };

        match ch {
            '0' ... '9' => {
                let num: String = lexer.consume_while(is_number).into_iter().collect();
                lexer.tokens.push(Token::Int(num.parse::<i32>().unwrap()));
            },
            '\'' => {
                lexer.next();
                let s: String = lexer.consume_while(is_not_quote).into_iter().collect();
                lexer.tokens.push(Token::Str(s));
                try!(lexer.consume('\'').ok_or(lexer.error("\' expected".to_string())));
            },
            '"' => {
                lexer.next();
                let s: String = lexer.consume_while(is_not_dquote).into_iter().collect();
                lexer.tokens.push(Token::Str(s));
                try!(lexer.consume('"').ok_or(lexer.error("\" expected".to_string())));
            },
            '+' | '<' | '(' | ')' | '[' | ']' | '{' | '}' | ':' | ',' | '.' => {
                let nch = lexer.next().unwrap();
                lexer.tokens.push(symbol_to_token(nch))
            },
            '=' => {
                lexer.next();
                if *lexer.it.peek().unwrap() != '=' {
                    lexer.tokens.push(Token::Eq)
                } else {
                    lexer.next();
                    lexer.tokens.push(Token::EqEq)
                }
            },
            '\n' => {
                lexer.next();
                lexer.tokens.push(Token::NewLine);
                lexer.is_line_head = true;
            }
            ch if is_alphabet(ch) => {
                let nch = lexer.next().unwrap();
                let mut id_vec = lexer.consume_while(is_alphanumeric);
                id_vec.insert(0, nch);
                lexer.tokens.push(ident_to_token(id_vec.into_iter().collect()));
            },
            ch if is_whitespace(ch) => {
                lexer.consume_while(is_whitespace);
            }
            _ => return Err(lexer.error(format!("Invalid character {} used", ch)))
        }
    };

    loop {
        match lexer.stack.pop() {
            Some(i) if i != 0 => lexer.tokens.push(Token::Dedent),
            _ => break,
        }
    }

    lexer.tokens.push(Token::EOF);
    Ok(lexer.tokens)
}

pub fn print_tokens(tokens: &Vec<Token>) {
    for t in tokens {
        println!("{:?}", t);
    }
}
