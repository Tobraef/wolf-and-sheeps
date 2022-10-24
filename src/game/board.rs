use std::iter::once;

use super::coord::Coord;

#[derive(Debug, Clone)]
pub struct Board {
    pub wolf: Coord,
    pub sheeps: [Coord; 4],
    pub selected: Option<Coord>,
    pub currently_moving: Species,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    pub from: Coord,
    pub to: Coord,
}

impl Move {
    pub fn new(from: Coord, to: Coord) -> Self {
        Self { from, to }
    }
}

#[derive(Clone)]
pub struct Controls {
    pub wolf_controlled_by: Control,
    pub sheep_controlled_by: Control,
}

#[derive(Clone, Debug)]
pub enum Control {
    Player,
    Computer,
}

#[derive(Clone, Debug)]
pub enum Species {
    Wolf,
    Sheep,
}

impl Board {
    pub fn iter(&self) -> impl Iterator<Item = &Coord> {
        self.sheeps.iter().chain(once(&self.wolf))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Coord> {
        self.sheeps.iter_mut().chain(once(&mut self.wolf))
    }
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            wolf_controlled_by: Control::Player,
            sheep_controlled_by: Control::Player,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            wolf: Coord { x: 3, y: 0 },
            sheeps: [
                Coord { x: 0, y: 7 },
                Coord { x: 2, y: 7 },
                Coord { x: 4, y: 7 },
                Coord { x: 6, y: 7 },
            ],
            selected: None,
            currently_moving: Species::Wolf,
        }
    }
}
