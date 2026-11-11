//! `story`: the three stories of war (`docs/design/war.md`) played as scripts on a retro map. A story
//! runs a new game to the pick, leads one civ, and each turn gives that civ's orders through
//! `World::act` (so the save is a normal save, and `judge` and `play --game` take it), reading the
//! world for its units, its tiles and the nearest rival by path. The AI plays the rest. The story's
//! functions are written the way `core/src/script.rs`'s `Lead` is: what each does on which turn is
//! in its doc comment.
//!
//! Every story sets gift 0 and keeps its guard by the AI's own rule (`units::guard_order`), and none
//! leaves its camp at the capital beside the guard: with every store at 10 grain at the pick (the
//! endowment less the scout's pack, on small 2 for every civ) and the levy bringing about three or four
//! food a turn, a camp resupplied from the store at home and the guard together outeat it
//! (`relay/specs/104/spec-review.md`). A store fills one warrior's pack at the pick, not three.

use std::fs;

use engine::actors::walk_cost;
use engine::data::{Fighter, SIZES, UNIT};
use engine::market::total;
use engine::over::Why;
use engine::{civs, claim, land, stats, units, Action, Config, Order, Phase, UnitKind, World};

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
const CONQUEST: Plan = Plan { size: "small", seed: 2, civ: 1, turns: 0 };

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
    /// What the led civ's armies did: the most warriors in one army, the most steps from its own capital
    /// any army got, and the most warriors it had on a rival's capital at a turn's end.
    reach: Reach,
}

/// The led civ's armies' best: see `Story::reach`.
#[derive(Default)]
struct Reach {
    largest: f64,
    farthest: i32,
    /// The turn since the pick the farthest was first reached.
    farthest_at: u32,
    at_capital: f64,
    /// The turns since the pick an army first stood on a rival's capital and a capital first fell.
    capital_at: Option<u32>,
    fell_at: Option<u32>,
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

    /// Sets the food the led civ keeps in its store.
    fn keep(&mut self, food: f64) {
        let _ = self.w.act(Action::SetKeep { food: Some(food) });
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
        self.w.act(Action::EndTurn)?;
        let n = self.n();
        let w = &self.w;
        if self.reach.fell_at.is_none() && w.civs.values().any(|c| c.is_out()) {
            self.reach.fell_at = Some(n);
        }
        let rivals: Vec<usize> = w.civs.values().filter(|c| c.id != self.me).map(|c| c.capital).collect();
        for u in w.units.values().filter(|u| u.civ == self.me && u.kind() == UnitKind::Camp) {
            self.reach.largest = self.reach.largest.max(u.people);
            let far = land::steps(w, w.civs[&self.me].capital, u.home);
            if far > self.reach.farthest {
                self.reach.farthest = far;
                self.reach.farthest_at = n;
            }
            if rivals.contains(&u.home) {
                self.reach.at_capital = self.reach.at_capital.max(u.people);
                self.reach.capital_at.get_or_insert(n);
            }
        }
        Ok(())
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
        let rcap = self.rival_capital();
        let mil: f64 = w.clans.values().filter(|k| k.civ == Some(self.rival) && k.home == rcap).map(|k| k.people).sum();
        let own: f64 =
            w.clans.values().filter(|k| k.civ == Some(self.me) && k.home == self.capital()).map(|k| k.people).sum();
        println!("   rival capital {rcap} clans {mil:.1}, own {own:.1}, givers {}", units::givers(w, self.me).len());
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
        "conquest" => (&CONQUEST, conquest),
        _ => return Err(format!("unknown story {name}: skirmish, stack, surrender or conquest\n{USAGE}")),
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
    let mut s = Story { w, me, rival, pick, trace, refused: Vec::new(), reach: Reach::default() };
    // `turns` is a cap: 0 plays to the end, which the limit always brings.
    while (turns == 0 || s.n() < turns) && !matches!(s.w.phase, Phase::Over(_)) {
        script(&mut s);
        s.end_turn()?;
        if s.trace {
            s.trace_line();
        }
    }
    fs::write(&save, serde_json::to_string(&s.w.save()).unwrap()).map_err(|e| format!("{save}: {e}"))?;
    let out = civs::alive(&s.w).count();
    let end = match s.w.phase {
        Phase::Over(o) => {
            let winner = &s.w.civs[&o.winner].name;
            let by = match o.why {
                Why::Conquest => "by conquest".to_string(),
                Why::Limit => "by the limit".to_string(),
                Why::Fell { by } => format!("after its capital fell to {}", s.w.civs[&by].name),
            };
            format!("game over at +{}: {winner} won {by}", o.turn)
        }
        _ => "the game goes on".to_string(),
    };
    let r = &s.reach;
    let capitals = s.w.civs.values().filter(|c| c.is_out()).count();
    println!(
        "{name} on {} seed {seed}: led civ {} against {}, {} turns past the pick ({} civs alive, {capitals} capitals fallen), saved to {save}; {end}; largest army {:.0}, farthest {} steps (at +{}), most at a rival capital {:.1}, first at one {}, first capital fell {}",
        size_name(radius),
        s.w.civs[&me].name,
        s.w.civs[&s.rival].name,
        s.n(),
        out,
        r.largest,
        r.farthest,
        r.farthest_at,
        r.at_capital,
        r.capital_at.map_or("never".to_string(), |n| format!("at +{n}")),
        r.fell_at.map_or("never".to_string(), |n| format!("at +{n}"))
    );
    for (n, order, why) in &s.refused {
        println!("  refused at +{n}: {order} ({why})");
    }
    Ok(())
}

/// **skirmish.** The led civ claims a corridor along the path to the nearest rival's capital, out
/// until it overlaps the rival's claim by two tiles, and keeps it.
///
/// - Every turn: the claim is the capital and the corridor, sent whenever it differs from the claim (a
///   tile lost is claimed again). Its guard is kept by the AI's rule, so its guard holds what its share
///   reaches, and a tile of the corridor the rival also holds is contested.
/// - From turn 30, or when the store fills three warriors' packs, those warriors are raised into one army, staged
///   (`stage_then_push`) and posted on the last corridor tile outside the rival's claim, with its zone
///   over the two tiles beyond. On small seed 2 the store never fills three packs, so the army is
///   the one warrior it can raise and the skirmish is the guards'.
fn skirmish(s: &mut Story) {
    let way = s.way();
    let theirs = s.w.civs[&s.rival].claim.clone();
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

/// The store banks until it fills `want` warriors' packs (or turn `by`), and every warrior it fills
/// is raised at once into one army.
fn bank_and_raise(s: &mut Story, want: u32, by: u32) {
    if s.camp().is_none() && s.n() > 0 && (s.fillable() >= want || s.n() >= by) {
        s.raise_all();
    }
}

/// The rival's claim and the corridor: the way to the rival's capital out to two tiles inside its
/// claim, the rival's capital left out.
fn corridor_claim(w: &World, me: u32, rival: u32) -> Vec<usize> {
    let (cap, theirs) = (w.civs[&me].capital, w.civs[&rival].claim.clone());
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
/// - +0: `UNIT.first` warriors are raised into an army at the capital, with their packs from the store
///   (refused if the store can't fill them, and then there is no army to march). Its two switches
///   go on, and the army is sent along the way to the rival's capital to the second tile
///   of the rival's claim (`Civ.claim`) on it, and left there. The rival's claim
///   takes back the tiles behind it (it skips only tiles a rival army holds), which cuts the army's
///   line; hungry three turns in a row with its line cut by the rival's land, it surrenders to it.
fn surrender(s: &mut Story) {
    if s.n() > 0 {
        return;
    }
    s.give(Order::Raise { n: UNIT.first, kind: UnitKind::Camp, fighter: Fighter::Warriors, into: None });
    let way = s.way();
    let theirs = s.w.civs[&s.rival].claim.clone();
    let first = way.iter().position(|t| theirs.contains(t)).unwrap_or(way.len().saturating_sub(2));
    let stop = way[(first + 1).min(way.len().saturating_sub(1))];
    if let Some(id) = s.camp().map(|u| u.id) {
        s.give(Order::Switches { unit: id, forage: true, supply: true });
        s.give(Order::Move { unit: id, to: stop });
    }
}

/// **conquest.** The strongest plan the player's levers allow (`stories.md`), to take the nearest rival's
/// capital and then the other's.
///
/// - +0: gift 0 (the story's), and the store kept whole (`SetKeep`, `CONQUEST_KEEP`): by default the
///   civ posts what it holds past a pack to the market, and with the keep the store banks to its cap.
/// - Once the store fills `CONQUEST_WANT` warriors' packs, or at turn `CONQUEST_BY`, every warrior it fills is
///   raised at once into one army (`bank_and_raise`), both switches on, and sent straight at the nearest
///   standing rival's capital: no stage and no wait. A wait at the stage tile eats a pack the store
///   can't refill (`probe.md`), and a march leaves no labour for the line, so the pack's five turns are
///   what the army has. Hopping a tile a turn to pillage on the way was tried: a rival's land holds too
///   little food to pillage and the hop is slower.
/// - An army lost on the way or at the capital is followed by the next one, banked and raised the same
///   way (`bank_and_raise` raises whenever the civ has no army). When a capital falls the target is the
///   other rival's.
fn conquest(s: &mut Story) {
    let cap = s.capital();
    if let Some(next) = civs::alive(&s.w)
        .filter(|c| c.id != s.me)
        .min_by_key(|c| (land::steps(&s.w, cap, c.capital), c.id))
        .map(|c| c.id)
    {
        s.rival = next;
    }
    if s.n() == 0 {
        // Keep the whole store, out of the market's posting: it banks to its cap.
        s.keep(CONQUEST_KEEP);
    }
    bank_and_raise(s, CONQUEST_WANT, CONQUEST_BY);
    let Some((id, home, marching)) = s.camp().map(|u| (u.id, u.home, u.camp().is_some_and(|c| c.dest.is_some())))
    else {
        return;
    };
    let target = s.rival_capital();
    if home == cap {
        s.give(Order::Switches { unit: id, forage: true, supply: true });
    }
    if !marching && home != target {
        s.give(Order::Move { unit: id, to: target });
    }
}

/// The warriors' packs the conquest story banks before it raises (waves of four beat bigger ones: a bigger
/// wave waits longer and starves as long on the march), the turn it raises at whatever the store holds,
/// and the food it keeps in the store (the store's cap).
const CONQUEST_WANT: u32 = 4;
const CONQUEST_BY: u32 = 80;
const CONQUEST_KEEP: f64 = 60.0;
