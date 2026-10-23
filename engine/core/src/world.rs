use std::collections::HashMap;

use serde::Serialize;

use crate::data::*;
use crate::hex::Hex;
use crate::market::{self, utility, Basket, Params};
use crate::rng::{rand, rand_index};

#[derive(Clone, Debug, Serialize)]
pub struct Tile {
    pub q: i32,
    pub r: i32,
    pub terrain: usize,
}

impl Tile {
    pub fn hex(&self) -> Hex {
        Hex { q: self.q, r: self.r }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Clan {
    pub id: usize,
    pub tile: usize,
    pub people: f64,
    pub market: Option<usize>,
    /// What the clan gathered this turn.
    pub harvest: Basket,
    /// What it holds after trading.
    pub basket: Basket,
}

#[derive(Clone, Debug, Serialize)]
pub struct Market {
    pub id: usize,
    pub tile: usize,
    /// Last real exchange ratios; kept when a turn doesn't clear.
    pub prices: Basket,
    pub volume: Basket,
    pub traders: usize,
    pub cleared: bool,
    pub iterations: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct TurnStats {
    pub turn: u32,
    pub season: Vec<f64>,
    pub prices: Vec<Basket>,
    pub volume: Vec<Basket>,
    /// Σ utility of every clan's harvest, as if nobody traded.
    pub autarky: f64,
    /// Σ utility after trading.
    pub traded: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct World {
    pub config: Config,
    pub turn: u32,
    pub rng: u32,
    pub tiles: Vec<Tile>,
    pub clans: Vec<Clan>,
    pub markets: Vec<Market>,
    pub season: Vec<f64>,
    pub history: Vec<TurnStats>,
}

impl World {
    pub fn new(config: Config) -> World {
        let mut rng = config.seed;
        let hexes = Hex::ORIGIN.within(config.radius);
        let regions: Vec<(Hex, usize)> = (0..REGIONS)
            .map(|_| {
                let h = hexes[rand_index(&mut rng, hexes.len())];
                (h, REGION_TERRAIN[rand_index(&mut rng, REGION_TERRAIN.len())])
            })
            .collect();
        let mut tiles: Vec<Tile> = hexes
            .iter()
            .map(|&h| {
                let terrain = if h.distance(Hex::ORIGIN) == config.radius {
                    SEA
                } else {
                    regions.iter().min_by_key(|(c, _)| c.distance(h)).unwrap().1
                };
                Tile { q: h.q, r: h.r, terrain }
            })
            .collect();
        let index: HashMap<Hex, usize> = tiles.iter().enumerate().map(|(i, t)| (t.hex(), i)).collect();
        let shore: Vec<usize> = (0..tiles.len())
            .filter(|&i| {
                TERRAIN[tiles[i].terrain].land
                    && tiles[i].hex().neighbors().any(|n| index.get(&n).is_some_and(|&j| tiles[j].terrain == SEA))
            })
            .collect();
        for i in shore {
            tiles[i].terrain = COAST;
        }

        let mut land: Vec<usize> = (0..tiles.len()).filter(|&i| TERRAIN[tiles[i].terrain].land).collect();
        let mut markets: Vec<Market> = Vec::new();
        if !land.is_empty() {
            let mut sites = vec![land[rand_index(&mut rng, land.len())]];
            while sites.len() < config.markets.min(land.len()) {
                let far = |&i: &usize| sites.iter().map(|&m| tiles[m].hex().distance(tiles[i].hex())).min().unwrap();
                sites.push(*land.iter().max_by_key(|i| far(i)).unwrap());
            }
            markets = sites
                .into_iter()
                .enumerate()
                .map(|(id, tile)| Market {
                    id,
                    tile,
                    prices: [1.0; N],
                    volume: [0.0; N],
                    traders: 0,
                    cleared: false,
                    iterations: 0,
                })
                .collect();
        }

        for i in (1..land.len()).rev() {
            land.swap(i, rand_index(&mut rng, i + 1));
        }
        let clans = land
            .iter()
            .take(config.clans)
            .enumerate()
            .map(|(id, &tile)| {
                let home = tiles[tile].hex();
                let market = markets
                    .iter()
                    .map(|m| (home.distance(tiles[m.tile].hex()), m.id))
                    .filter(|&(d, _)| d <= config.reach)
                    .min()
                    .map(|(_, id)| id);
                let people = (PEOPLE.0 + (PEOPLE.1 - PEOPLE.0) * rand(&mut rng)).round();
                Clan { id, tile, people, market, harvest: [0.0; N], basket: [0.0; N] }
            })
            .collect();

        World {
            config,
            turn: 0,
            rng,
            tiles,
            clans,
            markets,
            season: vec![1.0; TERRAIN.len()],
            history: Vec::new(),
        }
    }

    /// Season → harvest → trade → stats.
    pub fn step(&mut self) {
        let World { tiles, clans, markets, season, rng, .. } = self;
        self.turn += 1;
        for s in season.iter_mut() {
            *s = SEASON.0 + (SEASON.1 - SEASON.0) * rand(rng);
        }

        for clan in clans.iter_mut() {
            let home = tiles[clan.tile].hex();
            let mut h = [0.0; N];
            let mut n = 0.0;
            for t in tiles.iter().filter(|t| TERRAIN[t.terrain].land && t.hex().distance(home) <= FORAGE) {
                for (g, y) in TERRAIN[t.terrain].yields.iter().enumerate() {
                    h[g] += y * season[t.terrain];
                }
                n += 1.0;
            }
            for v in h.iter_mut() {
                *v *= clan.people / n;
            }
            clan.harvest = h;
            clan.basket = h;
        }

        for m in markets.iter_mut() {
            let members: Vec<usize> = clans.iter().filter(|c| c.market == Some(m.id)).map(|c| c.id).collect();
            let baskets: Vec<Basket> = members.iter().map(|&id| clans[id].harvest).collect();
            let c = market::clear(&baskets, &m.prices, &Params::default());
            m.prices = c.prices;
            m.volume = c.volume;
            m.traders = members.len();
            m.cleared = c.cleared;
            m.iterations = c.iterations;
            for (&id, b) in members.iter().zip(c.after) {
                clans[id].basket = b;
            }
        }

        self.history.push(TurnStats {
            turn: self.turn,
            season: season.clone(),
            prices: markets.iter().map(|m| m.prices).collect(),
            volume: markets.iter().map(|m| m.volume).collect(),
            autarky: clans.iter().map(|c| utility(&c.harvest)).sum(),
            traded: clans.iter().map(|c| utility(&c.basket)).sum(),
        });
    }

    pub fn run(&mut self, turns: u32) {
        for _ in 0..turns {
            self.step();
        }
    }
}
