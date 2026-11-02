//! `story`: the three stories of war (`docs/design/war.md`) played as scripts on a retro map. A story
//! runs a new game to the pick, leads one civ, and each turn gives that civ's orders through
//! `World::act` (so the save is a normal save, and `judge` and `play --game` take it), reading the
//! world for its units, its tiles and the nearest rival by path. The AI plays the rest. The story's
//! functions are written the way `core/src/script.rs`'s `Lead` is: what each does on which turn is
//! in its doc comment.
//!
//! Every story sets gift 0 and keeps its guard by the AI's own rule (`units::guard_order`), and none
//! leaves its camp at the capital beside the guard: with every store at 0 at the pick and the levy
//! bringing about three or four food a turn, a camp resupplied from the store at home and the guard
//! together outeat it (`relay/specs/104/spec-review.md`).

use std::fs;

use engine::actors::walk_cost;
use engine::data::{Fighter, SIZES, UNIT};
use engine::market::total;
use engine::{civs, claim, land, stats, units, Action, Config, Order, UnitKind, World};

use crate::retro::to_pick;
use crate::{size, USAGE};

/// A story's default map: the size, the seed, the civ (1-based, in id order) and the turns.
struct Plan {
    size: &'static str,
    seed: u32,
    civ: u32,
    turns: u32,
}

/// The stories' names and their default maps.
const SKIRMISH: Plan = Plan { size: "small", seed: 2, civ: 1, turns: 80 };
const STACK: Plan = Plan { size: "small", seed: 2, civ: 1, turns: 80 };
const SURRENDER: Plan = Plan { size: "small", seed: 2, civ: 1, turns: 60 };

/// A story in progress: the world, the civ it leads and the nearest rival by path.
struct Story {
    w: World,
    me: u32,
    rival: u32,
    /// The turn the story began (the pick).
    pick: u32,
    trace: bool,
    /// Orders refused so far: the turn since the pick, the order and the reason.
    refused: Vec<(u32, String, String)>,
}

impl Story {
    /// The turns since the pick.
    fn n(&self) -> u32 {
        self.w.turn - self.pick
    }

    /// Gives the led civ an order through `act`. A refusal is recorded and returns false.
    fn give(&mut self, order: Order) -> bool {
        let text = serde_json::to_string(&order).unwrap_or_default();
        match self.w.act(Action::Order { civ: self.me, order }) {
            Ok(()) => true,
            Err(e) => {
                let n = self.n();
                self.refused.push((n, text, e));
                false
            }
        }
    }

    /// The led civ's first army, if it has one.
    fn camp(&self) -> Option<&engine::Unit> {
        units::camp(&self.w, self.me)
    }

    /// The led civ's capital.
    fn capital(&self) -> usize {
        self.w.civs[&self.me].capital
    }

    /// The rival's capital.
    fn rival_capital(&self) -> usize {
        self.w.civs[&self.rival].capital
    }

    /// The walk from the led civ's capital to the rival's, the capital left out.
    fn way(&self) -> Vec<usize> {
        units::path_between(&self.w, self.capital(), self.rival_capital()).map(|(p, _)| p).unwrap_or_default()
    }

    /// The last tile of the way to the rival whose line is not cut for an army of `people`: the farthest
    /// tile of the way from which a walk to the capital carries a turn's food (`line`'s rule).
    fn stage_tile(&self, people: f64) -> usize {
        let cap = self.capital();
        let ok = |t: usize| {
            units::path_between(&self.w, t, cap).is_some_and(|(_, c)| {
                let cost = walk_cost(c, 1.0);
                cost <= 0.0 || people * UNIT.labour * UNIT.carry / cost + 1e-9 >= people * UNIT.food
            })
        };
        let way = self.way();
        way.iter().take_while(|&&t| ok(t)).last().copied().unwrap_or(cap)
    }

    /// Marches the first army to the stage tile and waits there until its pack is full, then pushes on
    /// to `target`, both switches on. Once it is past the stage it is only pushed, never called back.
    fn stage_then_push(&mut self, target: usize) {
        let Some((id, home, people, pack, marching, key, still)) = self.camp().map(|u| {
            let c = u.camp().expect("an army");
            (u.id, u.home, u.people, total(&u.store), c.dest.is_some(), units::pack_cap(u.people), c.still)
        }) else {
            return;
        };
        let stage = self.stage_tile(people);
        let way = self.way();
        let at = way.iter().position(|&t| t == home);
        let past = at.zip(way.iter().position(|&t| t == stage)).is_some_and(|(a, b)| a > b);
        if marching || home == target {
            return;
        }
        if self.n() == 0 {
            self.give(Order::Switches { unit: id, forage: true, supply: true });
        }
        if !past && home != stage {
            self.give(Order::Move { unit: id, to: stage });
        } else if pack + 1e-6 >= key - UNIT.food || still >= WAIT {
            self.give(Order::Move { unit: id, to: target });
        }
    }

    /// The warriors the store fills the packs of, and the clans could give.
    fn fillable(&self) -> u32 {
        let packs = (total(&self.w.civs[&self.me].store) / units::pack_cap(1.0)).floor() as u32;
        packs.min(units::givers(&self.w, self.me).len() as u32)
    }

    /// Raises every warrior the store fills at once into one army (the first raise makes it at the
    /// capital, the rest join it), both switches on. Returns how many were raised.
    fn raise_all(&mut self) -> u32 {
        let mut raised = 0;
        let mut into = self.camp().map(|u| u.id);
        loop {
            let n = self.fillable();
            if n == 0 || !self.give(Order::Raise { n, kind: UnitKind::Camp, fighter: Fighter::Warriors, into }) {
                break;
            }
            raised += n;
            into = self.camp().map(|u| u.id);
        }
        raised
    }

    /// Keeps the led civ's guard by the AI's own rule, once a turn.
    fn keep_guard(&mut self) {
        if let Some(order) = units::guard_order(&self.w, self.me) {
            self.give(order);
        }
    }

    /// Ends the turn: the guard is kept first.
    fn end_turn(&mut self) -> Result<(), String> {
        self.keep_guard();
        self.w.act(Action::EndTurn)
    }

    /// A line for `--trace`: the turn, the led civ's store, its armies, its guard, and the fight so far.
    fn trace_line(&self) {
        let w = &self.w;
        let st = stats::standing(w);
        let armies: Vec<String> = w
            .units
            .values()
            .filter(|u| u.civ == self.me && u.kind() == UnitKind::Camp)
            .map(|u| {
                format!(
                    "army {} at {} ({} steps out): {:.0}+{:.0} pack {:.1} hungry {}{}",
                    u.id,
                    u.home,
                    land::steps(w, self.capital(), u.home),
                    u.camp().map_or(0.0, |c| c.stack[0]),
                    u.camp().map_or(0.0, |c| c.stack[1]),
                    total(&u.store),
                    u.hungry,
                    if u.camp().is_some_and(|c| c.cut) { " cut" } else { "" }
                )
            })
            .collect();
        let (me, rv) = (&w.civs[&self.me], &w.civs[&self.rival]);
        println!(
            "+{} store {:.1} guard {:.1} claim {} held {}; rival guard {:.1} held {}; contested {}; fallen {:.2}/{:.2} taken {}/{}; standing {:.3}/{:.3}; {}",
            self.n(),
            total(&me.store),
            claim::guard_people(w, self.me),
            me.claim.len(),
            claim::held(w, self.me).len(),
            claim::guard_people(w, self.rival),
            claim::held(w, self.rival).len(),
            w.contested.len(),
            me.army.fallen,
            rv.army.fallen,
            me.army.taken,
            rv.army.taken,
            st.get(&self.me).copied().unwrap_or(0.0),
            st.get(&self.rival).copied().unwrap_or(0.0),
            armies.join("; "),
        );
    }
}

/// The name of the size with this radius.
fn size_name(radius: i32) -> &'static str {
    SIZES.iter().find(|s| s.1 == radius).map_or("custom", |s| s.0)
}

/// The civ `k` (1-based, in id order).
fn civ_id(w: &World, k: u32) -> Result<u32, String> {
    let ids: Vec<u32> = w.civs.keys().copied().collect();
    ids.get((k as usize).wrapping_sub(1))
        .copied()
        .ok_or_else(|| format!("--civ {k}: there are {} civs, 1 to {}", ids.len(), ids.len()))
}

pub(crate) fn story(args: &[String]) -> Result<(), String> {
    let name = args.first().ok_or(format!("story needs a name\n{USAGE}"))?.as_str();
    let (plan, script): (&Plan, fn(&mut Story)) = match name {
        "skirmish" => (&SKIRMISH, skirmish),
        "stack" => (&STACK, stack),
        "surrender" => (&SURRENDER, surrender),
        _ => return Err(format!("unknown story {name}: skirmish, stack or surrender\n{USAGE}")),
    };
    let (mut seed, mut radius, mut civ, mut turns, mut save) =
        (plan.seed, size(plan.size)?, plan.civ, plan.turns, None);
    let (mut fight, mut trace) = (true, false);
    let mut it = args[1..].iter();
    while let Some(flag) = it.next() {
        match flag.as_str() {
            "--no-fight" => fight = false,
            "--trace" => trace = true,
            "--seed" | "--size" | "--civ" | "--turns" | "--save" => {
                let value = it.next().ok_or(format!("{flag} needs a value\n{USAGE}"))?;
                let bad = |_| format!("{flag}: not a number: {value}");
                match flag.as_str() {
                    "--seed" => seed = value.parse().map_err(bad)?,
                    "--size" => radius = size(value)?,
                    "--civ" => civ = value.parse().map_err(bad)?,
                    "--turns" => turns = value.parse().map_err(bad)?,
                    _ => save = Some(value.clone()),
                }
            }
            _ => return Err(format!("unknown flag {flag}\n{USAGE}")),
        }
    }
    let save = save.ok_or(format!("story needs --save FILE\n{USAGE}"))?;

    let mut w = to_pick(Config { seed, radius, fight, ..Config::default() });
    let me = civ_id(&w, civ)?;
    w.act(Action::ChooseCiv { civ: Some(me) })?;
    w.act(Action::SetGift { food: 0.0 })?;
    let cap = w.civs[&me].capital;
    let rival = civs::alive(&w)
        .filter(|c| c.id != me)
        .min_by_key(|c| (land::steps(&w, cap, c.capital), c.id))
        .map(|c| c.id)
        .ok_or("there is no rival to fight")?;
    let pick = w.turn;
    let mut s = Story { w, me, rival, pick, trace, refused: Vec::new() };
    for _ in 0..turns {
        script(&mut s);
        s.end_turn()?;
        if s.trace {
            s.trace_line();
        }
    }
    fs::write(&save, serde_json::to_string(&s.w.save()).unwrap()).map_err(|e| format!("{save}: {e}"))?;
    let out = civs::alive(&s.w).count();
    println!(
        "{name} on {} seed {seed}: led civ {} against {}, {turns} turns past the pick ({} civs alive), saved to {save}",
        size_name(radius),
        s.w.civs[&me].name,
        s.w.civs[&s.rival].name,
        out
    );
    for (n, order, why) in &s.refused {
        println!("  refused at +{n}: {order} ({why})");
    }
    Ok(())
}

/// **skirmish.** The led civ claims a corridor along the path to the nearest rival's capital, out
/// until it overlaps the rival's claim by two tiles, and keeps it; its army is posted on its own land
/// off the capital.
///
/// - +0: the army goes to the AI's post (`units::post`: up to `UNIT.post` steps along that path, on its
///   own land), off the capital and beside no guard.
/// - Every turn: the claim is the stand-in (`claim::ai_claim`) with the corridor, sent whenever it
///   differs from the claim (a tile lost to fighting is claimed again).
fn skirmish(s: &mut Story) {
    let way = s.way();
    let theirs = claim::ai_claim(&s.w, s.rival);
    let first = way.iter().position(|t| theirs.contains(t)).unwrap_or(way.len());
    let stand = way[first.saturating_sub(1)];
    bank_and_raise(s, 3, 30);
    s.stage_then_push(stand);
    if let Some(id) = s.camp().filter(|u| u.home == stand && u.territory.len() == 1).map(|u| u.id) {
        let zone: Vec<usize> = way[first..].iter().copied().take(2).collect();
        s.give(Order::Zone { unit: id, tiles: zone });
    }
    let want = corridor_claim(&s.w, s.me, s.rival);
    if want != s.w.civs[&s.me].claim {
        s.give(Order::Claim { tiles: want });
    }
}

/// +0: the starting army is disbanded to its clans, so that it doesn't eat the store beside the
/// guard. Then the store banks until it fills `want` warriors' packs (or turn `by`), and every
/// warrior it fills is raised at once into one army.
fn bank_and_raise(s: &mut Story, want: u32, by: u32) {
    if s.n() == 0 {
        if let Some(id) = s.camp().map(|u| u.id) {
            s.give(Order::Disband { unit: id, n: None, fighter: None });
        }
    }
    if s.camp().is_none() && s.n() > 0 && (s.fillable() >= want || s.n() >= by) {
        s.raise_all();
    }
}

/// The stand-in claim and the corridor: the way to the rival's capital out to two tiles inside its
/// claim, the rival's capital left out.
fn corridor_claim(w: &World, me: u32, rival: u32) -> Vec<usize> {
    let (cap, theirs) = (w.civs[&me].capital, claim::ai_claim(w, rival));
    let way = units::path_between(w, cap, w.civs[&rival].capital).map(|(p, _)| p).unwrap_or_default();
    let first = way.iter().position(|t| theirs.contains(t)).unwrap_or(way.len());
    let end = (first + 2).min(way.len().saturating_sub(1));
    let mut tiles = vec![cap];
    tiles.extend(way[..end].iter().copied());
    tiles.sort_unstable();
    tiles.dedup();
    claim::check(w, me, &tiles).unwrap_or(tiles)
}

/// **stack.** The led civ banks its store, raises every warrior it fills at once into one army and
/// marches it at the nearest rival's capital.
///
/// - +0: the starting army is disbanded to its clans (it would eat the store beside the guard).
/// - Until the store fills `STACK_WANT` warriors' packs, or turn `STACK_BY`: nothing but the guard's
///   upkeep. Then every warrior the store fills is raised at once into one army at the capital.
/// - The army marches to the last tile of the way whose line is not cut (`stage_tile`), waits there
///   until its pack is full (auto-supply fills it from the store) or `WAIT` turns have passed, and then
///   pushes at the rival's capital with both switches on.
fn stack(s: &mut Story) {
    bank_and_raise(s, STACK_WANT, STACK_BY);
    let target = s.rival_capital();
    s.stage_then_push(target);
}

/// Turns an army waits at the stage for its pack before it pushes on regardless.
const WAIT: u32 = 10;
const STACK_WANT: u32 = 3;
const STACK_BY: u32 = 25;

/// **surrender.** The led civ marches its starting army into the rival's land, past its guard, and
/// halts it two tiles inside.
///
/// - +0: the army, both switches on, is sent along the way to the rival's capital to the second tile
///   of the rival's stand-in claim (`claim::ai_claim`) on it, and left there. The rival's stand-in
///   takes back the tiles behind it (it skips only tiles a rival army holds), which cuts the army's
///   line; hungry three turns in a row with its line cut by the rival's land, it surrenders to it.
fn surrender(s: &mut Story) {
    if s.n() > 0 {
        return;
    }
    let way = s.way();
    let theirs = claim::ai_claim(&s.w, s.rival);
    let first = way.iter().position(|t| theirs.contains(t)).unwrap_or(way.len().saturating_sub(2));
    let stop = way[(first + 1).min(way.len().saturating_sub(1))];
    if let Some(id) = s.camp().map(|u| u.id) {
        s.give(Order::Switches { unit: id, forage: true, supply: true });
        s.give(Order::Move { unit: id, to: stop });
    }
}
