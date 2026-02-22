#![allow(clippy::unused_self)]
use crate::types::defs::*;

impl Coord {
    pub fn xy(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn ay(a: char, y: usize) -> Option<Self> {
        let x = a as usize - 96; // a goes to 1, b goes to 2 ...
        if
        /*0 > x ||*/
        x > 8 {
            // Wrong coord
            // print!("Wrong coord!x:{};y:{},a:{}", x, y, a);
            // std::process::exit(1);
            return None;
        }
        Some(Self { x, y })
    }
    pub fn new(move_str: &str) -> Option<Self> {
        let Some(first_char) = move_str.chars().nth(0) else {
            eprintln!("Too short move sent: {move_str}");
            std::process::exit(1);
        };
        let Some(second_char) = move_str.chars().nth(1) else {
            eprintln!("Too short move sent: {move_str}");
            std::process::exit(1);
        };
        // let Some(to_return) = Self::ay(first_char, second_char as usize - 48) else {
        //     return None;
        // };
        let to_return = Self::ay(first_char, second_char as usize - 48)?;
        Some(to_return)
        // Self::ay(first_char, second_char as usize - 48)
    }
    pub fn zero_indexed(&self) -> (usize, usize) {
        (self.x - 1, self.y - 1)
    }
    #[allow(unused)]
    pub fn as_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }
    #[allow(unused)]
    pub fn one_d_coord(&self) -> usize {
        self.zero_indexed().0 + self.zero_indexed().1 * 8
    }
    pub fn manhattan(&self, other: Self) -> usize {
        // println!("x: {}", self.x.abs_diff(other.x));
        // println!("y: {}", self.y.abs_diff(other.y));
        // println!("self: {self}");
        // println!("other: {other}");
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

use std::fmt;
impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        const ALPHABET: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let Some(a) = ALPHABET.get(self.x - 1) else {
            unreachable!()
        };
        let y = format!("{}", self.y).to_string().chars().next().unwrap(); // This won't fail. Surely?
        buf.push(*a);
        buf.push(y);
        write!(f, "{buf}")
    }
}
