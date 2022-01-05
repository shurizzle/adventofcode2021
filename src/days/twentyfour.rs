use std::str::FromStr;

const INPUT: &str = include_str!("../../inputs/24");

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Register {
    W,
    X,
    Y,
    Z,
}

impl Register {
    pub fn get(&self, mem: &Mem) -> i64 {
        match self {
            Self::W => mem.w,
            Self::X => mem.x,
            Self::Y => mem.y,
            Self::Z => mem.z,
        }
    }

    pub fn set(&self, mem: &mut Mem, value: i64) {
        *match self {
            Self::W => &mut mem.w,
            Self::X => &mut mem.x,
            Self::Y => &mut mem.y,
            Self::Z => &mut mem.z,
        } = value
    }
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(());
        }

        match s.chars().next().unwrap() {
            'w' | 'W' => Ok(Self::W),
            'x' | 'X' => Ok(Self::X),
            'y' | 'Y' => Ok(Self::Y),
            'z' | 'Z' => Ok(Self::Z),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Literal(i64);

impl Literal {
    pub fn get(&self) -> i64 {
        self.0
    }
}

impl FromStr for Literal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i64>().map(|n| Literal(n)).or(Err(()))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum RegisterOrLiteral {
    Register(Register),
    Literal(Literal),
}

impl RegisterOrLiteral {
    pub fn get(&self, mem: &Mem) -> i64 {
        match self {
            Self::Register(ref reg) => reg.get(mem),
            Self::Literal(ref n) => n.get(),
        }
    }
}

impl FromStr for RegisterOrLiteral {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<Register>() {
            Ok(reg) => Ok(RegisterOrLiteral::Register(reg)),
            Err(_) => Ok(RegisterOrLiteral::Literal(s.parse()?)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Op {
    Inp(Register),
    Add(Register, RegisterOrLiteral),
    Mul(Register, RegisterOrLiteral),
    Div(Register, RegisterOrLiteral),
    Mod(Register, RegisterOrLiteral),
    Eql(Register, RegisterOrLiteral),
}

impl Op {
    pub fn apply<I>(&self, mem: &mut Mem, input: &mut I) -> Result<(), ()>
    where
        I: Input,
    {
        match self {
            Self::Inp(reg) => reg.set(mem, input.input().ok_or(())?),
            Self::Add(reg, val) => reg.set(mem, reg.get(mem) + val.get(mem)),
            Self::Mul(reg, val) => reg.set(mem, reg.get(mem) * val.get(mem)),
            Self::Div(reg, val) => reg.set(mem, reg.get(mem) / val.get(mem)),
            Self::Mod(reg, val) => reg.set(mem, reg.get(mem) % val.get(mem)),
            Self::Eql(reg, val) => reg.set(mem, if reg.get(mem) == val.get(mem) { 1 } else { 0 }),
        }

        Ok(())
    }
}

macro_rules! parse_binop {
    ($op:ident, $a:ty, $b:ty, $s:ident) => {{
        let sp = $s.find(' ').ok_or(())?;
        let start = sp + 1;
        if start >= $s.len() {
            return Err(());
        }
        Ok(Self::$op(
            <$a as FromStr>::from_str(&$s[..sp])?,
            <$b as FromStr>::from_str($s[(sp + 1)..].trim_start())?,
        ))
    }};
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let sc = s.find(' ').unwrap_or(s.len());
        let inst = s[..sc].to_lowercase();
        let start = sc + 1;
        if start >= s.len() {
            return Err(());
        }
        let s = s[(sc + 1)..].trim_start();

        match inst.as_str() {
            "inp" => Ok(Op::Inp(<Register as FromStr>::from_str(s)?)),
            "add" => parse_binop!(Add, Register, RegisterOrLiteral, s),
            "mul" => parse_binop!(Mul, Register, RegisterOrLiteral, s),
            "div" => parse_binop!(Div, Register, RegisterOrLiteral, s),
            "mod" => parse_binop!(Mod, Register, RegisterOrLiteral, s),
            "eql" => parse_binop!(Eql, Register, RegisterOrLiteral, s),
            _ => Err(()),
        }
    }
}

fn parse(text: &str) -> Vec<Op> {
    text.trim()
        .lines()
        .filter_map(|mut line| {
            line = line.trim();
            if line.is_empty() {
                None
            } else {
                Some(<Op as FromStr>::from_str(line).unwrap())
            }
        })
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
struct Mem {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    pc: usize,
}

trait Input {
    fn input(&mut self) -> Option<i64>;
}

impl Input for Vec<i64> {
    fn input(&mut self) -> Option<i64> {
        if self.is_empty() {
            None
        } else {
            Some(self.remove(0))
        }
    }
}

impl Input for i64 {
    fn input(&mut self) -> Option<i64> {
        let res = *self % 10;
        *self /= 10;
        Some(res)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Alu<'a, I> {
    mem: Mem,
    ops: &'a Vec<Op>,
    input: I,
}

impl<'a, I> Alu<'a, I>
where
    I: Input,
{
    pub fn new(ops: &'a Vec<Op>, input: I) -> Self {
        Self {
            mem: Default::default(),
            ops,
            input,
        }
    }

    #[inline]
    pub fn mem(&self) -> &Mem {
        &self.mem
    }

    pub fn ooo(&self) -> bool {
        self.mem.pc >= self.ops.len()
    }

    pub fn tick(&mut self) -> Result<(), ()> {
        if self.mem.pc < self.ops.len() {
            let res = self.ops[self.mem.pc].apply(&mut self.mem, &mut self.input)?;
            self.mem.pc += 1;
            Ok(res)
        } else {
            Err(())
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Program<'a> {
    ops: &'a Vec<Op>,
}

impl<'a> Program<'a> {
    pub fn new(ops: &'a Vec<Op>) -> Self {
        Self { ops }
    }

    pub fn run<I>(&self, input: I) -> Result<Alu<'a, I>, ()>
    where
        I: Input,
    {
        let mut alu = Alu::new(&self.ops, input);
        while !alu.ooo() {
            alu.tick()?;
        }
        Ok(alu)
    }
}

pub(crate) fn solution1(text: &str) -> String {
    let ops = parse(text);
    let monad = Program::new(&ops);
    let mut i = 0;
    for code in (11111111111111..=99999999999999).rev() {
        if !code.to_string().contains('0') {
            if i > 1000000 {
                println!("{}", code);
                i = 0;
            } else {
                i += 1;
            }
            if monad.run(code).unwrap().mem().z == 0 {
                return code.to_string();
            }
        }
    }
    unreachable!()
}

pub(crate) fn solution2(_text: &str) -> usize {
    todo!()
}

pub fn solution() {
    println!("Solution 1: {}", solution1(INPUT));
    println!("Solution 2: {}", solution2(INPUT));
}
