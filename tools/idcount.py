# Requirement IDs carried by phase headers: the "Requirements carried" block up to the first blank line or heading,
# ranges "X-01 to X-11" expanded, lines naming "Not carried" excluded, DR ids ignored. Usage: idcount.py LAST_PHASE
import re,sys,glob
one=re.compile(r'\b([A-Z]{1,3})-(\d{1,3})\b')
rng=re.compile(r'\b([A-Z]{1,3})-(\d{1,3}) (?:to|-|\.\.) \1-(\d{1,3})\b')
def ids(f):
    t=open(f).read()
    m=re.search(r'\*\*Requirements carried:\*\*(.*?)\n(?:\n|---|## )',t,re.S)
    out=set()
    if not m: return out
    for line in m.group(1).split("\n"):
        if 'not carried' in line.lower(): continue
        for a,b,c in rng.findall(line):
            for n in range(int(b),int(c)+1): out.add(f"{a}-{n:02d}")
        for a,b in one.findall(line):
            if a!='DR': out.add(f"{a}-{int(b):02d}")
    return out
s=set()
for i in range(int(sys.argv[1])+1):
    for f in glob.glob(f'docs/plan/phase-{i:02d}.md'): s|=ids(f)
print(len(s))
