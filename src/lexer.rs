use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Int(i32),
    True,
    False,
    If,
    Else,
    Plus,
    Lt,
    Eq,
    LParen,
    RParen,
    Colon,
    NewLine,
    Indent,
    Dedent,
    EOF,
}

fn symbol_to_token(ch: char) -> Token {
    match ch {
        '+' => Token::Plus,
        '<' => Token::Lt,
        '=' => Token::Eq,
        '(' => Token::LParen,
        ')' => Token::RParen,
        ':' => Token::Colon,
        _   => panic!("Invalid symbol"),
    }
}

fn ident_to_token(s: String) -> Token {
    match &s[..] {
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
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

fn calc_indent(stack: &mut Vec<usize>, tokens: &mut Vec<Token>,
               indent_level: usize) -> () {
    let mut last_indent_level = *(stack.last_mut().unwrap());
    if indent_level > last_indent_level {
        stack.push(indent_level);
        tokens.push(Token::Indent);
    } else if indent_level < last_indent_level {
        loop {
            stack.pop();
            tokens.push(Token::Dedent);
            last_indent_level = *(stack.last_mut().unwrap());
            if indent_level == last_indent_level {
                break;
            } else if indent_level > last_indent_level {
                panic!("Invalid indentation");
            }
        }
    }
}

fn consume_while<X>(it: &mut Peekable<Chars>, f: X) -> Vec<char>
where X: Fn(char) -> bool {
    let mut v: Vec<char> = vec![];
    while let Some(&ch) = it.peek() {
        if f(ch) {
            it.next(); v.push(ch)
        } else {
            break;
        }
    }
    v
}

pub fn tokenize(s: String) -> Vec<Token> {
    let mut stack: Vec<usize> = vec![0];
    let mut it = s.chars().peekable();
    let mut tokens: Vec<Token> = vec![];
    let mut blank_line = true;

    let indent_level = consume_while(&mut it, is_whitespace).len();
    match it.peek() {
        Some(&ch) if ch != '\n' =>
            calc_indent(&mut stack, &mut tokens, indent_level),
        _ => (),
    };

    loop {
        match it.peek() {
            Some(&ch) => match ch {
                '0' ... '9' => {
                    blank_line = false;
                    let num: String = consume_while(&mut it, is_number)
                        .into_iter()
                        .collect();
                    tokens.push(Token::Int(num.parse::<i32>().unwrap()));
                },
                '+' | '<' | '=' | '(' | ')' | ':' => {
                    blank_line = false;
                    let nch = it.next().unwrap();
                    tokens.push(symbol_to_token(nch))
                },
                '\n' => {
                    it.next();
                    if !blank_line { tokens.push(Token::NewLine); }
                    blank_line = true;

                    let indent_level = consume_while(&mut it, is_whitespace).len();
                    match it.peek() {
                        Some(&ch) if ch != '\n' =>
                            calc_indent(&mut stack, &mut tokens, indent_level),
                        _ => (),
                    };
                }
                ch if is_alphabet(ch) => {
                    blank_line = false;
                    let nch = it.next().unwrap();
                    let mut id_vec = consume_while(&mut it, is_alphanumeric);
                    id_vec.insert(0, nch);
                    tokens.push(ident_to_token(id_vec.into_iter().collect()));
                },
                ch if is_whitespace(ch) => {
                    consume_while(&mut it, is_whitespace);
                }
                _ => panic!("Invalid char"),
            },
            None => break,
        }
    };

    loop {
        match stack.pop() {
            Some(i) if i != 0 => tokens.push(Token::Dedent),
            _ => break,
        }
    }

    tokens.push(Token::EOF);
    tokens
}

pub fn print_tokens(tokens: &Vec<Token>) {
    for t in tokens {
        println!("{:?}", t);
    }
}
