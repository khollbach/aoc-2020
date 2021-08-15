use aoc_2020::{self, Res};
use std::env;
use std::process;

fn main() -> Res<()> {
    let mut args = env::args();
    let prog_name = args.next().unwrap();
    let args: Vec<_> = args.collect();

    let usage = || {
        eprint_usage(&prog_name);
        process::exit(1)
    };

    if args.len() != 1 {
        eprintln!("Expected 1 argument, got {}\n", args.len());
        usage();
    }

    let day: u32 = match args[0].parse() {
        Ok(n) if 1 <= n && n <= 25 => n,
        Ok(n) => {
            eprintln!("Expected a number from 1 through 25, got {}\n", n);
            usage()
        }
        Err(_) => {
            eprintln!("Not a number from 1 through 25: {}\n", args[0]);
            usage()
        }
    };

    aoc_2020::solution(day)
}

fn eprint_usage(prog_name: &str) {
    eprintln!(
        "usage: `{} <num>`\nwhere <num> is a number from 1 through 25",
        prog_name
    );
}
