# The Goal's scores from sweep --judge --json reports: python3 goal.py name file [name file ...]
import json, sys
def g(o, p):
    for k in p.split('.'): o = o[k]
    return o
runs = [(n, json.load(open(f))) for n, f in zip(sys.argv[1::2], sys.argv[2::2])]
floor = next((d['floor'] for _, d in runs if 'floor' in d), {})
def row(name, f, fp=None):
    vals = [f(d) for _, d in runs]
    fl = floor.get(fp, '') if fp else ''
    fmt = lambda v: f'{v:.4f}' if isinstance(v, float) else str(v)
    print(f"| {name} | " + ' | '.join(fmt(v) for v in vals) + f" | {fmt(fl) if fl != '' else ''} |")
print('| score | ' + ' | '.join(n for n, _ in runs) + ' | floor |'); print('|' + '---|' * (len(runs) + 2))
T = lambda p: (lambda d: g(d['total'], p))
row('towns at +200', T('towns_vs_control.run.towns_200'), 'total.towns_vs_control.run.towns_200')
row('capital towns at +200', T('towns_vs_control.run.capital_towns_200'), 'total.towns_vs_control.run.capital_towns_200')
row('clans in towns at +200', T('towns_vs_control.run.town_clans_200'), 'total.towns_vs_control.run.town_clans_200')
row('trade a person a turn, town clans', T('towns_vs_control.run.town.traded_per_turn'), 'total.towns_vs_control.run.town.traded_per_turn')
row('trade a person a turn, outside towns', T('towns_vs_control.run.outside.traded_per_turn'), 'total.towns_vs_control.run.outside.traded_per_turn')
row('trade a person a turn, farmers outside', T('towns_vs_control.run.farmers_outside.traded_per_turn'), 'total.towns_vs_control.run.farmers_outside.traded_per_turn')
row('markets alive at +200', T('land.markets_200'), 'total.land.markets_200')
row('clans at a market at +200', T('land.trading_200'), 'total.land.trading_200')
def grow(i):
    def f(d):
        s = d['total']['settling']['total']
        b, p = [(s['settled_born'], s['settled_person_turns']), (s['gathering_born'], s['gathering_person_turns'])][i]
        return b / p if p else 0.0
    return f
row('born a person a turn, settled', grow(0))
row('born a person a turn, gathering', grow(1))
row('crowding deaths (people)', T('settling.total.crowded'))
row('people at +200', T('settling.total.people_200'), 'total.settling.total.people_200')
row('settled clans at +200', T('settling.total.settled_200'), 'total.settling.total.settled_200')
row('starving share, all', T('fields_vs_control.starving_share'), 'total.fields_vs_control.starving_share')
row('starving share, gatherers', T('fields_vs_control.gatherer_starving_share'), 'total.fields_vs_control.gatherer_starving_share')
row('starving share, settled farmers', T('fields_vs_control.settled_starving_share'), 'total.fields_vs_control.settled_starving_share')
row('health: pre-games by centres', T('health.pregame_by_centres'))
row('health: panics, digest mismatches', lambda d: f"{d['total']['health']['panics']}, {d['total']['health']['digest_mismatches']}")
row('pre-game end turns', lambda d: ' '.join(str(m['pick']) if not isinstance(m['pick'], dict) else '?' for m in d['maps']))
row('aligned share at +200', T('standing.aligned_200'), 'total.standing.aligned_200')
row('top civ at +200', T('standing.top_200'), 'total.standing.top_200')
