//! Axial coordinates, pointy-top.

use serde::Serialize;

pub const DIRS: [(i32, i32); 6] = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
}

impl Hex {
    pub const ORIGIN: Hex = Hex { q: 0, r: 0 };

    pub fn distance(self, o: Hex) -> i32 {
        ((self.q - o.q).abs() + (self.q + self.r - o.q - o.r).abs() + (self.r - o.r).abs()) / 2
    }

    pub fn neighbors(self) -> impl Iterator<Item = Hex> {
        DIRS.into_iter().map(move |(dq, dr)| Hex { q: self.q + dq, r: self.r + dr })
    }

    pub fn within(self, radius: i32) -> Vec<Hex> {
        let mut out = Vec::new();
        for dq in -radius..=radius {
            for dr in (-radius).max(-dq - radius)..=radius.min(-dq + radius) {
                out.push(Hex { q: self.q + dq, r: self.r + dr });
            }
        }
        out
    }
}
