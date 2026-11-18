#!/usr/bin/env python3
"""Towns per retro map from `sweep --judge --json` reports: towns at +200, town clans, trade per
person a turn (after +100) in a town, of farmers outside towns, and of every clan outside a town.
Usage: python3 -I towns.py A.json [B.json ...]"""
import json, sys

def row(m):
    t = m['towns']['total']
    town, allg = t['town'], t['all']
    out_turns = allg['clan_turns'] - town['clan_turns']
    out_traded = allg['traded'] - town['traded']
    out = out_traded / out_turns if out_turns else 0.0
    return (t['towns_200'], t['town_clans_200'], m['towns']['formed'], town['traded_per_turn'],
            t['farmers_outside']['traded_per_turn'], out, town['clan_turns'])

for path in sys.argv[1:]:
    d = json.load(open(path))
    print(path)
    print(f"  {'map':10} {'towns200':>8} {'clans':>5} {'formed':>6} {'trade_town':>10} {'farm_out':>9} {'outside':>8} {'town_ct':>7}")
    tot = [0, 0, 0]; tt = [0.0, 0, 0.0, 0]
    maps = 0
    for m in d['maps']:
        r = row(m)
        maps += r[0] > 0
        tot = [tot[0] + r[0], tot[1] + r[1], tot[2] + r[2]]
        t = m['towns']['total']
        tt[0] += t['town']['traded']; tt[1] += t['town']['clan_turns']
        tt[2] += t['all']['traded'] - t['town']['traded']; tt[3] += t['all']['clan_turns'] - t['town']['clan_turns']
        print(f"  {m['size']+' '+str(m['seed']):10} {r[0]:8} {r[1]:5} {r[2]:6} {r[3]:10.4f} {r[4]:9.4f} {r[5]:8.4f} {r[6]:7}")
    print(f"  {'total':10} {tot[0]:8} {tot[1]:5} {tot[2]:6} {tt[0]/max(tt[1],1):10.4f} {'':9} {tt[2]/max(tt[3],1):8.4f}   maps with a town {maps}")
