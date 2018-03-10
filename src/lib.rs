#![feature(try_from)]
extern crate pancurses;
extern crate rand;

use pancurses::{Window, COLOR_PAIR};
use std::string::ToString;
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
        *rng.choose(&[Red, Yellow, Green, Pink, Blue, Orange])
            .unwrap()
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

fn get_scale(goal: i32) -> i32 {
    (goal as f32).log10() as i32 + 2
}

#[derive(Debug, Default, Clone)]
pub struct Display {
    screen_hw: (i32, i32),
    board_hw: (i32, i32),
    pub board: Board,
}

impl Display {
    pub fn new(board: Board, window: &Window) -> Self {
        let goal = board.goal;
        Display {
            screen_hw: window.get_max_yx(),
            board_hw: (6, goal * get_scale(goal)),
            board,
        }
    }

    fn hw_offset(&self) -> (i32, i32) {
        let (screen_h, screen_w) = self.screen_hw;
        let (board_h, board_w) = self.board_hw;
        ((screen_h - board_h) / 2, (screen_w - board_w) / 2)
    }

    pub fn add_str<S: ToString>(&self, window: &Window, y: i32, x: i32, value: S) {
        let (y_offset, x_offset) = self.hw_offset();
        window.mvaddstr(y_offset + y, x_offset + x, &value.to_string());
    }

    pub fn draw(&self, window: &Window) {
        for y in 0..6 {
            window.attron(COLOR_PAIR(0));
            self.add_str(window, y, self.board.scale, '|');
            self.add_str(window, y, self.board.scale * self.board.goal, '|');

            let color = y.try_into().unwrap();
            window.with_color_pair(y as u64, || {
                self.add_str(
                    window,
                    y,
                    self.board.snails[&color] * self.board.scale,
                    &repeat('@')
                        .take(self.board.scale as usize)
                        .collect::<String>(),
                );
            });
        }

        for x in 0..self.board.goal + 1 {
            self.add_str(window, 6, x * self.board.scale, &format!("{}", x));
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Board {
    goal: i32,
    snails: HashMap<Color, i32>,
    scale: i32,
}

impl Board {
    pub fn new(goal: i32) -> Self {
        use Color::*;

        let mut snails = HashMap::new();
        for &color in &[Red, Yellow, Green, Pink, Blue, Orange] {
            snails.insert(color, 0);
        }

        Board {
            goal,
            snails,
            scale: get_scale(goal),
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
}
