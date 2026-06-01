---
title: "Rust 案例 - 计算器"
date: 2025-04-10 22:10:00
author: 干徒
link: https://www.cnblogs.com/ganto/articles/18819665
tags: ["Rust", "技术"]
---

# Rust 案例 - 计算器

```rust
use std::io;
use std::io::Write;

fn main() {
    let mut user_typer = UserTyper::new(CommandLineComputer);
    user_typer.type_expr();
    println!("Result: {}", user_typer.compute());
}

trait Computer {
    fn compute(&self, expr: &str) -> i32;
}

struct CommandLineComputer;

impl Computer for CommandLineComputer {
    fn compute(&self, expr: &str) -> i32 {
        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut op: Option<char> = None;

        for c in expr.trim().chars() {
            if c.is_digit(10) {
                if op.is_none() {
                    num1.push(c);
                } else {
                    num2.push(c);
                }
                continue;
            }
            match c {
                '+' | '-' | '*' | '/' if op.is_none() => op = Some(c),
                _ if c.is_whitespace() => continue,
                _ => panic!("Invalid character: {}", c),
            }
        }

        if num1.is_empty() || num2.is_empty() || op.is_none() {
            panic!("Invalid expression: {}", expr);
        }

        let num1 = num1.parse::<i32>().unwrap();
        let num2: i32 = num2.parse().unwrap();
        let op = op.unwrap();

        match op {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => num1 / num2,
            _ => unreachable!(),
        }
    }
}

struct UserTyper<T: Computer> {
    computer: T,
    expr: String,
}

impl<T: Computer> UserTyper<T> {
    fn new(computer: T) -> Self {
        Self { computer, expr: String::new() }
    }

    fn type_expr(&mut self) {
        println!("Please type an expression: {}", self.expr);
        io::stdout().flush().expect("Unable to flush stdout");
        io::stdin().read_line(&mut self.expr).unwrap();
    }

    fn compute(&self) -> i32 {
        self.computer.compute(&self.expr)
    }
}

```