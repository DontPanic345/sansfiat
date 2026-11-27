import subprocess, sys, re
B="/home/runner/work/sansfiat/sansfiat/engine/target/release/sansfiat"
game, seed, size, civ, turns = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4], int(sys.argv[5])
def call(lines, new=False):
    args=[B,"play","--game",game]
    if new: args+=["--seed",seed,"--size",size,"--civ",civ]
    return subprocess.run(args,input="\n".join(lines)+"\n",capture_output=True,text=True).stdout
out=call([],True)
for t in range(turns):
    if "the game is over" in out.split("\n")[0]: break
    lines=[]
    m=re.search(r'capital: held by your army, ([0-9.]+) warriors', out)
    w=float(m.group(1)) if m else 0
    if w<4:
        r=re.search(r'^\s*(\{"type":"raise","n":1,"into":\d+\})', out, re.M) or re.search(r'^\s*(\{"type":"raise","n":1\})', out, re.M)
        if r: lines.append(r.group(1))
    m=re.search(r'^\s*(\{"type":"claim".*?\})\s+the claim in reach', out, re.M)
    if m: lines.append(m.group(1))
    lines.append("end")
    out=call(lines)
print(out.split("\n")[0])
