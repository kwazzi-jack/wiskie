// src/lib.rs

pub enum Expr {
    Number(f64),
    Symbol(String),
    Add(Box<Expr>, Box<Expr>),
    // Add more expression types as needed
}