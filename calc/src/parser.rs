use std::process::exit;

use crate::{token::{Token, TokenKind}, lexicalanalyzer::get_token, source::Source};

fn my_get_token(source: &mut Source, token: &mut Token) {
    get_token(source, token);
    source.next_pos = source.next_pos + token.str.len();
}

fn parse_primary_expression(source: &mut Source) -> f64 {
    let mut token: Token = Default::default();
    let mut minus_flag = false;
    let mut value;

    my_get_token(source, &mut token);
    if matches!(token.kind, TokenKind::SubOperatorToken) {
        minus_flag = true;
        my_get_token(source, &mut token);
    }

    if matches!(token.kind, TokenKind::NumberToken) {
        value = token.value;
    }
    else if matches!(token.kind, TokenKind::LeftParenToken) {
        value = parse_expression(source);
        my_get_token(source, &mut token);
        if !matches!(token.kind, TokenKind::RightParenToken) {
            println!("parse syntax error");
            exit(1);
        }
    }
    else {
        println!("parse syntax error 2");
        exit(1);
    }
    if minus_flag {
        value = -value;
    }
    return  value;
}

fn parse_term(source: &mut Source) -> f64 {
    let mut v1: f64 = parse_primary_expression(source);
    let mut v2: f64;
    let mut token: Token = Default::default();
    loop {
        my_get_token(source, &mut token);
        if !matches!(token.kind, TokenKind::MulOperatorToken) && !matches!(token.kind, TokenKind::DivOperatorToken) {
            source.next_pos = source.next_pos - token.str.len();
            break;
        }
        v2 = parse_primary_expression(source);
        if matches!(token.kind, TokenKind::MulOperatorToken) {
            v1 *= v2;
        }
        else if matches!(token.kind, TokenKind::DivOperatorToken) {
            v1 /= v2;
        }
    }
    v1
}

pub fn parse_expression(source: &mut Source) -> f64 {
    let mut v1: f64 = parse_term(source);
    let mut v2: f64;
    let mut token: Token = Default::default();
    loop {
        my_get_token(source, &mut token);
        if !matches!(token.kind, TokenKind::AddOperatorToken) && !matches!(token.kind, TokenKind::SubOperatorToken) {
            source.next_pos = source.next_pos - token.str.len();
            break;
        }
        v2 = parse_term(source);
        if matches!(token.kind, TokenKind::AddOperatorToken) {
            v1 += v2;
        }
        else if matches!(token.kind, TokenKind::SubOperatorToken) {
            v1 -= v2;
        }
    }
    v1
}
