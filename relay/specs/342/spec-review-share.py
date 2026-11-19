import json,sys
off=json.load(open('/tmp/runs/p0.json'))
for r in sys.argv[1:]:
    run=json.load(open(r))
    rows=[]
    for m,o in zip(run['maps'],off['maps']):
        sr={c['id']:c.get('at_200') or 0 for c in m['standing']['civs']}
        so={c['id']:c.get('at_200') or 0 for c in o['standing']['civs']}
        for cid,f in m['fields']['civs'].items():
            cid=int(cid)
            if cid not in sr: continue
            a=f['animal_food_200']/f['food_200'] if f['food_200'] else 0
            sh=f['herding_people_200']/f['people_200'] if f['people_200'] else 0
            rows.append((cid,a,sh,sr[cid]-so.get(cid,0)))
    print('==',r, 'animal shares:', ' '.join('%.2f'%x[1] for x in sorted(rows,key=lambda x:-x[1])))
    for th in (0.05,0.1,0.15):
        h=[x for x in rows if x[1]>=th]; o=[x for x in rows if x[1]<th]
        mh=sum(x[3] for x in h)/len(h) if h else float('nan'); mo=sum(x[3] for x in o)/len(o) if o else float('nan')
        print(f'  animal share >= {th}: {len(h)} civs, {sum(x[3]>0 for x in h)} gain, mean {mh:+.3f}; others {mo:+.3f}')
