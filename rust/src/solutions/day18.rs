use crate::Res;
use std::io::{self, prelude::*};
use std::ops::{Add, Mul};

pub fn main() -> Res<()> {
    let mut soln = Solution::new(io::stdin().lock())?;
    println!("{}", soln.part1());
    println!("{}", soln.part2());
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Num(u64),
    Op(char),
    LeftParen,
    RightParen,
}

fn tokenize(expr: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut i = 0;
    while i < expr.len() {
        let c = expr.as_bytes()[i] as char;
        match c {
            ' ' => {
                // Ignore spaces.
                i += 1;
            }
            '*' | '+' => {
                tokens.push(Token::Op(c));
                i += 1;
            }
            '(' => {
                tokens.push(Token::LeftParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RightParen);
                i += 1;
            }
            '0'..='9' => {
                // Parse the entire number.
                let mut j = i + 1;
                while j < expr.len() && matches!(expr.as_bytes()[j], b'0'..=b'9') {
                    j += 1;
                }
                let n: u64 = expr[i..j].parse().unwrap();
                tokens.push(Token::Num(n));
                i = j;
            }
            _ => panic!("Invalid char: {}", c),
        }
    }
    tokens
}

fn find_matching_rparen(tokens: &[Token], lparen_idx: usize) -> usize {
    let mut depth = 1;
    for (i, &token) in tokens.iter().enumerate().skip(lparen_idx + 1) {
        match token {
            Token::LeftParen => depth += 1,
            Token::RightParen => depth -= 1,
            _ => (),
        }
        if depth == 0 {
            return i;
        }
    }
    panic!("No matching rparen")
}

struct Solution {
    lines: Vec<Vec<Token>>,
    add_before_mul: bool,
}

impl Solution {
    fn new(input: impl BufRead) -> Res<Solution> {
        let mut lines = vec![];
        for line in input.lines() {
            lines.push(tokenize(&line?));
        }
        Ok(Solution {
            lines,
            add_before_mul: false,
        })
    }

    fn part1(&mut self) -> u64 {
        self.add_before_mul = false;
        self.eval_all()
    }

    fn part2(&mut self) -> u64 {
        self.add_before_mul = true;
        self.eval_all()
    }

    /// Returns the sum of all lines after evaluating each.
    fn eval_all(&self) -> u64 {
        self.lines.iter().map(|l| self.eval(l)).sum()
    }

    fn eval(&self, tokens: &[Token]) -> u64 {
        let mut nums = vec![];
        let mut ops = vec![];
        let mut expect_num = true;

        let mut i = 0;
        while i < tokens.len() {
            match tokens[i] {
                Token::LeftParen => {
                    let j = find_matching_rparen(tokens, i);
                    nums.push(self.eval(&tokens[i + 1..j]));
                    i = j + 1;
                }
                Token::RightParen => panic!("Spurrious rparen"),
                Token::Num(n) => {
                    assert!(expect_num);
                    nums.push(n);
                    i += 1;
                }
                Token::Op(c) => {
                    assert!(!expect_num);
                    ops.push(c);
                    i += 1;
                }
            }
            expect_num = !expect_num;
        }
        assert!(!expect_num);
        debug_assert_eq!(nums.len(), ops.len() + 1);

        fold_ops(&nums, &ops, self.add_before_mul)
    }
}

fn fold_ops(nums: &[u64], ops: &[char], add_before_mul: bool) -> u64 {
    if add_before_mul {
        let (nums, ops) = first_pass(&nums, &ops);
        return fold_ops(&nums, &ops, false);
    }

    let mut acc = nums[0];
    for (i, &op) in ops.iter().enumerate() {
        let f = match op {
            '*' => u64::mul,
            '+' => u64::add,
            _ => unreachable!(),
        };
        acc = f(acc, nums[i + 1]);
    }
    acc
}

/// Helper for fold_ops. Preprocess to perform the adds but not the muls.
///
/// E.g.:
///     1 + 2 * 3 + 4 * 5 + 6
/// becomes
///     3 * 7 * 11
fn first_pass(nums: &[u64], ops: &[char]) -> (Vec<u64>, Vec<char>) {
    let mut new_nums = vec![];
    let mut new_ops = vec![];

    new_nums.push(nums[0]);
    for (i, &op) in ops.iter().enumerate() {
        match op {
            '*' => {
                new_ops.push(op);
                new_nums.push(nums[i + 1]);
            }
            '+' => {
                let len = new_nums.len();
                new_nums[len - 1] += nums[i + 1];
            }
            _ => unreachable!(),
        }
    }

    (new_nums, new_ops)
}
