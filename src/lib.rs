#![feature(try_from)]
extern crate pancurses;
extern crate rand;

use pancurses::{Window, COLOR_PAIR};
use rand::{Rand, Rng};
use std::collections::HashMap;
use std::iter::repeat;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy)]
pub enum Colour {
    Red,
    Yellow,
    Green,
    Pink,
    Blue,
    Orange,
}

impl TryFrom<i32> for Colour {
    type Error = ();
    fn try_from(code: i32) -> Result<Self, Self::Error> {
        match code {
            0 => Ok(Colour::Red),
            1 => Ok(Colour::Yellow),
            2 => Ok(Colour::Green),
            3 => Ok(Colour::Pink),
            4 => Ok(Colour::Blue),
            5 => Ok(Colour::Orange),
            _ => Err(()),
        }
    }
}

impl Into<u64> for Colour {
    fn into(self) -> u64 {
        self as u64
    }
}

impl Rand for Colour {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        use Colour::*;
        *rng.choose(&[Red, Yellow, Green, Pink, Blue, Orange]).unwrap()
    }
}

pub trait WindowExt {
    fn with_colour_pair<I, F>(&self, colour: I, action: F)
    where
        F: Fn(),
        I: Into<u64>;
}

impl WindowExt for Window {
    fn with_colour_pair<I, F>(&self, colour: I, action: F)
    where
        F: Fn(),
        I: Into<u64>,
    {
        let pair = COLOR_PAIR(colour.into());
        self.attron(pair);
        action();
        self.attroff(pair);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Board {
    scale: i32,
    goal: i32,
    snails: HashMap<Colour, i32>,
}

impl Board {
    pub fn new(goal: i32) -> Self {
        use Colour::*;

        let scale = (goal as f32).log10() as i32 + 2;

        let mut snails = HashMap::new();
        for &colour in &[Red, Yellow, Green, Pink, Blue, Orange] {
            snails.insert(colour, 0);
        }

        Board {
            snails,
            scale,
            goal,
        }
    }

    pub fn advance(&mut self, colour: Colour) {
        self.snails.get_mut(&colour).map(|s| *s += 1);
    }

    pub fn winners(&self) -> Vec<Colour> {
        self.snails
            .iter()
            .filter(|s| *s.1 >= self.goal)
            .map(|s| *s.0)
            .collect()
    }

    pub fn draw(&self, window: &Window) {
        for y in 0..6 {
            window.attron(COLOR_PAIR(0));
            window.mvaddch(y, self.scale, '|');
            window.mvaddch(y, self.goal * self.scale, '|');

            let colour = y.try_into().unwrap();
            window.with_colour_pair(y as u64, || {
                window.mvaddstr(
                    y,
                    self.snails[&colour] * self.scale,
                    &repeat('@').take(self.scale as usize).collect::<String>(),
                );
            });
        }

        for x in 0..self.goal + 1 {
            window.mvaddstr(6, x * self.scale, &format!("{}", x));
        }
    }
}
