
use std::process::exit;
use crate::{token::{MAX_TOKEN_SIZE, Token, TokenKind}, source::Source};

enum LexerStatus {
    InitialStatus,
    InIntPartStatus,
    DotStatus,
    InFracPartStatus
}

pub fn get_token(source: &mut Source, token: &mut Token) {
    let mut status = LexerStatus::InitialStatus;
    token.str = String::new();
    let len = source.raw.len();
    if source.next_pos > len - 1 {
        return;
    }
    for (i, c) in source.raw.chars().enumerate() {
        if i < source.next_pos {
            continue;
        }
        if (matches!(status, LexerStatus::InIntPartStatus) || matches!(status, LexerStatus::InFracPartStatus)) && !c.is_numeric() && c != '.' {
            token.kind = TokenKind::NumberToken;
            token.value = token.str.trim().parse::<f64>().unwrap();
            source.next_pos = i;
            return;
        }
        if c.is_whitespace() {
            if c == '\n' {
                token.kind = TokenKind::EndOfLineToken;
                source.next_pos = i;
                return;
            }
            continue;
        }
        if i >= MAX_TOKEN_SIZE as usize {
            println!("token too long.");
            exit(1);
        }
        token.str.push(c);
        // token.str.push('\0');
        if c == '+' {
            token.kind = TokenKind::AddOperatorToken;
            source.next_pos = i + 1;
            return;
        }
        else if c == '-' {
            token.kind = TokenKind::SubOperatorToken;
            source.next_pos = i + 1;
            return;
        }
        else if c == '*' {
            token.kind = TokenKind::MulOperatorToken;
            source.next_pos = i + 1;
            return;
        }
        else if c == '/' {
            token.kind = TokenKind::DivOperatorToken;
            source.next_pos = i + 1;
            return;
        }
        else if c.is_numeric() {
            if matches!(status, LexerStatus::InitialStatus) {
                status = LexerStatus::InIntPartStatus;
            }
            else if matches!(status, LexerStatus::DotStatus) {
                status = LexerStatus::InFracPartStatus;
            }
        }
        else if c == '.' {
            if matches!(status, LexerStatus::InIntPartStatus) {
                status = LexerStatus::DotStatus;
            }
            else {
                println!("syntax error.");
                exit(1);
            }
        }
    }
}
