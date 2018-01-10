#![feature(entry_and_modify)]
extern crate rand;
extern crate pancurses;

use pancurses::{Window, COLOR_PAIR};
use rand::Rng;
use std::collections::HashMap;

pub const GOAL: u8 = 8;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Clone)]
pub enum Colour {
    Red,
    Yellow,
    Green,
    Pink,
    Blue,
    Orange,
}

fn from_colour_code(code: u8) -> Option<Colour> {
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

pub fn to_colour_code(colour: &Colour) -> u8 {
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

#[derive(Debug, Default, Clone)]
pub struct Board {
    snails: HashMap<Colour, u8>,
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
            .find(|&(_, p)| *p == GOAL)
            .map(|t| t.0.clone())
    }

    pub fn draw(&self, window: &Window) {
        for y in 0..6 {
            window.attron(COLOR_PAIR(0));
            window.mvaddch(y as i32, 2, '|');
            window.mvaddch(y as i32, GOAL as i32 * 2, '|');
        }

        for y in 0..6 {
            let colour = from_colour_code(y).unwrap();
            let &x = self.snails.get(&colour).unwrap();
            window.attron(COLOR_PAIR(y as u64 + 1));
            window.mvaddstr(y as i32, x as i32 * 2, "@@");
        }

        for x in 0..GOAL+1 {
            window.mvaddstr(6, x as i32 * 2, &format!("{}", x));
        }
    }
}
