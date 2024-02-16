use nom::IResult;
use wiskie::{
    ast::{Expr, Number},
    parser::parse_expr,
};

#[test]
fn test_parse_num() {
    let expected = Expr::Num(Number::Integer(42));
    match parse_expr("42") {
        IResult::Ok((_, result)) => assert_eq!(result, expected),
        IResult::Err(error) => panic!("Failed to parse number: {:?}", error),
    }
}

#[test]
fn test_parse_add_int_and_int() {
    let expr1 = Expr::Num(Number::Integer(1));
    let expr2 = Expr::Num(Number::Integer(2));
    let expected = Expr::add(Box::new(expr1), Box::new(expr2));
    match parse_expr("1 + 2") {
        IResult::Ok((_, result)) => assert_eq!(result, expected),
        IResult::Err(error) => panic!("Failed to parse addition: {:?}", error),
    }
}

#[test]
fn test_parse_mul_int_and_int() {
    let expr1 = Expr::Num(Number::Integer(3));
    let expr2 = Expr::Num(Number::Integer(4));
    let expected = Expr::multiply(Box::new(expr1), Box::new(expr2));
    match parse_expr("3 * 4") {
        IResult::Ok((_, result)) => assert_eq!(result, expected),
        IResult::Err(error) => panic!("Failed to parse multiplication: {:?}", error),
    }
}

#[test]
fn test_parse_div_int_and_int() {
    let expr1 = Expr::Num(Number::Integer(10));
    let expr2 = Expr::Num(Number::Integer(2));
    let expected = Expr::divide(Box::new(expr1), Box::new(expr2));
    match parse_expr("10 / 2") {
        IResult::Ok((_, result)) => assert_eq!(result, expected),
        IResult::Err(error) => panic!("Failed to parse division: {:?}", error),
    }
}