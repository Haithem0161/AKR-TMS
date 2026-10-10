import re,sys,glob
defined_dr=set(re.findall(r'^\| (DR-\d+) \|', open('docs/plan/research.md').read(), re.M))
rtext=''.join(open(f).read() for f in glob.glob('research/[0-9]*.md'))
rids=set(re.findall(r'\b([A-Z]{1,4}-\d{1,3})\b', rtext))
skip={'DR','RQ','UTF','ISO','AL','AR','SHA','HS','MT','CT','DS','SP','F','Q','X','A','B','C','D','E'}
# Product rule and finding codes (phase 07 goAML checker; NAME and DATE added by phase 21), never research IDs; research uses none of these prefixes.
local={'BR','CONS','DATE','DEP','EMIT','NAME','NORM','OFF','OQ','PKG','PRE','XSD'}
bad=[]
for f in sys.argv[1:]:
    for n,line in enumerate(open(f),1):
        for m in re.findall(r'\bDR-\d+\b', line):
            if m not in defined_dr: bad.append((f,n,m))
        for m in re.findall(r'\b([A-Z]{1,4}-\d{1,3})\b', line):
            pre=m.split('-')[0]
            if pre in ('DR','RQ','UTF','ISO','SHA','HS','CT','F','Q','A','B','D','E','X','P','M','R','Z','S0','DS','AL','AR','MT','PR','ID','UX0','T','L1','IQ','UTC','IBM','PDF','RFC','EU','UK','US','TLS','AES','CBI','GCC','WCAG','OWASP','RTL','KYC','AML','CTR','STR','SAR','PEP','EDD','CDD','SDD','DSL','API','SSE','SLO','RPO','RTO','HA','DR0','EOD','GL','ISIC','IP','NCCT','FATF','OFAC','SBR','COL','NEW','SS','RFI','QA','OPS','CFG','CHK','CH','MRO','IA','AS','VS','ADM','SEC','A1','A2','KU','AR8','AL16','AL32','PBKDF','HKDF','HMAC','OIDC','JWT','TOTP') or pre in local: continue
            if m not in rids and m not in defined_dr: bad.append((f,n,m))
seen=set()
for b in bad:
    if b not in seen: seen.add(b); print(*b)
print('DR defined:',len(defined_dr),' problems:',len(seen))
