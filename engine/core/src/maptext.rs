//! The text of a map, the inverse of `map::ascii`. `read` reads the rows of a grid of characters,
//! one per tile, for the map and for the layers over it; `parse` reads a map's glyphs into bare
//! tiles. The `map` command's output parses as printed: the legend and the counts after the rows
//! are ignored.

use crate::data::{COAST, DESERT, FOREST, HILLS, MARSH, MOUNTAIN, PLAINS, SEA};
use crate::hex::Hex;
use crate::map::{generate, index, Country, Tile};

/// A map read from its text: its radius and its tiles, bare (see `bare`).
pub struct Parsed {
    pub radius: i32,
    pub tiles: Vec<Tile>,
}

/// One character of a grid and where it stands in the text: its tile index, and its row and
/// column as the text numbers them (from 1).
struct Cell {
    c: char,
    i: usize,
    row: usize,
    col: usize,
}

/// Reads a grid of `text`: the radius, and one character per tile, by tile index. Each row has one
/// character a tile with single spaces between, indented as `map::ascii` indents it.
pub fn grid(text: &str) -> Result<(i32, Vec<char>), String> {
    let (radius, cells) = read(text)?;
    let mut out = vec![' '; cells.len()];
    for cell in cells {
        out[cell.i] = cell.c;
    }
    Ok((radius, out))
}

/// Reads a map's text: its radius and its tiles. A glyph is `map::glyph`'s, the inverse of its
/// table: `~` sea, `=` a lake, `,` coast, `.` plains, `f` forest, `h` hills, `^` mountain, `:`
/// desert, `%` marsh; `r` plains with a river, `z` a floodplain, `P` a pass, and `V`, `S`, `N` the
/// valley, sealed valley and neck. The text does not say the terrain under those last glyphs, so
/// they are plains.
pub fn parse(text: &str) -> Result<Parsed, String> {
    let (radius, cells) = read(text)?;
    let mut tiles = bare(radius);
    for cell in cells {
        if !set_glyph(&mut tiles[cell.i], cell.c) {
            return Err(format!("row {}, column {}: {:?} is not a map glyph", cell.row, cell.col, cell.c));
        }
    }
    for t in &mut tiles {
        t.trail_cost = crate::land::flat_cost(t.terrain);
    }
    Ok(Parsed { radius, tiles })
}

/// The tiles of a map of `radius` as a fresh world has them, with nothing on them: plains and sea
/// are for the caller to set. Every tile's food, wear, claim, trail, river, flow, lake, flood,
/// country, pass, hidden cost and span is clear, so a parsed map carries only its glyphs.
pub(crate) fn bare(radius: i32) -> Vec<Tile> {
    let mut rng = 1;
    let mut tiles = generate(radius, &mut rng);
    for t in &mut tiles {
        t.terrain = PLAINS;
        t.cap = [0.0; crate::data::N];
        t.stock = [0.0; crate::data::N];
        t.unworn = [0.0; crate::data::N];
        t.wear = [0.0; crate::data::N];
        t.take = [0.0; crate::data::N];
        t.wild_grain = 0.0;
        t.tame = 0;
        t.claim = None;
        t.traffic = 0.0;
        t.used = false;
        t.claim_run = 0;
        t.free_run = 0;
        t.trail = false;
        t.river = 0;
        t.flow = None;
        t.lake = false;
        t.flood = false;
        t.country = Country::Open;
        t.pass = false;
        t.trail_cost = None;
        t.span = None;
    }
    tiles
}

/// Sets `t` as glyph `c` shows it. `false` for a character that is no glyph.
fn set_glyph(t: &mut Tile, c: char) -> bool {
    match c {
        '~' => t.terrain = SEA,
        '=' => {
            t.terrain = SEA;
            t.lake = true;
        }
        ',' => t.terrain = COAST,
        '.' => t.terrain = PLAINS,
        'f' => t.terrain = FOREST,
        'h' => t.terrain = HILLS,
        '^' => t.terrain = MOUNTAIN,
        ':' => t.terrain = DESERT,
        '%' => t.terrain = MARSH,
        'r' => {
            t.terrain = PLAINS;
            t.river = 1;
        }
        'z' => {
            t.terrain = PLAINS;
            t.flood = true;
        }
        'V' | 'S' | 'N' => {
            t.terrain = PLAINS;
            t.country = match c {
                'V' => Country::Valley,
                'S' => Country::Sealed,
                _ => Country::Neck,
            };
        }
        'P' => {
            t.terrain = PLAINS;
            t.pass = true;
        }
        _ => return false,
    }
    true
}

/// Reads the rows of `text`: the radius, from the first non-blank row's glyph count, and the cells
/// of the `2·radius + 1` rows after it. Blank lines before them are skipped, an indent common to
/// the rows is removed, and anything after the last row is ignored.
fn read(text: &str) -> Result<(i32, Vec<Cell>), String> {
    let lines: Vec<&str> = text.lines().skip_while(|l| l.trim().is_empty()).collect();
    let first = lines.first().ok_or("the map has no rows")?;
    let radius = first.split_whitespace().count() as i32 - 1;
    let rows = (2 * radius + 1) as usize;
    if lines.len() < rows {
        return Err(format!("a map of radius {radius} has {rows} rows; the text has {}", lines.len()));
    }
    let rows = &lines[..rows];
    let indent = rows.iter().map(|l| l.len() - l.trim_start().len()).min().unwrap_or(0);
    let mut cells = Vec::new();
    for (y, line) in rows.iter().enumerate() {
        let r = y as i32 - radius;
        let lo = (-radius).max(-r - radius);
        let hi = radius.min(-r + radius);
        let body = line.get(indent..).unwrap_or("");
        let glyphs: Vec<&str> = body.split_whitespace().collect();
        let needed = (hi - lo + 1) as usize;
        if glyphs.len() != needed {
            let col = glyphs.len().min(needed) + 1;
            let what = if glyphs.len() < needed { "is missing" } else { "is extra" };
            return Err(format!(
                "row {} (r = {r}): {} glyphs where {needed} are needed; column {col} {what}",
                y + 1,
                glyphs.len()
            ));
        }
        for (k, token) in glyphs.iter().enumerate() {
            let mut chars = token.chars();
            let (Some(c), None) = (chars.next(), chars.next()) else {
                return Err(format!("row {} (r = {r}), column {}: {token:?} is not one glyph", y + 1, k + 1));
            };
            let i = index(radius, Hex { q: lo + k as i32, r }).expect("a tile of the row");
            cells.push(Cell { c, i, row: y + 1, col: k + 1 });
        }
    }
    Ok((radius, cells))
}
