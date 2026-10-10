# Coverage of research requirement IDs by phase headers. Baseline: unique PREFIX-NN defined in the first cell of a table
# row of research/01-33 (FM, FL, TR, G, IR excluded; 1,208, the planning.md baseline; VA, VB, VC, VD, VO are the 249 market IDs). Carried: idcount rule (header block, ranges expanded, "Not carried" lines excluded). Not carried: IDs named
# on a "Not carried" line of some phase. Usage: coverage.py OUTFILE
import re,glob,sys,collections
sys.path.insert(0,'tools')
one=re.compile(r'\b([A-Z]{1,3})-(\d{1,3})\b')
rng=re.compile(r'\b([A-Z]{1,3})-(\d{1,3}) (?:to|-|\.\.) \1-(\d{1,3})\b')
EX={'FM','FL','TR','G','IR','DR'}
base=collections.defaultdict(set)
for f in sorted(glob.glob('research/[0-3][0-9]-*.md')):
    n=int(f.split('/')[-1][:2])
    if n<1 or n>33: continue
    for a,b in re.findall(r'^\|\s*\**([A-Z]{1,4})-(\d{1,3})\**\s*\|',open(f).read(),re.M):
        if a in EX: continue
        base[f"{a}-{int(b):02d}"].add(f.split('/')[-1][:2])
def block(t):
    m=re.search(r'\*\*Requirements carried:\*\*(.*?)\n(?:\n|---|## )',t,re.S); return m.group(1) if m else ''
carried=collections.defaultdict(set); notc=collections.defaultdict(set)
for f in sorted(glob.glob('docs/plan/phase-[0-3][0-9].md')):
    p=f[-5:-3]; b=block(open(f).read())
    for line in b.split('\n'):
        tgt=notc if 'not carried' in line.lower() else carried
        for a,x,y in rng.findall(line):
            for n in range(int(x),int(y)+1): tgt[f"{a}-{n:02d}"].add(p)
        for a,x in one.findall(line):
            if a!='DR': tgt[f"{a}-{int(x):02d}"].add(p)
unc=sorted(k for k in base if k not in carried)
nc=[k for k in unc if k in notc]; open_=[k for k in unc if k not in notc]
key=lambda k:(k.split('-')[0],int(k.split('-')[1]))
with open(sys.argv[1],'w') as o:
    o.write(f"baseline {len(base)}; carried {sum(1 for k in base if k in carried)}; uncarried {len(unc)} "
            f"({len(nc)} named Not carried by a phase, {len(open_)} named nowhere)\n\n## Named nowhere (ID: research files)\n")
    for k in sorted(open_,key=key): o.write(f"{k}: {','.join(sorted(base[k]))}\n")
    o.write("\n## Named on a Not carried line (ID: phases)\n")
    for k in sorted(nc,key=key): o.write(f"{k}: phases {','.join(sorted(notc[k]))}\n")
print(open(sys.argv[1]).readline())
