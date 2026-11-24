import json,sys
cols=[('towns 200','towns.total.towns_200'),('capital towns 200','towns.total.capital_towns_200'),('town clans 200','towns.total.town_clans_200'),
('town traded/p','towns.total.town.traded_per_turn'),('farmers outside traded/p','towns.total.farmers_outside.traded_per_turn'),('all traded/p','towns.total.all.traded_per_turn'),
('town starving','towns.total.town.starving'),('markets 200','land.at_200.markets'),('trading 200','land.at_200.trading')]
def g(o,p):
  for k in p.split('.'): o=o[k]
  return o
runs=[(n,json.load(open(f))) for n,f in zip(sys.argv[1::2],sys.argv[2::2])]
print('| map | run | '+' | '.join(c[0] for c in cols)+' |'); print('|'+'---|'*(len(cols)+2))
for i in range(7):
  for n,d in runs:
    m=d['maps'][i]
    vals=[g(m,p) for _,p in cols]
    print(f"| {m['size']} {m['seed']} | {n} | "+' | '.join(f'{v:.3f}' if isinstance(v,float) else str(v) for v in vals)+' |')
for n,d in runs:
  t=d['total']; 
  vals=[]
  for _,p in cols:
    try: v=g(t['towns_vs_control']['run'],p.split('.',2)[2]) if p.startswith('towns.') else g(t['land'],p.replace('land.at_200.','')+'_200')
    except Exception as e: v='?'
    vals.append(v)
  print(f"| total | {n} | "+' | '.join(f'{v:.3f}' if isinstance(v,float) else str(v) for v in vals)+' |')
