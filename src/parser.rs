use crate::ast::{Expr, MathConst, Number, Operator, UnaryOperator};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::{char, digit1, multispace0}, combinator::{map, map_res, recognize, value}, sequence::{delimited, preceded, tuple}, IResult, Parser
};

// For parsing base expression
pub fn parse_base_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_num,
        parse_absolute,
        parse_parentheses,
        parse_negate, 
    ))(input)
}

pub fn parse_parentheses(input: &str) -> IResult<&str, Expr> {
    delimited(
        char('('), 
        parse_expr, 
        char(')'),
    )(input)
}

// For parsing general expressions
pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_add,
        parse_subtract,
        parse_term,
    ))(input)
}

// For parsing general terms
pub fn parse_term(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_multiply,
        parse_divide,
        parse_base_expr,
    ))(input)
}

// Parse an integer
fn parse_integer(input: &str) -> IResult<&str, Expr> {
    map_res(
        digit1, 
        |s: &str| s.parse::<i64>().map(|num| Expr::integer(num)),
    )(input)
}

// Parse a decimal
fn parse_decimal(input: &str) -> IResult<&str, Expr> {
    map_res(
        recognize(tuple((digit1, char('.'), digit1))),
        |s: &str| s.parse::<f64>().map(|num| Expr::decimal(num)),
    )(input)
}

// Parse in a number
pub fn parse_num(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_integer,
        parse_decimal,
    ))(input)
}

// Parse unary operations
pub fn parse_negate(input: &str) -> IResult<&str, Expr> {
    map(
        preceded(char('-'), parse_base_expr),
        |expr| Expr::negate(Box::new(expr)),
    )(input)
}

pub fn parse_absolute(input: &str) -> IResult<&str, Expr> {
    map(
        preceded(
            tag("Abs"),
            delimited(
                char('['), parse_base_expr, char(']'),
            )
        ),
        |expr| Expr::absolute(Box::new(expr)),
    )(input)
}

// Parse binary operations
pub fn parse_add(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((parse_expr, multispace0, char('+'), multispace0, parse_expr)),
        |(left, _, _, _, right)| Expr::add(Box::new(left), Box::new(right)),       
    )(input)
}

pub fn parse_subtract(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((parse_expr, multispace0, char('-'), multispace0, parse_expr)),
        |(left, _, _, _, right)| Expr::subtract(Box::new(left), Box::new(right)),       
    )(input)
}

pub fn parse_multiply(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((parse_expr, multispace0, char('*'), multispace0, parse_expr)),
        |(left, _, _, _, right)| Expr::multiply(Box::new(left), Box::new(right)),
    )(input)
}

pub fn parse_divide(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((parse_expr, multispace0, char('/'), multispace0, parse_expr)),
        |(left, _, _, _, right)| Expr::divide(Box::new(left), Box::new(right)),
    )(input)
}

