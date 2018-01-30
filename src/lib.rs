#![feature(entry_and_modify)]
extern crate pancurses;
extern crate rand;

use pancurses::{Window, COLOR_PAIR};
use rand::Rng;
use std::collections::HashMap;
use std::iter::repeat;

const GOAL: i32 = 8;
const SCALE: i32 = 2;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy)]
pub enum Colour {
    Red,
    Yellow,
    Green,
    Pink,
    Blue,
    Orange,
}

fn from_colour_code(code: i32) -> Option<Colour> {
    match code {
        0 => Some(Colour::Red),
        1 => Some(Colour::Yellow),
        2 => Some(Colour::Green),
        3 => Some(Colour::Pink),
        4 => Some(Colour::Blue),
        5 => Some(Colour::Orange),
        _ => None,
    }
}

pub fn roll() -> Colour {
    let n = rand::thread_rng().gen_range(0, 6);
    from_colour_code(n).unwrap()
}

pub trait WindowExt {
    fn with_colour_pair<F: Fn()>(&self, colour: i32, action: F);
}

impl WindowExt for Window {
    fn with_colour_pair<F: Fn()>(&self, colour: i32, action: F) {
        let pair = COLOR_PAIR(colour as u64);
        self.attron(pair);
        action();
        self.attroff(pair);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Board(HashMap<Colour, i32>);

impl Board {
    pub fn new() -> Self {
        use Colour::*;

        let mut snails = HashMap::new();
        for colour in vec![Red, Yellow, Green, Pink, Blue, Orange] {
            snails.insert(colour, 0);
        }

        Board(snails)
    }

    pub fn advance(&mut self, colour: Colour) {
        self.0.entry(colour).and_modify(|s| *s += 1);
    }

    pub fn winner(&self) -> Option<Colour> {
        self.0.iter().find(|s| *s.1 == GOAL).map(|s| s.0.clone())
    }

    pub fn draw(&self, window: &Window) {
        for y in 0..6 {
            window.attron(COLOR_PAIR(0));
            window.mvaddch(y, SCALE, '|');
            window.mvaddch(y, GOAL * SCALE, '|');

            let colour = from_colour_code(y).unwrap();
            let x = self.0[&colour];
            window.with_colour_pair(y, || {
                window.mvaddstr(
                    y,
                    x * SCALE,
                    &repeat('@').take(SCALE as usize).collect::<String>(),
                );
            });
        }

        for x in 0..GOAL + 1 {
            window.mvaddstr(6, x * SCALE, &format!("{}", x));
        }
    }
}
