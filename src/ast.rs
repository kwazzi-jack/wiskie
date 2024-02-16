
pub type Argument = Box<Expr>;
pub type Arguments = Vec<Argument>;

#[derive(Debug, PartialEq)]
pub enum Number {
    Integer(i64), // e.g. 10
    Decimal(f64), // e.g. 10.1
    MathConstant(MathConst), // e.g. Pi
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Absolute(Argument), // e.g. `Abs(10)`
    Negation(Argument), // e.g. `Neg(10)` or `-10`
    Sin(Argument), // e.g. `Sin(Pi)`
    Cos(Argument), // e.g. `Cos(90)`
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Addition(Argument, Argument), // e.g. `1 + x`
    Subtraction(Argument, Argument), // e.g. `1 - x`
    Multiplication(Argument, Argument), // e.g. `1 * x`
    Division(Argument, Argument), // e.g. `1 / x`
}

#[derive(Debug, PartialEq)]
pub struct CustomOperator(String, Arguments);

#[derive(Debug, PartialEq)]
pub enum Operator {
    UnOp(UnaryOperator),
    BinOp(BinaryOperator),
    CusOp(CustomOperator),
}

#[derive(Debug, PartialEq)]
pub enum MathConst {
    Pi,
    E,
}


#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(Number),
    Op(Operator),
}

impl Expr {

    // Constructors for Numbers
    pub fn number(num: Number) -> Self {
        Expr::Num(num)
    }

    pub fn integer(num: i64) -> Self {
        Expr::Num(
            Number::Integer(num)
        )
    }

    pub fn decimal(num: f64) -> Self {
        Expr::Num(
            Number::Decimal(num)
        )
    }

    pub fn math_constant(num: MathConst) -> Self {
        Expr::Num(
            Number::MathConstant(num)
        )
    }

    // Constructors for Operators
    pub fn operator(op: Operator) -> Self {
        Expr::Op(op)
    }

    // Constructors for Unary Operators
    pub fn unary_operator(op: UnaryOperator) -> Self {
        Expr::Op(
            Operator::UnOp(op)
        )
    }

    pub fn absolute(arg: Box<Expr>) -> Self {
        Expr::Op(
            Operator::UnOp(
                UnaryOperator::Absolute(arg)
            )
        )
    }

    pub fn negate(arg: Box<Expr>) -> Self {
        Expr::Op(
            Operator::UnOp(
                UnaryOperator::Absolute(arg)
            )
        )
    }

    // Constructors for Binary Operators
    pub fn binary_operator(op: BinaryOperator) -> Self {
        Expr::Op(
            Operator::BinOp(op)
        )
    }

    pub fn add(left: Box<Expr>, right: Box<Expr>) -> Self {
        Expr::Op(
            Operator::BinOp(
                BinaryOperator::Addition(left, right)
            )
        )
    }

    pub fn subtract(left: Box<Expr>, right: Box<Expr>) -> Self {
        Expr::Op(
            Operator::BinOp(
                BinaryOperator::Subtraction(left, right)
            )
        )
    }

    pub fn multiply(left: Box<Expr>, right: Box<Expr>) -> Self {
        Expr::Op(
            Operator::BinOp(
                BinaryOperator::Multiplication(left, right)
            )
        )
    }

    pub fn divide(left: Box<Expr>, right: Box<Expr>) -> Self {
        Expr::Op(
            Operator::BinOp(
                BinaryOperator::Division(left, right)
            )
        )
    }

    // Constructors for Custom Operators
    pub fn custom_operator(op: CustomOperator) -> Self {
        Expr::Op(
            Operator::CusOp(op)
        )
    }

    pub fn function(symbol: &str, args: Vec<Box<Expr>>) -> Self {
        Expr::Op(
            Operator::CusOp(
                CustomOperator(
                    symbol.to_string(), 
                    args,
                )
            )
        )
    }
}