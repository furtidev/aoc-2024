use std::fs;

fn main() {
    let p1 = part1("./input.txt");
    let p2 = part2("./input.txt");
    println!("Answer of Part 1 => {}\nAnswer of Part 2 => {}", p1, p2);
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    Int(i32),
    Comma,
    LeftParen,
    RightParen,
    Mul,
    Invalid,
    Dont,
    Do,
    Eof,
}

struct Lexer {
    input: Vec<char>,
    curr_position: usize,
    curr_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let input: Vec<char> = input.chars().collect();
        Self {
            curr_position: 0,
            curr_char: Some(input[0]),
            input,
        }
    }

    fn advance(&mut self) {
        self.curr_position += 1;
        self.curr_char = if self.curr_position < self.input.len() {
            Some(self.input[self.curr_position])
        } else {
            None
        };
    }

    fn peek(&self) -> char {
        return self.input[self.curr_position + 1];
    }

    pub fn make_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while self.curr_char.is_some() {
            match self.curr_char.unwrap() {
                ',' => tokens.push(Token::Comma),
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                'm' => {
                    self.advance();
                    self.advance();
                    // hey, what's wrong with getting the most out of the lexer?
                    if self.peek() == '(' {
                        tokens.push(Token::Mul);
                    }
                }
                'd' => {
                    self.advance();
                    if self.peek() == '(' {
                        tokens.push(Token::Do);
                    } else {
                        tokens.push(Token::Dont);
                    }
                }
                '0'..='9' => {
                    let starting_pos: usize = self.curr_position;

                    loop {
                        match self.curr_char.unwrap() {
                            '0'..='9' => {
                                self.advance();
                            }
                            _ => {
                                break;
                            }
                        }
                    }

                    let literal = &self.input[starting_pos..self.curr_position];
                    let number: String = literal.iter().collect();
                    tokens.push(Token::Int(number.parse::<i32>().unwrap()));

                    // continuing here, because we don't want self.advance() to run again, a token will be skipped.
                    continue;
                }
                _ => {
                    tokens.push(Token::Invalid);
                }
            }

            self.advance();
        }

        tokens.push(Token::Eof);

        return tokens;
    }
}

struct Parser {
    tokens: Vec<Token>,
    curr_position: usize,
    curr_tok: Option<Token>,
    part: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, part: usize) -> Self {
        Self {
            curr_position: 0,
            curr_tok: Some(tokens[0]),
            tokens,
            part,
        }
    }

    fn advance(&mut self) {
        self.curr_position += 1;
        self.curr_tok = if self.curr_position < self.tokens.len() {
            Some(self.tokens[self.curr_position])
        } else {
            None
        };
    }

    fn peek(&self, level: usize) -> Token {
        return self.tokens[self.curr_position + level];
    }

    pub fn parse(&mut self) -> Vec<i32> {
        let mut program: Vec<i32> = vec![];
        let mut dont: bool = false;

        while self.curr_tok.unwrap() != Token::Eof {
            match self.curr_tok.unwrap() {
                Token::Mul => {
                    if self.part == 2 && dont {
                        self.advance();
                        continue;
                    }
                    self.advance();
                    // very bruteforce-y validator
                    if matches!(self.peek(1), Token::Int(_)) {
                        if self.peek(2) == Token::Comma {
                            if matches!(self.peek(3), Token::Int(_)) {
                                if self.peek(4) == Token::RightParen {
                                    let first: i32 = match self.peek(1) {
                                        Token::Int(value) => value,
                                        _ => 0,
                                    };
                                    let second: i32 = match self.peek(3) {
                                        Token::Int(value) => value,
                                        _ => 0,
                                    };
                                    program.push(first * second);
                                }
                            }
                        }
                    }
                }
                Token::Do => dont = false,
                Token::Dont => dont = true,
                _ => {}
            }

            self.advance();
        }

        return program;
    }
}

fn part1(filepath: &str) -> i32 {
    let mut sum: i32 = 0;
    let program = fs::read_to_string(filepath).unwrap();

    let mut lexer = Lexer::new(program);
    let tokens: Vec<Token> = lexer.make_tokens();

    let mut parser = Parser::new(tokens, 1);
    let instructions: Vec<i32> = parser.parse();

    for num in instructions {
        sum += num;
    }

    return sum;
}

fn part2(filepath: &str) -> i32 {
    let mut sum: i32 = 0;
    let program = fs::read_to_string(filepath).unwrap();

    let mut lexer = Lexer::new(program);
    let tokens: Vec<Token> = lexer.make_tokens();

    let mut parser = Parser::new(tokens, 2);
    let instructions: Vec<i32> = parser.parse();

    for num in instructions {
        sum += num;
    }

    return sum;
}

#[test]
fn test_part1() {
    let result = part1("./example.txt");
    assert_eq!(result, 161);
}

#[test]
fn test_part2() {
    let result = part2("./example.txt");
    assert_eq!(result, 48);
}
