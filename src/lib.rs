#![feature(entry_and_modify)]
extern crate pancurses;
extern crate rand;

use pancurses::{Window, COLOR_PAIR};
use rand::Rng;
use std::collections::HashMap;

const GOAL: i32 = 8;
const SCALE: i32 = 2;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Clone)]
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

pub fn to_colour_code(colour: &Colour) -> i32 {
    match *colour {
        Colour::Red => 0,
        Colour::Yellow => 1,
        Colour::Green => 2,
        Colour::Pink => 3,
        Colour::Blue => 4,
        Colour::Orange => 5,
    }
}

pub fn roll() -> Colour {
    let n = rand::thread_rng().gen_range(0, 6);
    from_colour_code(n).unwrap()
}

pub fn replicate(count: i32, ch: char) -> String {
    let mut result = String::new();
    for _ in 0..count {
        result.push(ch)
    }
    result
}

pub fn with_colour_pair<F>(window: &Window, colour: i32, action: F)
where
    F: Fn(),
{
    let pair = COLOR_PAIR(colour as u64);
    window.attron(pair);
    action();
    window.attroff(pair);
}

#[derive(Debug, Default, Clone)]
pub struct Board {
    snails: HashMap<Colour, i32>,
}

impl Board {
    pub fn new() -> Self {
        use Colour::*;

        let mut snails = HashMap::new();
        for colour in vec![Red, Yellow, Green, Pink, Blue, Orange] {
            snails.insert(colour, 0);
        }

        Board { snails: snails }
    }

    pub fn advance(&mut self, colour: Colour) {
        self.snails.entry(colour).and_modify(|s| *s += 1);
    }

    pub fn winner(&self) -> Option<Colour> {
        self.snails
            .iter()
            .find(|s| *s.1 == GOAL)
            .map(|s| s.0.clone())
    }

    pub fn draw(&self, window: &Window) {
        for y in 0..6 {
            window.attron(COLOR_PAIR(0));
            window.mvaddch(y, SCALE, '|');
            window.mvaddch(y, GOAL * SCALE, '|');

            let colour = from_colour_code(y).unwrap();
            let x = self.snails[&colour];
            with_colour_pair(window, y, || {
                window.mvaddstr(y, x * SCALE, &replicate(SCALE, '@'));
            });
        }

        for x in 0..GOAL + 1 {
            window.mvaddstr(6, x * SCALE, &format!("{}", x));
        }
    }
}
