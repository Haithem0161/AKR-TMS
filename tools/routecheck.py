# API.md endpoint rows vs phase route tables. A row is owned when some phase file has a route row with the same method
# and path (path parameters normalised to {}). Usage: routecheck.py OUTFILE
import re,glob,sys
row=re.compile(r'^\|\s*(GET|POST|PUT|PATCH|DELETE)\s*\|\s*`?(/[^|`\s]*)`?(?:\s*\([^)|]*\))?\s*\|',re.M)  # a trailing (NEW) or (added) is allowed
norm=lambda p:re.sub(r'\{[^}]*\}','{}',p.rstrip('/'))
api={}; sec=None
for i,l in enumerate(open('docs/API.md').read().split('\n'),1):
    if l.startswith('### '): sec=l[4:].split('(')[0].strip()
    m=row.match(l)
    if m: api.setdefault((m.group(1),norm(m.group(2))),(i,sec))
own={}
for f in sorted(glob.glob('docs/plan/phase-[0-3][0-9].md')):
    for m in row.finditer(open(f).read()):
        k=(m.group(1),norm(m.group(2)))
        for pre in ('','/api/v1'):
            if k[1].startswith('/api/v1'): k=(k[0],k[1][7:])
        own.setdefault(k,set()).add(f[-5:-3])
missing=[(v,k) for k,v in api.items() if k not in own]
extra=sorted(k for k in own if k not in api)
with open(sys.argv[1],'w') as o:
    o.write(f"API.md rows {len(api)}; owned by a phase route table {len(api)-len(missing)}; not owned {len(missing)}; "
            f"phase routes absent from API.md {len(extra)}\n\n## API.md rows with no phase route row (line, section, method path)\n")
    for (i,sec),(m,p) in sorted(missing): o.write(f"l.{i} [{sec}] {m} {p}\n")
    o.write("\n## Phase route rows absent from API.md (method path: phases)\n")
    for m,p in extra: o.write(f"{m} {p}: {','.join(sorted(own[(m,p)]))}\n")
print(open(sys.argv[1]).readline())
