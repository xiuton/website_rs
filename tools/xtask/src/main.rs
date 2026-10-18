//! 项目任务入口。
//!
//! 统一本地与 CI 的检查命令，避免文档/CI 各维护一套顺序：
//!
//! ```text
//! cargo xtask fmt      # cargo fmt --all
//! cargo xtask check    # fmt 检查 + clippy + 测试
//! cargo xtask build    # trunk release 构建（post_build 自动预渲染）
//! cargo xtask serve    # trunk serve
//! ```

use std::process::{Command, ExitCode};

fn run(program: &str, args: &[&str]) -> bool {
    println!("$ {program} {}", args.join(" "));
    match Command::new(program).args(args).status() {
        Ok(status) => status.success(),
        Err(err) => {
            eprintln!("无法执行 {program}: {err}");
            false
        }
    }
}

fn main() -> ExitCode {
    let task = std::env::args().nth(1).unwrap_or_else(|| "help".to_string());
    let ok = match task.as_str() {
        "fmt" => run("cargo", &["fmt", "--all"]),
        "check" => {
            run("cargo", &["fmt", "--all", "--", "--check"])
                && run(
                    "cargo",
                    &["clippy", "--workspace", "--exclude", "font-subset", "--all-targets", "--", "-D", "warnings"],
                )
                && run("cargo", &["test", "--workspace", "--exclude", "font-subset"])
        }
        "build" => run("trunk", &["build", "--release", "--no-default-features"]),
        "serve" => run("trunk", &["serve"]),
        "help" | "-h" | "--help" => {
            println!("用法: cargo xtask <fmt|check|build|serve>");
            true
        }
        other => {
            eprintln!("未知任务: {other}");
            eprintln!("用法: cargo xtask <fmt|check|build|serve>");
            false
        }
    };

    if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
