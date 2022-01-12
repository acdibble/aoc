use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Action {
    Explode,
    Split,
}

#[derive(Clone, Debug)]
enum Value {
    Literal(i32),
    Pair(Box<Pair>),
}

impl Value {
    fn new_pair(pair: Pair) -> Self {
        Self::Pair(Box::from(pair))
    }

    fn split(&self) -> Option<Self> {
        match self {
            Self::Literal(amount) if *amount >= 10 => {
                let left_amount = *amount / 2;
                let left = Self::Literal(left_amount);
                let right = Self::Literal(*amount - left_amount);
                Some(Self::new_pair(Pair { left, right }))
            }
            _ => None,
        }
    }

    fn reduce(&mut self, depth: i32, action: Action) -> (bool, Option<i32>, Option<i32>) {
        match self {
            Self::Pair(pair) => pair.reduce(depth, action),
            _ => (false, None, None),
        }
    }

    fn magnitude(&self) -> i32 {
        match self {
            Self::Literal(value) => *value,
            Self::Pair(pair) => pair.magnitude(),
        }
    }
}

#[derive(Clone, Debug)]
struct Pair {
    left: Value,
    right: Value,
}

impl Pair {
    fn reduce(&mut self, depth: i32, action: Action) -> (bool, Option<i32>, Option<i32>) {
        if action == Action::Explode {
            if depth == 4 {
                let (left, right) = self.explode();
                return (true, Some(left), Some(right));
            }
        }

        match self.left.reduce(depth + 1, action) {
            (true, left @ Some(_), Some(right)) => {
                self.left = Value::Literal(0);
                self.add_to_right(right, true);
                return (true, left, None);
            }
            (true, None, Some(right)) => {
                self.add_to_right(right, true);
                return (true, None, None);
            }
            (true, left, right) => return (true, left, right),
            _ => (),
        }

        if action == Action::Split {
            if let Some(pair) = self.left.split() {
                self.left = pair;
                return (true, None, None);
            }
        }

        match self.right.reduce(depth + 1, action) {
            (true, Some(left), right @ Some(_)) => {
                self.right = Value::Literal(0);
                self.add_to_left(left, true);
                return (true, None, right);
            }
            (true, Some(left), None) => {
                self.add_to_left(left, true);
                return (true, None, None);
            }
            (true, left, right) => return (true, left, right),
            _ => (),
        }

        if action == Action::Split {
            if let Some(pair) = self.right.split() {
                self.right = pair;
                return (true, None, None);
            }
        }

        (false, None, None)
    }

    fn add_to_right(&mut self, amount: i32, should_switch: bool) {
        match &mut self.right {
            Value::Literal(value) => *value += amount,
            Value::Pair(pair) if should_switch => pair.add_to_left(amount, false),
            Value::Pair(pair) => pair.add_to_right(amount, false),
        }
    }

    fn add_to_left(&mut self, amount: i32, should_switch: bool) {
        match &mut self.left {
            Value::Literal(value) => *value += amount,
            Value::Pair(pair) if should_switch => pair.add_to_right(amount, false),
            Value::Pair(pair) => pair.add_to_left(amount, false),
        }
    }

    fn explode(&self) -> (i32, i32) {
        let left = match &self.left {
            Value::Literal(value) => value,
            Value::Pair(_) => unreachable!("cannot explode left child"),
        };
        let right = match &self.right {
            Value::Literal(value) => value,
            Value::Pair(_) => unreachable!("cannot explode right child"),
        };
        (*left, *right)
    }

    fn magnitude(&self) -> i32 {
        3 * self.left.magnitude() + 2 * self.right.magnitude()
    }
}

#[cfg(test)]
impl Pair {
    fn to_string_with_buffer(&self, buf: &mut String) {
        buf.push('[');
        match &self.left {
            Value::Literal(value) => buf.push_str(&value.to_string()),
            Value::Pair(pair) => pair.to_string_with_buffer(buf),
        }
        buf.push(',');
        match &self.right {
            Value::Literal(value) => buf.push_str(&value.to_string()),
            Value::Pair(pair) => pair.to_string_with_buffer(buf),
        }
        buf.push(']')
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        self.to_string_with_buffer(&mut result);
        result
    }
}

impl std::ops::Add<Self> for Pair {
    type Output = Self;

    fn add(self, other: Self) -> Self>::Output {
        let mut result = Pair {
            left: Value::new_pair(self),
            right: Value::new_pair(other),
        };

        loop {
            #[cfg(test)]
            println!("{}", result.to_string());
            while let (true, _, _) = result.reduce(0, Action::Explode) {
                #[cfg(test)]
                println!("{}", result.to_string());
            }

            if let (true, _, _) = result.reduce(0, Action::Split) {
                continue;
            }

            break;
        }

        result
    }
}

type SnailfishChars<'a> = std::iter::Peekable<std::str::Chars<'a>>;

fn parse_literal(chars: &mut SnailfishChars) -> Value {
    let mut value = 0;

    while matches!(chars.peek(), Some('0'..='9')) {
        let digit = chars.next().unwrap().to_digit(10).unwrap();

        value *= 10;
        value += digit;
    }

    Value::Literal(value as i32)
}

fn parse_value(chars: &mut SnailfishChars) -> Value {
    match chars.peek() {
        Some('[') => Value::Pair(Box::from(parse_pair(chars))),
        Some('0'..='9') => parse_literal(chars),
        _ => unreachable!("expected start of left"),
    }
}

fn parse_pair(chars: &mut SnailfishChars) -> Pair {
    chars.next(); // consume '['
    let left = parse_value(chars);

    if !matches!(chars.next(), Some(',')) {
        unreachable!("expected comma")
    }

    let right = parse_value(chars);

    chars.next(); // consume ']'

    Pair { left, right }
}

fn parse_number(input: &str) -> Pair {
    let mut chars = input.chars().peekable();
    parse_pair(&mut chars)
}

fn sum_list(input: &str) -> Pair {
    input
        .lines()
        .map(parse_number)
        .reduce(|a, b| a + b)
        .unwrap()
}

fn part_one(input: &str) -> i32 {
    let result = sum_list(input);

    result.magnitude()
}

fn part_two(input: &str) -> i32 {
    let numbers: Vec<_> = input.lines().map(parse_number).collect();

    let mut largest = i32::MIN;

    for (index, a) in numbers.iter().enumerate() {
        for b in numbers.iter().skip(index + 1) {
            let c = a.clone() + b.clone();
            largest = largest.max(c.magnitude());
            let c = b.clone() + a.clone();
            largest = largest.max(c.magnitude());
        }
    }

    largest
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reduce() {
        // let mut reducible = parse_number("[[[[[9,8],1],2],3],4]");
        // reducible.reduce(0);
        // // println!("{}", reducible.to_string());
        // assert_eq!("[[[[0,9],2],3],4]", reducible.to_string());

        // let mut reducible = parse_number("[7,[6,[5,[4,[3,2]]]]]");
        // reducible.reduce(0);
        // // println!("{}", reducible.to_string());
        // assert_eq!("[7,[6,[5,[7,0]]]]", reducible.to_string());

        // let mut reducible = parse_number("[[6,[5,[4,[3,2]]]],1]");
        // reducible.reduce(0);
        // // println!("{}", reducible.to_string());
        // assert_eq!("[[6,[5,[7,0]]],3]", reducible.to_string());

        // let mut reducible = parse_number("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        // reducible.reduce(0);
        // // println!("{}", reducible.to_string());
        // assert_eq!("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", reducible.to_string());

        // reducible.reduce(0);
        // // println!("{}", reducible.to_string());
        // assert_eq!("[[3,[2,[8,0]]],[9,[5,[7,0]]]]", reducible.to_string());
    }

    #[test]
    fn test_add() {
        //     let a = parse_number("[[[[4,3],4],4],[7,[[8,4],9]]]");
        //     let b = parse_number("[1,1]");
        //     let c = a + b;
        //     assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", c.to_string());

        let a = parse_number("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
        let b = parse_number("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]");
        let c = a + b;
        assert_eq!(
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            c.to_string()
        );

        // let a = parse_number("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]");
        // let b = parse_number("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]");
        // let c = a + b;
        // assert_eq!(
        //     "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
        //     c.to_string()
        // );
    }

    #[test]
    fn test_lists() {
        let result = sum_list(
            "[1,1]
[2,2]
[3,3]
[4,4]",
        );

        assert_eq!("[[[[1,1],[2,2]],[3,3]],[4,4]]", result.to_string());

        let result = sum_list(
            "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]",
        );
        assert_eq!("[[[[3,0],[5,3]],[4,4]],[5,5]]", result.to_string());

        let result = sum_list(
            "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]",
        );
        assert_eq!("[[[[5,0],[7,4]],[5,5]],[6,6]]", result.to_string());

        let result = sum_list(
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]",
        );
        assert_eq!(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            result.to_string()
        );
    }
}

fn time_it<F, T>(fun: F) -> T
where
    F: Fn() -> T,
{
    let start = SystemTime::now();
    let result = fun();
    println!("Time elapsed: {} µs", start.elapsed().unwrap().as_micros());
    result
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    time_it(|| println!("part 1: {}", part_one(&input)));
    time_it(|| println!("part 2: {}", part_two(&input)));

    Ok(())
}
