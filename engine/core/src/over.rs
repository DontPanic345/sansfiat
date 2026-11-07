//! How a game ends. The game is `Phase::Over` once the led civ's capital has fallen (`Fell`), once
//! one capital is the last standing (`Conquest`), or once the turns since the pick reach the limit
//! (`Limit`). The `over` step is the last of `TURN`: it writes the turn's Standing and makes the end.
//! The result is fixed at `Over`; the world runs on after it.

use serde::{Deserialize, Serialize};

use crate::civs;
use crate::data::END;
use crate::events::{Event, EventKind};
use crate::stats;
use crate::world::{Phase, World};

/// The end of a game: who won, why, and the turns since the pick it came at.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Over {
    pub winner: u32,
    pub why: Why,
    pub turn: u32,
}

/// Why a game ended. `Fell`'s `by` is the civ that took the led civ's capital.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Why {
    Conquest,
    Limit,
    Fell { by: u32 },
}

/// The turn limit in force for `w`, in turns after the pick: the config's, else the size's from
/// `END`. Read in any phase, since the pick's cards show it.
pub fn limit(w: &World) -> u32 {
    if let Some(limit) = w.config.limit {
        return limit;
    }
    let radius = w.config.radius;
    END.limit.iter().find(|(r, _)| radius <= *r).or(END.limit.last()).map_or(0, |&(_, turns)| turns)
}

/// The world turn the game's limit falls on, `None` before the pick.
pub fn ends_at(w: &World) -> Option<u32> {
    w.picked.map(|p| p + limit(w))
}

/// The civ with the highest Standing, a tie to the lower id: among the civs standing, or among all
/// when none is. `None` in a world with no civ.
pub fn winner_by_standing(w: &World) -> Option<u32> {
    let standing = stats::standing(w);
    let standing_civs: Vec<u32> = civs::alive(w).map(|c| c.id).collect();
    let pool = if standing_civs.is_empty() { w.civs.keys().copied().collect() } else { standing_civs };
    let at = |c: &u32| standing.get(c).copied().unwrap_or(0.0);
    pool.into_iter().max_by(|a, b| at(a).total_cmp(&at(b)).then(b.cmp(a)))
}

/// The led civ's fall, if it has fallen: the taker.
fn led_fall(w: &World) -> Option<u32> {
    let me = w.player?;
    w.civs.get(&me)?.fallen.map(|(_, by)| by)
}

/// The end this turn calls for, if any, checked in order: a led civ's fall, then the last capital,
/// then the limit. `turns` is the turns since the pick.
fn ending(w: &World, turns: u32) -> Option<Over> {
    if let Some(by) = led_fall(w) {
        return Some(Over { winner: winner_by_standing(w)?, why: Why::Fell { by }, turn: turns });
    }
    let founded = w.civs.len();
    let standing: Vec<u32> = civs::alive(w).map(|c| c.id).collect();
    if founded >= 2 && standing.len() <= 1 {
        let winner = winner_by_standing(w)?;
        return Some(Over { winner, why: Why::Conquest, turn: turns });
    }
    if turns >= limit(w) {
        return Some(Over { winner: winner_by_standing(w)?, why: Why::Limit, turn: turns });
    }
    None
}

/// The `over` step, last in `TURN`: after the pick, pushes the turn's Standing (`World::standing`),
/// then, in `Main`, ends the game if `ending` calls for it. Runs before `World.turn` advances, so the
/// turns since the pick are `w.turn + 1 - picked`.
pub fn step(w: &mut World) {
    let Some(picked) = w.picked else { return };
    let standing = stats::standing(w);
    w.standing.push(standing);
    if matches!(w.phase, Phase::Over(_)) {
        return;
    }
    let turns = w.turn + 1 - picked;
    let Some(over) = ending(w, turns) else { return };
    let (tile, civ) = match over.why {
        Why::Fell { .. } => (w.civs[&w.player.expect("a led civ fell")].capital, over.winner),
        _ => (w.civs[&over.winner].capital, over.winner),
    };
    w.phase = Phase::Over(over);
    w.emit(Event::new(EventKind::GameOver, tile).civ(civ).other(match over.why {
        Why::Fell { by } => Some(by),
        _ => None,
    }));
    let name = |id: u32| w.civs.get(&id).map_or_else(|| format!("civ {id}"), |c| c.name.clone());
    let why = match over.why {
        Why::Conquest => "the last capital standing".to_string(),
        Why::Limit => "the highest Standing at the turn limit".to_string(),
        Why::Fell { by } => format!("the led civ's capital fell to {}", name(by)),
    };
    let winner = name(over.winner);
    w.note(over.winner, format!("won the game: {why}"));
    if let Some(me) = w.player.filter(|&me| me != over.winner) {
        w.note(me, format!("the game ended with {winner} winning: {why}"));
    }
}
