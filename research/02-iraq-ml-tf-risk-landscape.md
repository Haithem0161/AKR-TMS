# 02 — Iraq: ML/TF/PF risk landscape (threats, vulnerabilities, typologies)

Status: in progress. Main source so far: MENAFATF MER of Iraq, May 2024, Chapter 1 (paras 41-67), which summarises
Iraq's first National Risk Assessment (NRA, completed Oct 2022, approved by the Council of Ministers Jan 2023,
World Bank methodology, nine working groups). STR, confiscation and corruption figures are in file 01 section 5.

## 1. Country context that shapes risk

- Population ~40.15 million (2020 census estimate); 19 governorates incl. the Kurdistan Region (Erbil,
  Sulaymaniyah, Duhok, Halabja). Borders: Turkey, **Iran (~1,480 km)**, Jordan, Saudi Arabia, Syria, Kuwait.
- GDP ~USD 207bn; **oil ~95% of exports**; still a **cash-based society** (cash economy ~42% of GDP, 2017 estimate,
  file 01).
- **Banks (2022): 76** — 7 state-owned, 40 private (Iraqi and foreign), 16 foreign branches; assets ~USD 128bn
  (61.8% of GDP); banking = **96.2%** of formal financial-sector assets. Adults with accounts 14.57m (2021);
  financial inclusion 33.5% (2021) vs 20.8% (2017).
- Non-banks are tiny by assets: insurance 0.5%, securities 0.5%, exchange companies (class A 0.43%, class B
  0.71%, class C currency exchange 1.16%), e-payment providers 0.10%.
- Iraq is **not a financial hub**: the dollar historically came only through the CBI for commercial purposes, which
  pushed an **alternative system of unlicensed exchange and money-transfer companies** (hawala), fast, cheap and
  cash-based, which criminals increasingly use.
- Remittances: inflows ~USD 624m (2022), outflows ~USD 852m (2021).

## 2. National ML threat ranking (NRA 2022)

Overall ML risk: **moderate to high**. Overall TF risk: **high**.

| Predicate crime | NRA rating | Facts from MER (2018-2022) |
|---|---|---|
| **Corruption** (theft of public money, bribery, embezzlement, illicit gain, abuse of office, oil-derivative smuggling as crime against public funds) | **High, ranked 1st** | ~USD 2.6bn confiscated (largest); 15 ML convictions; proceeds laundered locally and **largely abroad** |
| **Drug trafficking** (transit country; crystal meth and **Captagon** sold locally; 17 tons seized) | **High, ranked 1st** | ~USD 60m confiscated; **no ML convictions** |
| Smuggling of oil and oil derivatives (tank trucks on rural routes, pipelines near borders; est. ~10,000 barrels/day) | Moderate to high | ~USD 13.6m confiscated; no ML convictions |
| Tax and customs evasion (mostly legal persons; manipulated accounts; **falsified import invoices; import licences in names of people not active in trade**, hiding the BO; proceeds in personal accounts) | Moderate | ~USD 43.6m confiscated; 8 ML convictions |
| Trafficking in antiquities (~200,000 archaeological sites) | Moderate | ~USD 0.6m confiscated |
| Forgery (official documents, **commercial invoices**, medical reports) | Moderate | ~USD 56.4m; 9 ML convictions |
| Fraud (social-media remittance scams, unlicensed "loan/investment" companies; 2,481 cases) | Moderate | ~USD 65m; 5 ML convictions |
| Human trafficking (prostitution, forced child labour, begging, labour trafficking, forced marriage, organ sales; **48% of investigations in the Kurdistan Region**) | Ranked 11th | ~USD 0.17m; MER says credible sources see it as a major threat in Kurdistan, alongside cybercrime |

MER criticism: the NRA did not classify **external threats**, treated Kurdistan only to a limited extent, and
under-analysed cybercrime and human trafficking (authorities claimed no cybercrime complaints at all).

## 3. Sector vulnerability ranking (NRA)

- **High:** real estate; exchange offices.
- **Moderate to high:** **banks**.
- Moderate: e-payment companies. Moderate-low: DPMS. Low: securities brokers, insurers, lawyers.
- Banks are named with exchange and real estate as "the most prominent sectors misused for ML".
- Legal persons (2023 assessment): residual ML risk moderate to low (joint-stock and solidarity companies low;
  simple companies and sole proprietorships moderate-low; **LLCs and foreign-company branches medium**); residual
  TF risk highest (moderate-high) for **LLCs and foreign branches**.
- NPOs: moderate TF risk; **16 NPOs misused for TF** in 2018-2022 (suspicious foreign funding, activity in former
  ISIL areas).
- The CBI ran banking-sector risk assessments in 2019 (updated 2020-2022) covering customer activity, geography,
  services and delivery channels.

## 4. Terrorist financing

- **ISIL** is the main threat (remnants in deserts, valleys, rugged areas, e.g. Al-Anbar). Funding before 2017:
  donations, small businesses, **fake charities**, oil smuggling, kidnap for ransom, extortion/"taxes", drugs,
  antiquities, arms, unlicensed money transfer companies, human trafficking. After 2017: **extortion, theft, drug
  smuggling, foreign funding**.
- Movement of funds: **unlicensed money-transfer companies** between governorates and to Syria, Turkey, Belgium,
  Sudan; **cash couriers** across borders; formal banks and transfer companies also used. Uses: operations, daily
  needs, **salaries to fighters**.
- Other groups mostly self-finance.

## 5. Proliferation financing

Long border with Iran (subject to UNSCR 2231), dense commercial and financial ties and **religious tourism** with
Iran raise PF risk, especially **import/export of dual-use goods** (MER IO11 rated Low). US actions (file 01
section 4) show the live risk: Iraqi banks used to move dollars to Iran-linked actors (IRGC, Kata'ib Hizballah,
Asa'ib Ahl al-Haq) through **front companies** and nested correspondent accounts.

## 6. Methods most relevant to banks (from NRA, MER and CBI documents)

- Heavy cash placement; proceeds of major crimes moved **across borders**, especially corruption proceeds.
- Front companies and nominee importers; **fake or inflated import invoices** to buy dollars (the classic Iraqi
  dollar-auction abuse); trade-based ML.
- Unlicensed hawala connected to bank accounts (cash collected, consolidated into accounts, wired abroad).
- Real estate purchases (rule: real estate above IQD 500m must go through banks).
- Social-media fraud and fake investment companies; mule accounts.
- Public-sector payroll fraud and "ghost employees" (salary localisation, file 01 section 9).
- Dollar smuggling to neighbouring sanctioned economies (Iran, Syria) via exchange houses and cash couriers.

## 7. What this means for AKR-TMS

- Default typology packs should rank **corruption / PEP**, **trade-based ML and import-invoice abuse**, **hawala
  and pass-through accounts**, **cash structuring**, **sanctions evasion via front companies**, **drug-proceeds cash
  patterns** and **TF micro-patterns** (small regular transfers to conflict areas, NPO flows) above the
  Western-default fraud scenarios.
- Customer-risk factors should weight: legal-person type (LLC, foreign branch), NPO status, real-estate and
  exchange-company customers, border-governorate branches, Iran/Syria exposure, cash intensity.
- Kurdistan Region needs its own attention (human trafficking indicators, Kurdish name forms).
- Expect **weak external data** (no reliable BO registry access, no cybercrime statistics): the system should get
  more out of the bank's own data (network analysis across customers, shared identifiers) rather than rely on
  outside databases.

Sources:
- [MENAFATF — Mutual Evaluation Report of Iraq, May 2024 (PDF)](https://www.menafatf.org/sites/default/files/Newsletter/IRAQ%20MER_En_0.pdf), Chapter 1 paras 41-67
- File 01 sections 4-5 (US actions, STR and confiscation statistics)
