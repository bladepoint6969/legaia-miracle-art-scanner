use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use arts_scanner::Move;
use clap::Parser;
use itertools::Itertools;

#[derive(Debug, Clone, Parser)]
struct Cli {
    #[arg(short, long)]
    /// File path to arts file
    ///
    /// The file should have one art per line, include no whitespace, and use
    /// the following characters:
    ///   <: Left
    ///   >: Right
    ///   ^: High
    ///   v: Low
    arts: Option<PathBuf>,
    #[arg(short, long, default_value_t = 9, value_parser = 1..=9)]
    /// The maximum number of consecutive identical moves
    max_consecutive_repeats: i64,
}

fn main() {
    let args = Cli::parse();

    let arts: Vec<Vec<Move>> = match args.arts {
        Some(art_file) => {
            let art_file = File::open(art_file).unwrap();
            let arts = BufReader::new(art_file).lines();
            arts.map(Result::unwrap)
                .map(|art| {
                    art.chars()
                        .map(|ch| match ch {
                            '<' => Move::Left,
                            '>' => Move::Right,
                            '^' => Move::High,
                            'v' => Move::Low,
                            _ => panic!("Unrecognized"),
                        })
                        .collect()
                })
                .collect()
        }
        None => vec![],
    };

    let all_combos = (0..9)
        .map(|_| [Move::High, Move::Left, Move::Low, Move::Right])
        .multi_cartesian_product()
        .filter(|combo| !combo_has_too_many_repeats(combo, args.max_consecutive_repeats))
        .filter(|combo| !combo_has_art(combo, &arts))
        .unique();

    for combo in all_combos {
        println!("{combo:?}");
    }
}

fn combo_has_art(combo: &[Move], arts: &[Vec<Move>]) -> bool {
    arts.iter().any(|art| {
        let mut combo_window = combo.windows(art.len());
        combo_window.any(|section| {
            let mut zip = section.iter().zip(art.iter());
            zip.all(|(this, that)| this == that)
        })
    })
}

fn combo_has_too_many_repeats(combo: &[Move], max_allowed_repeats: i64) -> bool {
    if max_allowed_repeats == 9 {
        return false;
    }
    combo
        .windows(max_allowed_repeats as usize + 1)
        .any(|window| match window {
            [head, tail @ ..] => tail.iter().all(|x| x == head),
            [] => false,
        })
}
