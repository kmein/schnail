#![feature(try_from)]
extern crate pancurses;
extern crate rand;

use pancurses::{Window, COLOR_PAIR};
use rand::{Rand, Rng};
use std::collections::HashMap;
use std::iter::repeat;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy)]
pub enum Color {
    Red,
    Yellow,
    Green,
    Pink,
    Blue,
    Orange,
}

impl TryFrom<i32> for Color {
    type Error = ();
    fn try_from(code: i32) -> Result<Self, Self::Error> {
        match code {
            0 => Ok(Color::Red),
            1 => Ok(Color::Yellow),
            2 => Ok(Color::Green),
            3 => Ok(Color::Pink),
            4 => Ok(Color::Blue),
            5 => Ok(Color::Orange),
            _ => Err(()),
        }
    }
}

impl Into<u64> for Color {
    fn into(self) -> u64 {
        self as u64
    }
}

impl Rand for Color {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        use Color::*;
        *rng.choose(&[Red, Yellow, Green, Pink, Blue, Orange]).unwrap()
    }
}

pub trait WindowExt {
    fn with_color_pair<I, F>(&self, color: I, action: F)
    where
        F: Fn(),
        I: Into<u64>;
}

impl WindowExt for Window {
    fn with_color_pair<I, F>(&self, color: I, action: F)
    where
        F: Fn(),
        I: Into<u64>,
    {
        let pair = COLOR_PAIR(color.into());
        self.attron(pair);
        action();
        self.attroff(pair);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Board {
    scale: i32,
    goal: i32,
    snails: HashMap<Color, i32>,
}

impl Board {
    pub fn new(goal: i32) -> Self {
        use Color::*;

        let scale = (goal as f32).log10() as i32 + 2;

        let mut snails = HashMap::new();
        for &color in &[Red, Yellow, Green, Pink, Blue, Orange] {
            snails.insert(color, 0);
        }

        Board {
            snails,
            scale,
            goal,
        }
    }

    pub fn advance(&mut self, color: Color) {
        self.snails.get_mut(&color).map(|s| *s += 1);
    }

    pub fn winners(&self) -> Vec<Color> {
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

            let color = y.try_into().unwrap();
            window.with_color_pair(y as u64, || {
                window.mvaddstr(
                    y,
                    self.snails[&color] * self.scale,
                    &repeat('@').take(self.scale as usize).collect::<String>(),
                );
            });
        }

        for x in 0..self.goal + 1 {
            window.mvaddstr(6, x * self.scale, &format!("{}", x));
        }
    }
}
