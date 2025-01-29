pub type Number = f64;

pub const EPSILON: Number = 1e-9;

pub trait AboutEq {
    fn about_eq(self, v: Self) -> bool;
    fn about_zero(self) -> bool;
}

impl AboutEq for Number {
    fn about_eq(self, v: Self) -> bool {
        (v - self).abs() <= EPSILON
    }
    fn about_zero(self) -> bool {
        self.abs() <= EPSILON
    }
}

// const OPS: &str = "+-*/()";
// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// enum Op {
//     Add,
//     Sub,
//     Mul,
//     Div,
//     PaL,
//     PaR,
// }
// impl Op {
//     fn from_char(c: &char) -> Op {
//         match c {
//             '+' => Op::Add,
//             '-' => Op::Sub,
//             '*' => Op::Mul,
//             '/' => Op::Div,
//             '(' => Op::PaL,
//             ')' => Op::PaR,
//             _ => panic!(),
//         }
//     }
//     fn precedence(&self) -> u8 {
//         match self {
//             Op::PaL | Op::PaR => 3,
//             Op::Mul | Op::Div => 1,
//             Op::Add | Op::Sub => 0,
//         }
//     }
// }
// impl Ord for Op {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.precedence().cmp(&other.precedence())
//     }
// }
// impl PartialOrd for Op {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

// enum Token {
//     Op(Op),
//     Num(Number),
// }

// fn parse_number(s: &str) -> IResult<&str, Token, ()> {
//     if let Ok((i, o)) = take_while1::<_, _, ()>(char::is_numeric)(s) {
//         if let Ok(o) = o.parse() {
//             return Ok((i, Token::Num(o)));
//         }
//         return Err(nom::Err::Error(()));
//     }
//     return Err(nom::Err::Error(()));
// }

// fn parse_op(s: &str) -> IResult<&str, Token, ()> {
//     one_of(OPS)(s).map(|(i, o)| (i, Token::Op(Op::from_char(&o))))
// }

// pub fn parse(s: &str) -> Result<(), ()> {
//     let Ok((_, stack)): Result<(_, Vec<Token>), _> = ws(all_consuming(many0::<_, _, (), _>(
//         preceded(take_while(char::is_whitespace), alt(parse_number, parse_op)),
//     )))(s) else {
//         return Err(());
//     };
//     let mut output: Vec<Number> = Vec::new();
//     let mut ops: Vec<Op> = Vec::new();
//     for (quant, op) in stack {
//         output.push();
//         let op = Op::from_char(&op);
//         while ops.last().is_some_and(|o| o != &Op::PaL && o >= &op) {
//             ops.push(ops.pop().unwrap())
//         }
//     }
//     Ok(())
// }
