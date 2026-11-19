import json, sys

def load(p):
    return json.load(open(p))

run, off = load(sys.argv[1]), load(sys.argv[2])
herding, others = [], []
herders = animals = 0
kt = [0.0] * 3; kp = [0.0] * 3
farm = []
st = sto = ct = cto = 0
maps_ratio = []
for m, o in zip(run['maps'], off['maps']):
    sr = {c['id']: c.get('at_200') or 0.0 for c in m['standing']['civs']}
    so = {c['id']: c.get('at_200') or 0.0 for c in o['standing']['civs']}
    for cid, f in m['fields']['civs'].items():
        cid = int(cid)
        if cid not in sr:
            continue
        share = f['herding_people_200'] / f['people_200'] if f['people_200'] else 0.0
        g = sr[cid] - so.get(cid, 0.0)
        (herding if share >= 0.1 else others).append(g)
        herders += f['herders_200']; animals += f['animals_200']
    t = m['fields']['total']
    for f in m['fields']['civs'].values():
        for k in range(3):
            kt[k] += f['kind_tiles'][k]; kp[k] += f['kind_people'][k]
    to = o['fields']['total']
    st += t['starving']; ct += t['clan_turns']; sto += to['starving']; cto += to['clan_turns']
    maps_ratio.append((t['starving'] / t['clan_turns']) / max(to['starving'] / to['clan_turns'], 1e-9))
    for c in m['fields_vs_control']['civs']:
        if c.get('farms'):
            farm.append(c['gain'])

mean = lambda x: sum(x) / len(x) if x else float('nan')
tp = [kt[k] / kp[k] if kp[k] else float('nan') for k in range(3)]
print(f"herding civs {len(herding)} gaining {sum(g > 0 for g in herding)} mean {mean(herding):+.3f}; others {mean(others):+.3f}")
print(f"herders {herders} animals {animals:.0f}")
print(f"tiles a person herders {tp[0]:.3f} gatherers {tp[1]:.3f} farmers {tp[2]:.3f} ratio {tp[0]/tp[1]:.2f}")
print(f"farming civs {len(farm)} gain {mean(farm):+.3f}")
print(f"starving ratio {(st/ct)/(sto/cto):.2f}; maps " + " ".join(f"{r:.2f}" for r in maps_ratio))
