# 22. Arabic, Kurdish and Iraqi name matching for screening and entity resolution

Status: done (2026-10-06)

Scope: how names of people and entities in Iraqi core banking systems (Arabic script, sometimes Latin) and in payment messages (SWIFT, Latin script) are structured, written and varied, and how AKR-TMS should normalise, block, score, explain and remember matches against sanctions, national-list, PEP and adverse-media data. Part of the evidence base for the AKR-TMS plan. Cross-references: file 01 (Iraqi regulatory framework, sections 6 and 7), file 04 (Wolfsberg, UN list), file 14 (screening engine vendors) and file 15 (risk data providers) are written by other agents.

## Contents

1. Method and sources of evidence
2. Arabic name structure: ism, nasab, laqab, nisba, kunya, noms de guerre
3. Iraqi naming practice and what Iraqi identity documents hold
4. Kurdish, Turkmen, Assyrian and Chaldean, Armenian, Yazidi and other Iraqi name families
5. Transliteration systems and what Iraq actually uses
6. Orthographic variation in Arabic script and Unicode pitfalls
7. Latin-side variation and cross-script matching
8. The very-common-names problem and token weighting (with UN list statistics)
9. Secondary identifiers that cut false positives
10. Algorithms and tools (string metrics, phonetic keys, ML vendors, OpenSanctions, open-source NLP, Rust crates)
11. What regulators and Wolfsberg say; how OFAC scores names
12. Evaluation: test sets, precision and recall, building a synthetic Iraqi corpus
13. Matching pipeline design for AKR-TMS
14. What it means for AKR-TMS
15. Derived requirements
16. Sources

## 1. Method and sources of evidence

**Purpose.** Evidence base for the AKR-TMS plan on how names of people and entities in Iraqi bank data (mostly Arabic script, some Kurdish, some Latin) and in payment messages (Latin) should be matched against sanctions, national-list, PEP and adverse-media data, with fewer false positives and no silent misses.

**Evidence labels used in this file.** *Source* = read in a named primary or official document; *Measured* = computed in this research on 2026-10-06 from public data or a prototype (stated with n); *Secondary* = a reputable secondary source (Wikipedia, a vendor article, a news site); *Not verified* = general knowledge or a plausible inference that could not be checked. Where a claim is not verified it is said so in the text.

**Data analysed (all aggregate; no individual's name from any sanctions list is reproduced, and every example name in this file is invented unless a source is quoted).**

| Data | Size | Use |
|---|---|---|
| UN consolidated list XML (generated 2026-10-06 14:06 UTC; file 04 section 4a) | 736 individuals, 274 entities; 329 Arabic-script individual names | script, token, alias, date and orthography statistics (section 8.1); cross-script retrieval (7.3) |
| Iraqi national sanctions list, AML/CFT Office web service (file 01 section 6): paged JSON (6,280 rows, 4,467 distinct ids) and Excel export | 6,253 individuals, 5,698 distinct persons (core chains) | structure, token concentration, identifiers, homonym statistics (8.2, 6.5); candidate pool and population for the experiments (12) |
| Wikidata (CC0): persons with citizenship Iraq, Arabic and English labels | 5,042 label pairs; 3,022 born 1930-2007 with a birth date (3,007 with two or more tokens used in E1b) | Latin spelling variants (5.3, 7.1), key evaluations (10.2), ordinary-name queries (12.3) |
| Prototypes (Python 3.14 with rapidfuzz, jellyfish; Rust 1.97; PostgreSQL 16 in Docker) | scripts in the session scratchpad (`n22/`), not part of the repository | all measurements in sections 7, 8, 10, 12 and 13 |

**Documents and code read** (full URLs in section 16): IFLA "Names of Persons: Arabic Names" (2018); ALA-LC Arabic Romanization Table (2012, Wayback copy); UNGEGN working-group page; CLDR `Arabic-Latin.xml`; OFAC FAQs 5, 122, 124, 125, 246-249; Cohen, Ravikumar and Fienberg (2003); Splink documentation; PostgreSQL pg_trgm documentation; OpenSanctions licensing page; source code of nomenklatura, rigour (including its Rust core) and yente; CAMeL Tools source and character maps; Prowide Core SWIFT field definitions; vendor pages of NetOwl and Babel Street; crates.io metadata; internal files 01, 03, 04 and 05.

**What could not be reached or verified.**
- **Web search was exhausted** (200 of 200 session-wide calls) partway through, so later topics rely on direct fetches of known URLs; any literature not fetched directly (Arabic Soundex variants, MITRE and NEWS corpora, academic Arabic name-matching papers) is mentioned only as "not evaluated".
- **ICAO Doc 9303** (icao.int) and the original **Library of Congress** ALA-LC file (loc.gov) refuse automated access (Cloudflare 403); ALA-LC was read from a Wayback copy, and the ICAO Arabic transliteration table could not be opened.
- **IBM InfoSphere Global Name Recognition** documentation and the **Babel Street** technical documentation (JavaScript application) could not be read; vendor statements are limited to what NetOwl's and Babel Street's public pages say.
- **Iraqi bank practice** (how Latin names are produced, what the CBS holds, SWIFT gateway timeouts) has no public source; it is listed as a discovery question.
- The national-list **web service is undocumented**; the export behaviour found in 6.5 is not a supported interface.
- All performance figures come from prototypes on one laptop-class CPU with synthetic names; they show orders of magnitude and relative effects, not production guarantees.

**Reading aid: Arabic names and particles used in this file** (transliteration in the commonest English spelling; the variants are the subject of sections 5 to 8).

| Arabic | Transliteration | Note |
|---|---|---|
| محمد, أحمد, محمود | Muhammad, Ahmad, Mahmud | three different names; share the consonant skeleton M-H-M-D or H-M-D |
| علي, حسن, حسين | Ali, Hasan, Husayn | حسن and حسين are different names (the second has a ya) |
| صالح, صلاح | Salih, Salah | different names that Latin phonetic keys merge |
| قاسم, جاسم, كاظم | Qasim, Jasim, Kazim (Kadhim) | Iraqi speech says Gasim for قاسم |
| عبد الله, عبد الرحمن | Abd Allah (Abdullah), Abd al-Rahman | Abd compounds, written as one or two words |
| صلاح الدين, نور الدين | Salah al-Din, Nur al-Din | khitab compounds ending in al-Din |
| أبو علي, أم حسين | Abu Ali, Umm Husayn | kunya: "father of Ali", "mother of Husayn" |
| بن, ابن, بنت | bin or ibn, bint | "son of", "daughter of" (nasab) |
| ال | al- | definite article, attached to the next word |
| البغدادي, التميمي, الموصلي | al-Baghdadi, al-Tamimi, al-Mawsili (al-Mosuli) | nisba: from Baghdad, from the Tamim tribe, from Mosul |
| المكنى | al-mukanna | "the one with the kunya": marker used in the Iraqi national list |
| الملقب | al-mulaqqab | "the one nicknamed" |
| فاطمة, عائشة, زينب | Fatima, A'isha, Zaynab | feminine names: ta marbuta, hamza on ya |
| گۆران, ئاراس | Goran, Aras (Kurdish Sorani) | Kurdish letters گ ۆ and the initial ئ |

**How the file is organised.** Sections 2 to 4 describe names (Arabic, Iraqi, Kurdish and minority); 5 to 7 describe scripts, orthography and cross-script matching; 8 and 9 the common-names problem and the identifiers that resolve it; 10 and 11 the algorithms, tools and regulatory expectations; 12 the evaluation experiments and the synthetic corpus; 13 the pipeline design; 14 to 16 the synthesis, the derived requirements and the sources. Each topic section ends with "AKR-TMS hooks".

## 2. Arabic name structure: ism, nasab, laqab, nisba, kunya, noms de guerre

### 2.1 The elements of an Arabic name

IFLA's "Names of Persons: Arabic Names" (revised August 2018, based on the ALECSO text of 1994) lists the elements "normally forming part of a name". All example names below are invented.

| Element | Meaning | Form | Invented example | State in modern use (IFLA 2018, Cultural Atlas) |
|---|---|---|---|---|
| **Ism** (اسم) | the given or personal name, "usually of religious significance" | simple (Ahmad, Ali, Umar, Muhammad) or compound: "the prefix 'Abd (slave) with one of the 99 names of God" (Abd Allah, Abd al-Rahman) | أحمد جاسم | continues everywhere |
| **Nasab** (نسب) | patronymic: "ibn (son of)" or "bint (daughter of)" plus the father's and/or grandfather's name | compound with a particle | محمد بن علي بن حسن | "by and large disappeared" in the Mashriq; survives in Tunisia, Algeria and Morocco "preceded by the word ben and not ibn". In Iraq the chain is implicit: it is the order given name, father, grandfather |
| **Kunya** (كنية) | "Abu (father of) or Umm (mother of)" plus the name of a son or daughter; "not necessarily an indication of actual relationship and sometimes used to show honour and courtesy" | compound | أبو حسن | "in Iraq, as well as some other Arab countries, the kunya may be observed as a form of courteous address" |
| **Laqab** (لقب) | sobriquet, honorary title or epithet "indicating a person qualify or defect"; the **khitab** is an honorific laqab ending in al-Din ("the faith"): Nur al-Din, Fakhr al-Din | simple or compound | الرشيد, نور الدين | "the laqab and nisba are the most consistently used name elements" |
| **Nisba** (نسبة) | attributive element "indicating origin, residence, trade", formed by adding ya' twice (-i when romanised) | adjective | البغدادي, التميمي, البصري | most consistently used; works as the family name |
| **Shuhra** (شهرة) | not an element but a role: "the best known form" under which a person is known, which "may be" any one of the elements | | | the name the person uses in a given context |

Modern practice in Iraq (The Cultural Atlas, Iraqi naming): "[personal name] [father's personal name] [grandfather's personal name]". "Most Iraqis traditionally lack surnames. Instead, they may use their grandfather's personal name, a nisba (family/tribal name), or a laqab (nickname/title) as a functional surname when conforming to Western conventions." "Women typically maintain their own names after marriage." The same source warns that "the same name can be written as Majid, Majeed, or Mejeed" and that "only names written in Arabic characters are considered official and have standardised spellings in Iraq" (a secondary source; file 01 section 7 confirms the Arabic-script national card is the primary record).

Elements that are not part of the name but appear glued to it in real data (not separately sourced; standard practice in the region, to be confirmed with Iraqi banks): the honorific titles **Sayyid/Sayyed** (descendant of the Prophet, very common among Iraqi Shia), **Sheikh**, **Hajj/Hajji/Hajja** (pilgrim), **Mulla**, **Dr.** and **Eng.** (دكتور, مهندس). They carry no identifying power and should be stripped into a "title" bucket before scoring.

### 2.2 What this does to a "first name, last name" data model

- The unit of identity is an **ordered chain** (own name, father, grandfather, optionally great-grandfather, then family, tribe, nisba or laqab). The last token is not reliably a "surname": it can be the grandfather's ism, a tribe, a nisba or a laqab (IFLA: "entry should be under the last element in a name whether it is a laqab, nisba or ism").
- A person is known by a **shuhra** that changes with context: the bank's KYC record carries the document chain, a sanctions list may carry the kunya-plus-nisba form used in propaganda, a payment message may carry whatever the remitter typed. Matching must therefore be able to align **partial chains** (given plus father only; given plus tribe; given plus grandfather) and not only whole strings.
- Compound isms are single names split by spaces in Arabic and in Latin: Abd + one of the divine names, and X al-Din. A **closed lexicon** of the second element (the 99 names plus the Iraqi Shia forms such as Abd al-Hussein, Abd al-Zahra, Abd al-Ali, Abd al-Ameer) lets the pipeline join or split them canonically (sections 6.3 and 7.2).
- The definite article attaches to the following word and is often ignored in indexing (IFLA: "the definite article al is sometimes disregarded by libraries when filing"): screening must treat "al-" as optional (section 6.3).

### 2.3 Terrorist aliases: kunyas and noms de guerre

Designated fighters usually act under a chosen name, not their registered one. Vera Mironova (The Majalla, 21 July 2017) describes the construction used by ISIL members: "The first part distinguishes a person as a father. For example, if a fighter's oldest son is named Hasan, the first part of the fighter's name would be Abu Hasan." "The second part of the name often refers to the fighter's place of origin. For example, if he is from Salqeen, his name will be Abu Hasan Alsalqeeni." "The third part of the name consists of the name of the tribe the individual belongs to." She adds that "fighters choose their names themselves, they can intentionally misrepresent their identities". The parts are therefore: kunya (Abu or Umm plus a child's name or a companion's name), nisba (origin or trade), tribe.

What the UN consolidated list shows (section 8.1; the "Arab-world cohort" is the 194 individuals defined there): in the Arab-world cohort 76% of individuals have at least one alias (3.55 aliases on average, maximum 17); 15.7% of the aliases start with a kunya particle (Abu, Abou, Abo, Abi, Umm, Um, Oum), and 34% of individuals have at least one kunya alias; **70 of the 108 kunya aliases are rated "Low" by the UN itself**; of the 93 aliases that start with Abu, Abou, Abo or Umm, 53 are bare kunyas (two tokens or fewer) and 40 are longer (kunya plus nisba, tribe or another name); only 3 of 688 aliases are in Arabic script; the Iraq 1518 list (former regime) has far fewer aliases (0.85 per person) and almost no kunyas, probably because its subjects were state officials with documented names (an inference, not stated by the UN). The Wolfsberg sanctions-screening guidance (2019, file 04 section 2d) lists the weak aliases that should not be screened on their own: "short strings, aliases with digits, nickname words ... geographic references, common prefix plus one name"; the UN defines Low quality a.k.a. as "probably insufficient alone for a positive identification" (file 04 section 4a).

Matching consequences:

| Observation | Rule for AKR-TMS |
|---|---|
| A kunya names a role (father of Hasan), not a person; the same kunya is used by many people (in the UN cohort 9 of 74 distinct kunya strings are shared by two or three listed individuals, and all nine are generic forms such as Abu Ali, Abu Ahmad, Abu Muhammad, Abu Omar, Abu Ismail) | A kunya-only match never creates an alert on its own; it adds weight only when the other tokens or a secondary identifier also match |
| In Iraq the kunya is also a courtesy form of address (IFLA), so ordinary customers may be known as "Abu Ali" | Parse the kunya out of customer names and payment text, keep it as a separate field, and score it separately |
| Three-part noms de guerre (kunya, nisba, tribe) are more specific than a bare kunya | Score kunya plus nisba plus tribe jointly: a kunya plus a nisba that agrees with the customer's place or tribe is a stronger signal than either alone |
| Kunya, nisba and laqab appear in free text (SWIFT field 70, narrative, memo fields) not only in name fields | Screen free text with the same normalisation, but at a stricter threshold and with phrase-level (not token-level) matching |
| Aliases of designated persons are almost all Latin (685 of 688) | Keep the Latin alias, generate Arabic renderings of it (section 7), and match against both |

### 2.4 AKR-TMS hooks

- Name model: store customer and list names as **parsed chains** with typed parts (own name, father, grandfather, great-grandfather, family or tribe, nisba, laqab, kunya, title, mother's name) plus the original unparsed string; never rely on one "surname" field.
- Parser rules (Arabic and Latin): detect Abu/Umm kunya (with the following name), ibn/bin/bint and ben/ould markers, al-Din and Abd compounds (via lexicon), titles, nisba suffixes (-i, -iyya, -awi) and a maintained **Iraqi tribe and family lexicon** (to be seeded from public lists and curated with the bank's compliance team).
- Kunya and weak-alias policy: configurable and documented per list (Wolfsberg: "documented configuration"); default is "weak aliases score but never alert alone".
- The analyst screen shows the parsed chain of both sides, aligned token by token, with the unmatched tokens visible.
- Partial-chain matching is a first-class mode (given plus father; given plus tribe; given plus grandfather) with its own thresholds, because Iraqi records are often shortened.
- Women keep their own chain after marriage: never use a husband's name as part of the match key; use the mother's name field for the mother-and-child link only.

## 3. Iraqi naming practice and what Iraqi identity documents hold

### 3.1 How an Iraqi name is built

- **Chain, not first name plus surname.** UK Home Office, Iraq knowledge base profile: "Names consist of: forename (personal name), middle name (father's personal forename), surname (grandfather's personal forename, tribal or family name)"; a fourth name sometimes appears on identity cards and passports. The Cultural Atlas gives the same pattern, "[personal name] [father's personal name] [grandfather's personal name]", and notes that "most Iraqis traditionally lack surnames".
- **Two registers.** Sayari's guide to Iraqi names: "It is common for Iraqis to use a two- or three-part name in daily life that consists of their given name, father's name, and in some cases, their grandfather's name", while official documents carry longer chains; "region-based surnames often do not appear on official documents" (a vendor article, undated, so treat it as indicative). A person is therefore *a short name in the street, a long chain in the registry, a tribe or nisba in intelligence reports*.
- **Names are fixed in law and practice.** UK Home Office: "Women must keep their birth name following marriage" and "A person cannot change their name on their Iraq passport or identity card". For matching this means a woman's chain does not take the husband's name, and that a name on an Iraqi document is stable over time (an old record and a new record of the same person should carry the same chain, apart from spelling and the optional tribe element).
- **Arabic script is the official record.** The Cultural Atlas: "Only names written in Arabic characters are considered official and have standardised spellings in Iraq." Latin spellings are produced downstream (passport office, bank, remitter) and are not standardised.

### 3.2 What the Iraqi documents hold

| Document | Name data | Notes for AKR-TMS |
|---|---|---|
| **Unified National Card** (البطاقة الوطنية الموحدة, issued since 2016, file 01 section 7) | In Arabic: first name, last name (family or tribe, optional), father's name, **mother's name**, paternal grandfather's name, **maternal grandfather's name**. On the back an ICAO machine-readable zone in Latin script gives "primary identifier" (surname) and "secondary identifier" (given names). Uqudo's extraction schema (IRQ_ID) returns each name part in three forms: Arabic, transliterated, and "Kurdish to standard Arabic" | The card is the best KYC source: parts are already separated. The MRZ Latin form is one transliteration among several |
| **Passport** | A-series passports "contain text in Arabic, Kurdish and English"; the data page has "Full Name and Surname", date and place of birth, nationality, and "a 2-line machine readable zone, according to ICAO standard 9303"; the biometric passport was "first issued 9 January 2023" (Wikipedia). UK Home Office: names "can show in a variety of ways in an Iraq passport and in some cases may replicate the customer's surname in both the 'full name' field and 'surname' field" | The Latin name on a passport is the form most likely to appear in SWIFT messages; the chain may be duplicated or split differently from the card |
| **Civil status ID** (هوية الأحوال المدنية) | "name, date and place of birth, religion in Arabic, parent's details, photograph" (UK Home Office) | Legacy, still accepted by banks (file 01 section 7) |
| **Residence card** (بطاقة السكن), nationality certificate, family card, ration card | Arabic chain; the family card links relatives | Residence card number is a KYC field (see the IDC form below) |
| **Iraqi Depository Center individual KYC form** (rs.iq, March 2026, Arabic) | Investor name in four boxes: first, second, third, fourth (**laqab**); **mother's name** (اسم الأم); unified card number, issue and expiry; residence card number (with the Iraqi address format محلة/زقاق/دار); place and date of birth; nationality; spouse's name; passport number | An Iraqi regulator-side KYC form: the four-part name with a laqab slot and the mother's name are standard KYC data in Iraq's financial sector |
| **IQTFS national sanctions list** (file 01 section 6; all 6,253 individuals read on 2026-10-06, section 6.5) | Full Arabic name: core chain of 5.05 tokens on average (61.5% have exactly 5), plus an alias segment in 26.9% of rows; **mother's name** in 99.6% of rows (2.87 tokens on average); **birth year** in 95.9% | Same chain and mother's-name structure as the national card, so the national list is matched against the KYC record almost field for field |
| **UN consolidated list** (section 8.1) | Four name fields, one Arabic original name (329 of 736), Latin aliases; **no structured mother's name** (16 of 736 comments mention it) | Mother's name helps only for the national list |

The three Latin renderings of one Iraqi name (card MRZ, passport, bank or remitter spelling) can all differ; the Arabic original is the only form that is stable across documents.

### 3.3 The mother's name in Iraqi practice

- It is **printed on the national card together with the maternal grandfather's name**, requested on Iraqi KYC forms (IDC form above), and **published on the national sanctions list** next to the birth year. These are the two identifiers the Office chose to make a list entry specific.
- It works as a **tie-breaker in both directions**: the same mother's chain on both sides is strong evidence of identity (the chain is long and varied), while a different mother's chain is strong evidence against, even when the person's own chain is identical. Because many Iraqis share given name, father and grandfather, this pair is far more selective than the person's own name (section 8).
- It is entered by branch staff as free text and therefore has the same spelling, spacing and truncation problems as the customer's name (section 6), and often fewer parts (first name only, or two parts) than the list holds. It must be compared with the chain logic, not with exact equality.
- Not verified: how many Iraqi banks hold the mother's name as a structured CBS field and how complete it is; discovery questions for each bank are listed in the hooks below.

### 3.4 Group and regional notes that change the matching rules

- **Kurdish chain.** Sayari: Kurds use "the given name followed by the patronymic and family name". Kurdish family names (the Cultural Atlas gives tribal or geographic examples) are more often present than among Arabs. Kurdish spellings differ between Kurdish-script and Arabic-script documents (section 4), which is why the card extraction schema carries a "Kurdish to standard Arabic" variant.
- **Names do not reveal sect or ethnicity reliably, and must not be used as a proxy for it.** Name-based inference of religion or ethnicity is unreliable and discriminatory; AKR-TMS may detect the *language or script origin* of a name (Arabic, Kurdish, Turkic, Armenian, Syriac-origin) only to choose transliteration and normalisation rules, never to score risk.

### 3.5 AKR-TMS hooks

- **CBS discovery checklist per bank** (to be filled during onboarding): which table and columns hold the name (one string or parts), the Latin name (if any), the mother's name and its parts, the family number, the card number, the residence card number, the date and place of birth; how many customers have each field populated; what the branch keyboard layouts and data-entry rules are; whether names were migrated from an older system. Output a **name data-quality profile** per bank before go-live.
- **Parse on ingest**: if the CBS holds one string, split it into the chain with the parser of section 2.4 and keep the raw string; if it holds parts, trust them but still normalise.
- **Secondary-identifier fields as first-class citizens**: mother's chain (with maternal grandfather), birth year, birth date, place of birth (Iraqi governorate gazetteer), card number, family number, residence card number, passport number.
- **Latin name is optional and derived**: store it when present (source: CBS, passport, SWIFT), and generate candidate Latin renderings from the Arabic original rather than the reverse (section 7).
- **Name-change events** are rare in Iraq (UK Home Office); a change of name on an existing customer is a data-quality alert and a re-screening trigger in its own right.
- **No sect or ethnicity inference** from names: write this into the model-governance document (file 29) and into the feature list of any ML component.

## 4. Kurdish, Turkmen, Assyrian and Chaldean, Armenian, Yazidi and other Iraqi name families

### 4.1 Name families found in Iraqi banks

Iraq's population is mostly Arab, with Kurds, Turkmen, Yazidis, Assyrians and Chaldeans, Armenians, Shabaks, Mandaeans and others (Wikipedia, Iraq). All of them appear in CBS databases in **Arabic script**, because Iraqi civil documents are in Arabic (file 01 section 7), while their names follow different structures and spelling traditions. Sources are secondary (Wikipedia, Cultural Atlas, Sayari); items marked "not verified" come from general knowledge and must be checked with Iraqi compliance staff.

| Group | Names in Iraqi documents | Structure and features | Matching consequences |
|---|---|---|---|
| **Arab (Sunni and Shia)** | Arabic script | Chain: given name, father, grandfather, optional tribe, nisba or laqab (section 3). Iraq has "approximately 150 tribes, 30 of which are considered the most influential, composed of about 2,000 smaller clans", and "75% of Iraqis are members of a tribe or have kinship to one" (Wikipedia, Arab tribes of Iraq; the 75% figure is not verified). Tribal names in the form al-Jubouri, al-Dulaimi, al-Shammari | A tribe or nisba token is shared by thousands of people: it must carry a low weight (section 8) and an unknown tribe must not block a match |
| **Kurds, Sorani** (Erbil, Sulaymaniyah, Kirkuk, Halabja) | Central Kurdish is written in a modified Perso-Arabic script of 34 letters; "the Kurdo-Arabic alphabet is commonly used in Iraq and Iran" (Wikipedia). Iraqi ID extraction returns a "Kurdish to standard Arabic" variant of every name part (Uqudo) | "The given name is followed by the patronymic and family name" (Sayari); Kurds "adopted Arabic conventions but often incorporate tribal or geographical family names" (Cultural Atlas); many given names are Kurdish rather than Arabic-Islamic | Extra letters پ چ ژ گ ڤ ڕ ڵ ۆ ێ ە must be folded (sections 4.3 and 6.4); the same person can appear in Kurdish spelling on a Kurdistan-issued document and in standard Arabic spelling elsewhere |
| **Kurds, Kurmanji / Badini** (Duhok, Zakho, Sinjar) | "The Hawar alphabet is primarily used in Syria and Turkey" (Latin, 31 letters: 26 basic plus ç ê î ş û), while in Iraq the Arabic-based script is common | Same chain as Sorani; Yazidis speak Kurmanji | Latin Hawar spellings (Şêrko, Hêvî) arrive through diaspora, passports and the media; casefold and diacritic stripping must keep ş, ç, ê, î, û recoverable |
| **Turkmen** (Kirkuk, Tal Afar, Erbil, Mosul; about 3 million by a 2013 Ministry of Planning estimate quoted by Wikipedia) | Arabic script in documents; the Turkmen language moved to a Latin Turkish alphabet "in Iraqi schools" by 2005 (Wikipedia) | Turkic and Arabic-Islamic given names; Turkish-style family names (not verified for Iraq: the Wikipedia article does not cover names) | Latin renderings follow Turkish letters (ç ş ğ ı İ ö ü). Python's `casefold("İbrahim")` returns "i̇brahim" (i plus a combining dot, tested 2026-10-06), so Turkish capital İ needs an explicit rule |
| **Assyrians and Chaldeans** (Nineveh Plain, Baghdad, Duhok, Erbil) | Arabic script in documents, Syriac script in church use | "Semitic naming conventions, which structure names similar to Arabic conventions"; traditional patronymic "bet or bit" ("house of"); "in the 1920s-30s many adopted the personal name of a grandparent as their family name" (Cultural Atlas). Given names are biblical or Aramaic (Wikipedia notes that Sargis is used by both Armenians and Assyrians; Abdisho is a Syriac compound with "Abd") | Compound "Abd" names that are not on the Islamic list (Abdisho, Abd al-Masih); Latin forms often follow European saints' names (George, Joseph) beside the Arabic form (Jurj, Yusuf): a name lexicon with cross-script equivalents is needed |
| **Armenians** (Baghdad, Basra, Mosul; small) | Arabic script in documents | Surname suffix "-yan/-ian ... transliterated as -yan, -ian, or less often -jan" with historic variants -ents, -uni, -li, -ji (Wikipedia, Armenian name) | The suffix is a family marker with 3 Latin forms; the Arabic spelling of the same surname varies with the transliterator. Eastern and Western Armenian spell given names differently (Petros/Bedros, Hovhannes/Hovannes: not verified here) |
| **Yazidis** | Arabic script in documents | Castes sheikh, pir and murid; tribes such as Rojkî, Halitî, Çekvânî (Wikipedia, Yazidis, which does not cover personal naming); Kurmanji given names | Caste titles (sheikh, pir, mir) are titles, not names: strip like Sayyid |
| **Mandaeans** (Sabian-Mandaean; Baghdad, Basra, Nasiriyah) | Arabic script | Secular birth name plus a religious name; "historically, lay Mandaeans lacked family surnames and instead used matronymic references" with "bar" (son) and "beth" (daughter) plus the mother's name; modern official surnames derive from clan names (Wikipedia, Mandaean name) | The **mother's name can be part of a Mandaean's own chain**: treat "bar/beth + mother" as a matronymic element, not as a mismatch with a father-based chain |
| **Faili Kurds, Shabaks, families of Persian origin** | Arabic script, Persian letters possible | Nisba from Persian places (al-Isfahani, al-Khorasani, al-Tabrizi: not verified for Iraq) | Persian ya and kaf (ی ک گ پ چ ژ) appear in Arabic-language data: 12 uses of Persian ya sit inside Arabic-language entries of the UN file (section 8.1) |

### 4.2 Kurdish script and Unicode facts that matter

- **Alphabet.** "The Sorani alphabet comprises 34 letters in the modified Perso-Arabic script"; beyond Arabic it adds پ (p), چ (ch), ژ (zh), گ (g), ڤ (v), ڕ (r, trilled), ڵ (l, velarised), ۆ (o), ێ (ê), ە (the short vowel written ae). Code points (verified with Python 3.14, Unicode 16.0): پ U+067E, چ U+0686, ژ U+0698, گ U+06AF, ڤ U+06A4, ڕ U+0695, ڵ U+06B5, ۆ U+06C6, ێ U+06CE, ە U+06D5. None of these decomposes under NFC, NFD, NFKC or NFKD.
- **Same visible letter, different code point.** "Although the Kurdistan Region's standardization uses ک (U+06A9) instead of ك (U+0643), both remain in use" (Wikipedia, Kurdish alphabets). Kurdish keyboards also type ی (Farsi yeh U+06CC) where Arabic keyboards type ي (U+064A). We tested علي typed both ways: the two strings are different and **stay different after NFC and NFKC**; the same holds for كريم against کریم. Unicode normalisation does not fix this; an explicit fold table does (sections 4.3 and 6.4).
- **Legacy encodings.** "Alifonts ... enabled typing of Kurdish with Arabic or Farsi keyboard layouts" using "a non-standard mapping"; the zero-width non-joiner "is non-standard but occurs a lot, most of the time this is due to poor conversions from non-Unicode to Unicode mapping"; "some old converters convert Teh Marbuta (0629) to Heh + ZWNJ (0647 200C) instead of the correct Ae (06D5)" (Wikipedia, Kurdish typography). Data migrated from older bank systems may therefore carry ZWNJ in the middle of names or a wrong letter for Kurdish ە.
- **A real adjudication of Kurdish spelling already exists.** The national card extraction schema (Uqudo IRQ_ID) returns, for each name part, the Arabic text, a transliteration and a "Kurdish to standard Arabic" version, which shows the market treats Kurdish-to-standard-Arabic mapping as a separate normalisation step. Pairs of original and standardised names from the bank's own KYC data are the best training data for it.

### 4.3 Proposed folding for Kurdish and Persian letters (to validate with Kurdish speakers)

Every fold is lossy. Keep the raw string, the folded string and a flag that says which folds were applied.

| Letter (code point) | Default fold for matching | Plausible Arabicised alternatives to generate as extra variants |
|---|---|---|
| ک U+06A9, ڭ U+06AD | ك | none |
| ی U+06CC, ێ U+06CE, ې U+06D0, ے U+06D2 | ي | none |
| ە U+06D5, ھ U+06BE, ہ U+06C1 | ه | final ە may be written ه, ة or ا in Arabic spellings |
| ۆ U+06C6, ۇ U+06C7, ۋ U+06CB, ۊ U+06CA | و | none |
| ڕ U+0695, ڑ U+0691 | ر | none |
| ڵ U+06B5 | ل | none |
| ڤ U+06A4 | ف | ب |
| پ U+067E | ب | ف |
| چ U+0686 | ج | ك, ش |
| ژ U+0698 | ز | ج |
| گ U+06AF | ك | ق, ج, غ |

(The variant column is design hypothesis from how Arabic speakers adapt sounds absent from Arabic, not verified against Iraqi data. The Iraqi dialect also pronounces ق as g, so the pair گ and ق must be bridged on both Kurdish and Arabic names.)

### 4.4 AKR-TMS hooks

- **Per-token origin detection** (by script letters and suffix lexicon, never by religion or sect): Kurdish-only letters, Turkish letters in Latin, Armenian -ian/-yan/-jan, Syriac-origin given names. The origin selects the fold table, the casefold rule and the variant generator; it is never a risk feature.
- **Fold tables as versioned data** (file 29 model governance): the Kurdish and Persian folds above, the Turkish casefold rule, ZWNJ removal, and a conversion repair for Heh plus ZWNJ to ە; every release is tested against the test pack of section 12.
- **Lexicons**: Iraqi tribes and families with variants; Assyrian and Armenian given names and surname suffixes with cross-script equivalents; Abd-compounds beyond the Islamic list; titles (Sayyid, Sheikh, Pir, Mir, Hajji, Mulla, Dr., Eng.).
- **Matronymic detection**: "bar/beth + name" and "bint/bin + name" patterns must not be scored as a father mismatch.
- **Kurdish-Arabic pair learning**: when the bank holds both the Kurdish and the standard Arabic spelling of a customer, store the pair as a bank-specific variant.
- **Test pack**: at least 300 synthetic Kurdish, Turkmen, Assyrian and Armenian names in both scripts (section 12) must pass before each release.

## 5. Transliteration systems and what Iraq actually uses

### 5.1 The systems

No single system governs how Arabic names become Latin ones. The formal systems below exist for libraries, geographic names and machine-readable travel documents; banks, passport offices and payment messages mix them with local habits.

| System | Issuer and date | Character | Key conventions (verified from the source named) | Where it is used |
|---|---|---|---|---|
| **ISO 233** (1984) and **ISO 233-2** (1993) | ISO | Letter-for-letter, reversible, with diacritics (ISO 233); "simplified transliteration" for indexing (ISO 233-2: "Arabic words are vocalized prior to romanization") | ISO 233 uses special symbols (for example ẗ for ta marbuta); ISO 233-2 "is used in French libraries and in North African libraries, and is recommended by ISSN" (Wikipedia, ISO 233) | Libraries; not seen on Iraqi civil documents |
| **DIN 31635** (1982) | DIN (Germany) | Scientific, one-to-one | ǧ ḫ ṯ ḥ with diacritics | Arabic studies |
| **ALA-LC** (tables 1997, 2011, 2012) | Library of Congress and American Library Association | Cataloguing; digraphs plus diacritics | Read from the 2012 table (Wayback copy of loc.gov, 2026-10-06): ث th, ج j, خ kh, ذ dh, ش sh, ص ṣ, ض ḍ, ط ṭ, ظ ẓ, ع ʿ, غ gh, ق q, ك k, ه and ة h, و w, ي y. Rule 17: "The romanized form al is connected with the following word by a hyphen" and "The ل of the article is always romanized l, whether it is followed by a sun letter or not"; rule 18: "the definite article al is given in lower case in all positions"; rule 25: "ابن and بن are both romanized ibn in all positions", with an exception for "modern names, typically North African" where بن is bin | Library catalogues. Public knowledge bases inherit it: **14.8% of the English labels of 5,042 Iraqi persons on Wikidata contain ayn or hamza modifier marks** (ʻ ʿ ʼ ʾ), and 23.2% contain some non-ASCII character |
| **BGN/PCGN** (1956) | US Board on Geographic Names and UK Permanent Committee on Geographical Names | Geographic names | j, kh, th, gh, q; definite article capitalised without hyphen ("Al ...") per the Wikipedia summary | US and UK government gazetteers |
| **UNGEGN** | UN Group of Experts on Geographical Names | Geographic names | The current system is the "Unified Arabic Transliteration System", "approved by UN resolution XI/3 in 2017", "based on the system adopted by Arabic experts at the conference held in Beirut in 2007" (WGRS page); implemented in Jordan, Oman and Saudi Arabia. Recommended "for romanization of the geographical names" only | Toponyms; not for personal names |
| **ICAO Doc 9303**, Part 3, section 6 and Appendix B | ICAO | Machine-readable travel documents | The visual zone must carry a Latin transliteration when the name is not in Latin letters; the machine-readable zone name field is "limited to the OCR-B characters A to Z and <". An Arabic table is recommended for the MRZ (ICAO working papers describe X-prefixed codes such as XTA/XAH for ta marbuta and XAY for alef maksura: second-hand, because **icao.int and loc.gov refuse automated access (Cloudflare 403)** and the Doc 9303 text could not be opened) | Passports and ID cards (Iraq: A-series passports and the Unified National Card have an ICAO MRZ) |
| **Buckwalter** (ALPNET, 1990s) | Tim Buckwalter | One-to-one ASCII mapping for software | From the CAMeL Tools map `ar2bw_map.json` (MIT licence): ء to ', آ to \|, أ to >, ؤ to &, إ to <, ئ to }, ا A, ب b, ة p, ت t, ث v, ج j, ح H, خ x, د d, ذ *, ر r, ز z, س s, ش $, ص S, ض D, ط T, ظ Z, ع E, غ g, ف f, ق q, ك k, ل l, م m, ن n, ه h, و w, ى Y, ي y, tatweel _; short vowels a u i, nunation F N K, shadda ~, sukun o, dagger alef `, alef wasla {; extended پ P, چ J, ڤ V, گ G | NLP research. Never used by banks, but ideal as an **internal canonical form** for debugging and for rule tables |
| **Arabic chat alphabet (Arabizi)** | Informal | Chat Latin | 3 = ع, 5 = خ, 6 = ط, 7 = ح, 8 = غ, 9 = ص or ق, 2 = hamza or ق; "in Iraq" چ "may be used to transcribe /tʃ/" but is "most often transcribed as if it were تش"; for ق the options include g and q; "never used in formal settings" (Wikipedia, Arabizi) | Customer-typed text in mobile apps, memos, social media |

### 5.2 What Iraqi passports, cards and banks actually use

Evidence found (no authoritative statement on Iraqi bank practice exists in public sources):

- **Passports.** A-series passports "contain text in Arabic, Kurdish and English"; the data page has "Full Name and Surname" and a two-line ICAO MRZ; the biometric passport has been issued since 9 January 2023 (Wikipedia, Iraqi passport). UK Home Office: names "can show in a variety of ways in an Iraq passport". The Latin spelling is therefore chosen by the issuing office and printed in two places (visual zone and MRZ).
- **National card.** The back of the Unified National Card carries an MRZ with primary and secondary identifier in Latin (Uqudo IRQ_ID schema), which is a third Latin rendering next to the passport and the bank's own.
- **Law and practice.** "Only names written in Arabic characters are considered official and have standardised spellings in Iraq" (Cultural Atlas). The Latin form is a convenience copy.
- **Public data.** Wikidata labels of Iraqi persons show the actual spread (section 5.3 and Table 8.1e): the commonest Latin spelling of محمد (283 persons whose given name is Muhammad) is Muhammad (106), then Mohammed (103), Mohammad (32), Mohamed (19) and Muhammed (8): no spelling has even 40% of the usage. The UN list gives a similar picture (Table 8.1e).
- **Payment messages.** Names reach the screening gateway in Latin capitals typed by branch staff or generated from CBS fields; SWIFT FIN fields 50K and 59 allow "[/34x$]35x[$35x]0-3", that is a name plus address in at most four lines of 35 characters (Prowide Core, SRU 2025 field definitions), which cuts a long Iraqi chain (given name, father, grandfather, tribe) and forces abbreviations or truncation. The FIN character set has no Arabic letters or diacritics (general SWIFT knowledge, not re-verified here).
- **Not verified**: how each Iraqi bank derives Latin names (system transliteration, branch typing, passport copy). **Discovery step**: compare, for the same customer, the Arabic name in the CBS with the Latin names found in the bank's own outgoing payments, KYC English-name field and SWIFT records. This yields a **bank-specific variant table** and shows how often Latin names are truncated or reordered (hook below).

### 5.3 Iraqi pronunciation changes the Latin spelling

Latin spellings follow what the writer hears. In Iraqi Arabic ق is often pronounced g and ظ/ض are rendered dh or z, so one Arabic name receives several Latin forms (counts from the aligned Arabic and English labels of Iraqi persons on Wikidata; tokens of equal-length names only; a few alignment errors are included):

| Arabic name | Latin spellings found (count) |
|---|---|
| قاسم | Qasim 30, Qasem 4, Qassim 3, Kasim 2, Qassem 1, Kasem 1, Kassim 1 |
| كاظم | Kazim 26, Kadhim 22, Kadhum 4, Kazem 4, Kadim 2, Khadim 1 |
| فاضل | Fadhil 16, Fadil 12, Fadhel 6, Fadel 4, Fazil 2, Fahdel 1, Fadelle 1 |
| حيدر | Haydar 10, Haidar 10, Haider 7, Hayder 5 |
| جاسم | Jasim 45, Jassim 25, Jassem 5, Gasim 1 |
| جعفر | Jafar 23, Jaafar 8, Jaafer 1, Jafaar 1 |
| ثامر | Thamir 13, Thamer 4 |
| حسين | Hussein 44, Husayn 13, Hussain 11, Husain 1 |

Practical consequence: two *different* names can be closer to each other in Latin than two spellings of the *same* name: Jaro-Winkler gives Jasim against Qasim 0.87 (different names) but Qasim against Kassem 0.70 (same name), and Hussein against Husayn 0.82 (section 10.2). The Arabic original removes this problem: all of Qasim, Qasem, Qassim, Kasim and Gasim are the one Arabic string قاسم (distance 0), while جاسم is a different string, so exact comparison of normalised Arabic already separates the two names and no fuzzy threshold is needed. **Matching should therefore go through the Arabic form whenever it is available.**

### 5.4 AKR-TMS hooks

- **Arabic is the system of record for matching.** Index and compare the normalised Arabic name first; Latin names (SWIFT, passports, aliases) are matched through generated Arabic or phonetic keys (section 7), never by comparing two Latin strings only.
- **Own canonical key**: define an internal, reversible ASCII form (Buckwalter-style) for Arabic and a documented Latin-to-key mapping with Iraqi dialect reflexes (q, g, k; dh, z, d; ch, sh, j; ou, u, oo; ei, ai, ay). Rules are versioned data (file 29).
- **Do not pick a standard.** Support ALA-LC marks (ʻ ʿ ʼ ʾ), BGN-style forms, ICAO MRZ forms (with the `<` filler), Arabizi digits and the common Iraqi spellings as *inputs*; strip marks, map digits, and treat all as variants.
- **Bank-specific variant learning** from read-only CBS data: pair each customer's Arabic name with the Latin names found in that bank's payments and KYC English-name fields (with a data-governance approval); keep frequencies; use the table to expand queries and rank variants (file 30 for privacy).
- **Payment-message reassembly**: join multi-line name fields of 50K and 59 into one name candidate, split name from address using line position, handle truncation at 35 characters, and flag messages whose name appears truncated (a data-quality signal for the Wolfsberg payment-transparency work, file 04 section 2f).
- **Record the source of each Latin name** (passport, MRZ, SWIFT, typed) because the likely spelling variants differ by source.

## 6. Orthographic variation in Arabic script and Unicode pitfalls

### 6.1 What varies in Arabic script, and how often (measured)

Three real Arabic-script data sets were measured (2026-10-06; scripts in the scratchpad): the **UN consolidated list** (329 individual names, Table 8.1g), the **Iraqi national list** downloaded from the AML/CFT Office web service (6,253 individuals, 5,698 distinct persons by core chain; see 6.5 for how it was obtained) and **Wikidata labels** of 5,042 Iraqi persons with Arabic and English labels (CC0). These are curated sources: customer data typed in branches will vary more.

| Variation | Example | UN Arabic names | Iraqi national list | Wikidata Arabic labels |
|---|---|---|---|---|
| Hamza and madda on alef (أ إ آ) against plain ا | أحمد / احمد, إبراهيم / ابراهيم, آمنة / امنة | 10.5% of alef-type letters (Arab-world names) | 1.7% (336 of 20,309, core names; the kunya أبو in alias segments adds more) | 8.6% (910 of 10,608) |
| Hamza seats and bare hamza (ؤ ئ ء) | مؤيد / مويد, عائشة / عايشة, رؤوف / رءوف | 4 ئ, 3 ء | ؤ 77, ئ 389, ء 101 | not tabulated |
| Alif maksura (ى) against ya (ي) | مصطفى / مصطفي, موسى / موسي, عيسى / عيسي | 6% of ya-type letters | 3.3% (535 against 15,483) | 3.0% (220 against 7,004) |
| Ta marbuta (ة) against ha (ه) | فاطمة / فاطمه | 52 ة | 393 ة | 458 ة |
| Diacritics (harakat, shadda, dagger alef) | مُحَمَّد | 1 name | 0 | 13 marks |
| Tatweel (U+0640) | محـمد | 2 names | 1 | 0 |
| Persian ya (ی) and keheh (ک) in Arabic-language data | علی / علي, کريم / كريم | 12 and 1 occurrences in the Arabic-language lists; 89 of 135 Taliban names carry Persian or Pashto letters | 0 in names, 1 in mother names | 13 ya, 16 labels with any Persian or Kurdish letter |
| Presentation forms (U+FB50-FDFF, U+FE70-FEFF) | pasted from PDF or legacy systems | 6 of 329 names (1.8%) | 0 | not tabulated |
| Space inside compounds | عبد الله / عبدالله | "عبد" separate 70% (66 against 28 glued) | **98.1% separate (1,759 against 34)** | n/a |
| Space after Abu | أبو علي / أبوعلي | 4 separate, 2 glued | 914 separate, 1 glued (mostly inside alias segments) | n/a |
| Attached definite article | الجبوري (attached) / ال جبوري (separate) | 256 attached tokens, 1 separate | 8,013 attached, 14 separate | n/a |
| Kunya or alias written inside the name field | محمد علي المكنى أبو أحمد | none (aliases are separate fields) | **1,679 names (26.9%) carry an alias segment: 1,470 contain the marker "المكنى" ("the one with the kunya"), 562 use parentheses (200 start with المكنى, 322 with أبو), 4 use "الملقب"; 67% of the segments contain أبو or أم**; the alias field is filled in for only 3 of 6,253 entries | none |
| Whitespace defects | double space, leading or trailing space | 1 name | 49 double spaces, 35 leading or trailing (1.3% of names) | none |
| Stray digits and symbols | | 0 | 1 name with digits, 12 names with double quotes, 1 slash, 1 Arabic comma | none |

Findings that matter for design:

1. **Even the official list is not spelled consistently.** In the national list 67 normalised name tokens occur in two or more raw spellings (minority spellings are 1.25% of all core-chain token occurrences). Examples (raw counts): احمد 1,737 against أحمد 58; ابراهيم 452, إبراهيم 110 and أبراهيم 3; اسماعيل 148 against إسماعيل 53; علي 1,177 against على 5. About one Ismail in four and one Ibrahim in five is spelled with a hamza.
2. **The folding changes a lot of strings.** 25.6% of the national list core chains contain at least one letter that the fold table of 6.4 rewrites (mostly ى, ة and hamza forms, which are correct spellings), so raw string equality between a list and a customer file will miss many true matches even when both are "correct".
3. **Compounds go both ways.** The national list almost always writes عبد as a separate word (the next word is الله in 43% of cases, then الرحمن, الكريم, القادر, الرزاق, العزيز), and 25.7% of its names (1,610 of 6,253) contain an Abd compound; customers will write both forms, so the compound must be joined or split canonically (6.3).
4. **Aliases hide in the name field** (26.9% of national-list names): the parser must read the Iraqi list formats "... المكنى أبو X", "(المكنى أبو X)" and "(أبو X)" and move the segment to the kunya field before matching. If it does not, the last token of the name is the kunya's name instead of the family or tribe, which corrupts every chain statistic and every token alignment.
5. **Persian and Kurdish letters are absent from the official Iraqi list but will be present in customer data** (section 4.3).

### 6.2 Unicode normalisation pitfalls (tested with Python 3.14, Unicode 16.0, 2026-10-06)

| Pitfall | Test result | Consequence |
|---|---|---|
| NFC and NFKC keep أ إ آ ؤ ئ composed | NFC leaves U+0623 unchanged; **NFD splits it into U+0627 plus U+0654** (hamza above); U+0625 into U+0627 plus U+0655; U+0622 into U+0627 plus U+0653; ؤ into U+0648 plus U+0654; ئ into U+064A plus U+0654 | "NFD then drop combining marks (category Mn)" removes hamza from alef, waw and ya in one pass: أحمد to احمد, إبراهيم to ابراهيم, آمنة to امنة, مؤيد to مويد, عائشة to عايشة |
| Standalone hamza has no decomposition | U+0621 stays; رؤوف becomes **رووف** while the alternative spelling رءوف stays رءوف (or روف once ء is dropped) | The two spellings of Rauf do not converge unless the hamza rule is explicit; this is the kind of case the golden test set must hold |
| Letter variants are not Unicode-equivalent | ي U+064A, ى U+0649, ی U+06CC, ێ U+06CE are four different code points with no decomposition; likewise ك U+0643 and ک U+06A9; ه U+0647, ە U+06D5, ھ U+06BE, ہ U+06C1; ة U+0629 | Unicode normalisation does **not** unify them: علي typed on an Arabic keyboard and علی typed on a Persian or Kurdish keyboard are different strings after NFC and after NFKC (tested). An explicit fold table is required |
| Presentation forms | **NFKC** maps U+FEFB (lam-alef ligature) to ل plus ا, U+FDF2 (Allah ligature) to الله, U+FE8E and U+FEE4 to the base letters; U+FDFA expands to an 18-character phrase; NFC does nothing | NFKC is needed, and it changed exactly 6 of the 329 UN names. CAMeL Tools maps U+FDFC and U+FDFD by hand before NFKC (tested: NFKC turns the rial sign U+FDFC into ریال with a Persian ya, and leaves the basmala ligature U+FDFD unchanged) |
| Digits | Arabic-Indic U+0660 to U+0669 and Extended Arabic-Indic U+06F0 to U+06F9 keep their code points under NFKC | ID numbers, dates and phone numbers typed in Eastern digits will not match ASCII digits unless mapped explicitly |
| Invisible format characters | ZWNJ U+200C, ZWJ U+200D, LRM U+200E, RLM U+200F, embeddings U+202A to U+202E, isolates U+2066 to U+2069, BOM U+FEFF and Arabic Letter Mark U+061C are category Cf; NBSP is Zs | Strip Cf, map Zs to a space. Kurdish text and converted legacy text carry ZWNJ in the middle of words (section 4.2) |
| Tatweel and marks | Tatweel U+0640 is category **Lm** (not Mn), so combining-mark stripping does not remove it; dagger alef U+0670 and the Quranic marks are Mn | Remove tatweel explicitly |
| Case folding of Latin Turkish and Kurdish letters | `"İbrahim".casefold()` gives "i̇brahim" (i plus U+0307); `"ß".casefold()` gives "ss"; ş ç ê î û survive | Use an explicit fold for İ, then strip combining marks only in the ASCII-folded form, never in the stored form |
| Measuring length | Code points, not UTF-8 bytes or UTF-16 units | Edit distances and thresholds are defined over Unicode scalar values after normalisation |

### 6.3 Compounds, particles and token boundaries

- **Abd compounds** (عبد الله, عبد الرحمن, عبد الحسين, عبد الزهرة, عبد الأمير ...): join "عبد" and the next word into one token for matching (عبدالله), keep the separate-word spelling for display. In Latin, "Abdul Rahman", "Abd al-Rahman", "Abdelrahman", "Abdurrahman" and "Abderrahman" all appear (15 written forms of this one compound in 78 UN occurrences, Table 8.1e), so the same join must apply on the Latin side (section 7.2). The set of valid second elements is a closed, countable lexicon (the divine names plus the Shia compounds).
- **X al-Din compounds** (صلاح الدين, نور الدين, سيف الدين): written with a separate الدين in the UN Arabic names (11 cases, never glued); join as one token.
- **Definite article**: attached to the next word in 8,013 of 8,027 uses in the national list: strip a leading ال from tokens longer than three letters and drop a standalone ال. In Latin the article appears as "al-", "al ", "el-", "el " and glued forms (Table 8.1f); "as-/ar-/ad-" sun-letter spellings occur too (about 1% of the article tokens in the Wikidata labels).
- **Kunya**: a standalone أبو or ام followed by a name starts a kunya; join them (أبوعلي) inside the kunya field only.
- **Particles** (بن, ابن, بنت, آل): 20 uses of آل ("family of") and almost no بن in the Iraqi list, so Iraqi chains are not marked by nasab particles (section 2.1); treat them as removable when present.
- **Tokenise on whitespace after normalisation**, never by Unicode word-break rules alone: UAX 29 splits Latin "al-Rahman" at the hyphen but not Arabic words, so Arabic and Latin names would be segmented by different logic.

### 6.4 Recommended Arabic normalisation pipeline (N-steps)

Keep four forms of every name: **raw**; **light** (steps N1 to N4 only: diacritics and tatweel removed, hamza kept); **folded** (all steps); and the **key** (consonant key of section 7.4). Exact matches on raw or light forms rank above matches that needed a fold.

| Step | Operation |
|---|---|
| N1 | Validate UTF-8, keep raw, trim |
| N2 | Map U+FDFC and U+FDFD by hand, apply NFKC |
| N3 | Remove Cf characters, map Zs and control characters to a space, collapse spaces |
| N4 | Remove tatweel; NFD, remove Mn (diacritics, dagger alef, Quranic marks) except keep hamza marks in the *light* form |
| N5 | Hamza and madda fold: أ إ آ ٱ to ا; ؤ to و; ئ to ي; drop ء (light form keeps them) |
| N6 | Letter folds: ى to ي; ة to ه; Persian and Kurdish table of section 4.3 (ی ێ ې ے to ي; ک to ك; ە ھ ہ to ه; ۆ ۇ ۋ ۊ to و; ڕ ڑ to ر; ڵ to ل; ڤ to ف; پ to ب; چ to ج; ژ to ز; گ to ك) |
| N7 | Digits U+0660-0669 and U+06F0-06F9 to ASCII; Arabic comma and punctuation to ASCII or space |
| N8 | Alias segments (introduced by المكنى, الملقب, المعروف, المدعو or a parenthesis) to the alias or kunya field; remaining punctuation to a space |
| N9 | Tokenise on spaces; classify tokens (title, particle, kunya, compound, article) with the lexicons of section 2.4 |
| N10 | Strip the article; join Abd, al-Din and kunya compounds; map known spelling variants through the lexicon |
| N11 | Emit tokens, chain role of each token, fold flags and the four forms |

Folds that merge different names (accept for blocking, penalise in scoring): ة and ه (عبدة and عبده), ى and ي (علا and علي), the hamza folds (سماء and سما), گ and ك. Each fold carries a small score penalty (for example 0.02 to 0.05) applied once per name when the raw tokens differ, so a raw match outranks a folded one.

### 6.5 The Iraqi national list as a data source: how it was read, and a defect found

The IQTFS web application (file 01 section 6) calls `GET https://api.aml.iq/api/localsanctionslist/getall/{page}` with the parameters `Type` (0 individuals, 1 entities), `Name`, `DecisionYears`, `SortByDate`, `IsAsc` and `IsExcelExport`, found in the application's JavaScript bundle. Tested on 2026-10-06, without authentication:

- **Paging is unreliable.** Walking pages 1 to 628 returned 6,280 rows but only **4,467 distinct ids**: the ordering is by creation time and 1,496 rows share one timestamp, so offset paging repeats some rows (some ids appear in up to 49 pages) and misses others (about 29% of the list). A re-fetch of the same page returns the same rows, so the defect is stable and silent.
- **The Excel export returns everything in one call.** `getall/1?Type=0&Name=&IsExcelExport=true` returned a 382 KB .xlsx with all **6,253 individuals** (columns Name, Type EN/AR/KU, Alias, Birth Year, Decision Number, Mother Name, Decision Year, Is Excluded, Created At; no id column); entities (27) come with `Type=1`. All rows have Is Excluded = No on that date; 539 groups of exact duplicate rows (same name, mother and birth year; 1,080 rows) exist, leaving about 5,700 distinct persons after normalisation (5,698 by the core chain used in section 8.2).
- Data defects seen: two impossible birth years (198 and 28138), 4.1% of entries without a birth year, 24 without a mother's name, 3 aliases only.

Implication for AKR-TMS (cross-reference file 01 section 6): **never ingest the national list by walking pages**; use the export or, better, an agreed bulk or API-key feed from the Office; always reconcile the row count with `rowCount`, de-duplicate identical rows, and alert when the distinct count falls. The undocumented behaviour of a public web service is not a supported interface, so the formal feed must be requested from the AML/CFT Office.

### 6.6 AKR-TMS hooks

- **A single normalisation library in Rust** (called by ingestion, batch screening and the phase-2 API) with a table of about 80 letter mappings plus the `unicode-normalization` crate for NFKC and NFD, versioned together with a golden test file of 300 or more input and output pairs taken from section 6.1 and the Kurdish cases of section 4.
- **Idempotence and stability tests**: `normalise(normalise(x)) == normalise(x)`; no change to results between releases unless the changelog says so; pin the Unicode version.
- **Per-bank data-quality profile at onboarding** (first run of the discovery checklist, section 3.5): share of names with presentation forms, ZWNJ, Persian or Kurdish letters, double spaces, parentheses, Latin letters inside Arabic fields, Eastern digits, joined or split Abd compounds, hamza and ya variants. The profile sets the expected recall loss of exact matching and justifies the fuzzy settings to the bank.
- **Fold penalty table** in the screening configuration (Wolfsberg: documented configuration, file 04 section 2d).
- **Alias extraction from the name field** for the Iraqi list format (المكنى, أبو: 26.9% of its rows) and for customer names that carry a parenthesised nickname.
- **Ingestion contract for the national list**: use bulk or API-key feed; completeness check against `rowCount`; duplicate removal; delta detection by (name, mother, birth year, decision number); keep every version (file 04 section 4d).

## 7. Latin-side variation and cross-script matching

### 7.1 What the Latin side looks like (measured)

The same three data sets give the picture of Latin names derived from Arabic ones.

- **UN list** (Tables 8.1e and 8.1f): Muhammad in 13 spellings, Abd al-Rahman in 15 written forms, Hussein in 11 spellings; the definite article as "AL-X" (72%), "AL X" (13%), "EL X" (7%), glued (7%); Abd compounds separate (48%), glued (45%) or hyphenated (6%).
- **Wikidata labels of 5,042 Iraqi persons** (Arabic label and English label of the same person; CC0): the Arabic and English labels have the **same number of words in 81.8%** of cases and a different number in 18.2% (English one word shorter in 448 cases, one word longer in 310); **23.2%** of the English labels contain a non-ASCII character and 14.8% carry ayn or hamza modifier marks (ALA-LC style). The article appears as "al-X" in 1,871 tokens (88%), "al X" in 215 (10%), "el X" or "el-X" in 16 and in a sun-letter form ("as-", "ar-", "ad-", "an-") in 20 (1%). Among 714 Arabic names containing an Abd word, the English label has "Abdul X" in 219, "Abd (al-)X" in 82, "Abdel X" in 30, a glued "AbdulX" form in 112 and another pattern in 271.
- Spelling choices follow the **writer's source and habit**: library-style forms with ayn marks (ʻAbd al-Karim, ʻAli, ʻUmar) in labels derived from catalogues, English-press forms (Mohammed, Hussein, Haider) and Maghreb forms (Ben, Ould, Abdel) in UN entries for North African listees. Which style a payment message uses depends on who typed it, not on the customer.

### 7.2 Recommended Latin normalisation (L-steps)

| Step | Operation |
|---|---|
| L1 | NFKD, remove combining marks; map letters that do not decompose by table (ø, ł, đ, ð, þ, æ, œ, ħ, ŋ, ı) as the rigour-core table does; locale-free casefold; İ to i |
| L2 | Remove ayn and hamza marks and apostrophes (ʻ ʿ ʼ ʾ ' ’ ` ´) but record that one was present ("Sa'd" and "Saad" differ by that flag only) |
| L3 | Hyphens, dots and commas to spaces; remember the comma: "AL-DULAIMI, AHMED" is family name first |
| L4 | Titles removed (Mr, Mrs, Dr, Eng, Sayyid, Sayed, Seyyed, Sheikh, Hajj, Hajji, Haj, Mulla); suffixes removed (Jr) |
| L5 | Nasab markers (bin, ibn, ben, bint, binti, ould, walad) neutral or removed |
| L6 | Definite-article particles (al, el, ul, ar, as, ad, an, ash, at, az, adh, ath), separate or hyphenated or glued: strip when followed by three or more letters; for glued forms test against the list of known tokens |
| L7 | Abd compounds: join abd, abdu, abdul, abdel, abdal, abdur, abder and the following token (with its own article) into one token, then map the second element through the compound lexicon so that Abdurrahman, Abderrahman, Abdelrahman, Abd al-Rahman and Abdulrahman give one key |
| L8 | Kunya: abu, abou, abo, abi, umm, um, oum plus the next token to the kunya field |
| L9 | Arabizi digits inside a token that also has letters (7ussein, 3ali, 5aled, 9asim): 2 hamza, 3 ayn, 5 kh, 6 t, 7 h, 8 gh, 9 q or s; only in name fields |
| L10 | Emit the consonant key of 7.4 for each token and for the whole name |

Latin names must also be **re-assembled** before step L1 when they come from multi-line payment fields (section 5.2), and long chains cut at 35 characters must be flagged as truncated.

### 7.3 Four ways to bridge scripts

| Approach | How it works | Strengths | Weaknesses and evidence |
|---|---|---|---|
| **A. Transliterate everything to Latin** (the OpenSanctions route for broad scripts) | Arabic to Latin by an ICU transform or a table, then Latin matching and phonetic keys | One algorithm; works for any script pair | Arabic script omits short vowels, so the output is a skeleton ("mhmd") that Latin phonetic algorithms handle badly; CLDR and ICU4X transliterators exist but rigour-core, OpenSanctions' own Rust library, **deliberately excludes Arabic** from its fast transliteration (only Latin, Cyrillic, Greek, Armenian, Georgian and Hangul are admitted; "broad-script transliteration (Han / Arabic / Devanagari / ...) is deliberately not in Rust") |
| **B. Back-transliterate Latin to Arabic** | Generate the plausible Arabic spellings of a Latin name from a lexicon of Arabic names with their Latin variants | Exact, fast comparison on the Arabic normalised form, which is the system of record in Iraq; best fit for the Arabic-only national list | Needs a good lexicon (about 3,000 given names, plus tribes); unknown names fall back to rules |
| **C. Shared consonant key** (used in the experiments below) | Map both scripts to one coarse consonant alphabet, compare with a weighted edit distance | Small, explainable, no model; strong on full names | Vowels and some letter distinctions are lost (Table 8.1i); needs token alignment on top |
| **D. Learned name matcher** (Babel Street Rosette, IBM GNR, NetOwl; or an in-house model) | Statistical or neural model trained on name pairs | Handles unusual variants | Needs training pairs; opaque unless built carefully; vendor claims not independently tested here (section 10.4) |

**Experiments (this research, 2026-10-06, Python prototype, consonant key of 7.4 with a weighted edit distance; truth = the UN's own Arabic and Latin names of the same person; the UN transliterations are editorial and consistent, so real bank data will score lower):**

| Test | Queries | Candidate pool | Top-1 | Top-5 | Competing candidates scoring 0.90 or more |
|---|---|---|---|---|---|
| Arabic full name to Latin primary names and aliases (2,894 strings of 736 records) | 328 Arabic-named UN records | 736 records | **98.5%** (Iraq list 100%, Al-Qaida/Libya/Yemen 98.4%, Taliban 97.8%) | 99.7% | n/a |
| same, query = first two Arabic tokens | 193 Arab-world records | 736 records | 31.1% | 40.4% | n/a |
| same, query = first three Arabic tokens | 193 | 736 records | 58.5% | 75.1% | n/a |
| Latin full name to **6,034 Arabic names** (328 UN names plus 5,706 distinct national-list persons, de-duplicated by name, mother and birth year) | 193 Arab-world UN records | 6,034 | **99.5%** | 100% | **0.00 on average** (0.05 at 0.85, 0.32 at 0.80) |
| Latin first three tokens to the same pool | 193 | 6,034 | 70.5% | 84.5% | 0.00 |
| Latin first two tokens | 193 | 6,034 | 33.2% | 46.6% | 0.02 |

Reading: **with the full chain a simple consonant key already works across scripts, and 5,700 Arabic distractors hardly compete** (0.32 candidates at 0.80); the collapse on truncated names is not a script problem but a *partial-chain* problem (whole-name similarity penalises the missing tokens), which is why section 13 uses token alignment with explicit rules for missing parts, and why secondary identifiers carry the weight for short names (section 12).

Two quality checks on key-equality blocking (Wikidata: 10,820 aligned Arabic and Latin token pairs, 2,434 Arabic name classes, 3,507 Latin spellings): requiring the Arabic and Latin consonant keys to be **equal** found only 35.7% of true pairs with the strict key and 87.2% with the loose key (q, g, k merged; th and t merged; dh and d merged; w, y and ayn dropped), and the loose key was right only about half of the time (class-pair precision 0.52). **Key equality is good enough for blocking only in its loose form and must be followed by a graded score.**

### 7.4 The consonant key used in the experiments (specification)

| Arabic letter | Key | Latin spellings mapped to it |
|---|---|---|
| ب | B | b, p |
| ت, ط | T | t |
| ث | θ | th |
| ج | J | j, dj |
| ح, ه | H | h |
| خ | X | kh, x |
| د, ض | D | d |
| ذ | ð | dh |
| ر | R | r |
| ز, ظ | Z | z |
| س, ص | S | s, ss |
| ش | C | sh, ch |
| ع | E | apostrophe (ayn or hamza) |
| غ | G | gh, g |
| ف | F | f, v |
| ق | Q | q |
| ك | K | k, c |
| ل | L | l |
| م | M | m |
| ن | N | n |
| و | W | w (vowels a e i o u are dropped) |
| ي | Y | y |
| ا and hamza | dropped | vowels dropped |

Substitution costs: Q-K 0.35, Q-G 0.30, G-K 0.35, θ-T 0.30, θ-S 0.35, ð-D 0.30, ð-Z 0.30, J-G 0.45, C-K 0.50, C-S 0.40, X-K 0.50, X-H 0.50, B-F 0.60, S-Z 0.50, D-T 0.50, E-H 0.60; insertion or deletion of W, Y or E costs 0.4, of any other symbol 1.0; similarity is 1 minus distance over the longer length. Doubled letters are collapsed. The table is a starting point: the costs must be re-estimated on the bank's own Arabic and Latin pairs (hook below).

### 7.5 AKR-TMS hooks

- **Dual indexing**: every list name and alias (Arabic or Latin) is stored with its normalised tokens, its consonant key and its script; every customer name likewise. Comparison picks the pair of forms in the same script when one exists (Arabic against Arabic, exact after normalisation), and the key otherwise.
- **Given-name lexicon** of about 3,000 Arabic, Kurdish, Syriac and Armenian names with their Arabic spellings, common Latin spellings and keys, built from the national list, Wikidata (CC0) and the bank's own pairs; it drives back-transliteration (approach B) and is the test oracle.
- **Learn the key costs from data**: estimate the substitution and deletion costs from Arabic and Latin pairs of the same customer in the bank's CBS and payments (with governance approval), so the table reflects how that bank's staff actually spell.
- **Per-list script plan**: Arabic-only lists (national list, many PEP entries): Latin queries are back-transliterated and also compared by key; Latin-only lists (OFAC, EU, UK, parts of UN): Arabic customer names are compared through generated Latin variants and the key; lists with both (UN originals): compare same-script first.
- **Always keep the source script and the alignment** so the analyst screen can show "Arabic against Arabic (exact after normalisation)" or "Arabic against Latin alias (key distance 0.12)".
- **Do not rely on Latin phonetic algorithms** (section 10.2).

## 8. The very-common-names problem and token weighting (with UN list statistics)

### 8.1 Measured on the UN consolidated list (downloaded and parsed 2026-10-06)

**Method.** The UN XML (`consolidated.xml`, `dateGenerated` 2026-10-06T14:06Z, 736 individuals and 274 entities, see file 04 section 4a) was parsed with Python 3.14 (standard library only: `xml.etree`, `unicodedata`). Tokens are the whitespace-separated words of FIRST_NAME to FOURTH_NAME (and of aliases), upper-cased; punctuation other than hyphen and apostrophe is removed. Scripts are detected by Unicode block. The **Arab-world cohort** is 194 individuals: all 65 on the Iraq (1518) list plus the 129 individuals on the Al-Qaida, Libya and Yemen lists whose original-script name is Arabic. "Family" counts (Muhammad, Hussein, and so on) use explicit spelling lists and are therefore lower bounds. No individual's name is reproduced in this file: only generic tokens, counts and ratios. (Scratch scripts: `scratchpad/n22/un_stats1..10.py`; outputs `out-un_stats*.txt`.)

**Table 8.1a. Script coverage**

| Item | Result |
|---|---|
| Individuals with a name in original script | 338 of 736: 329 Arabic script, 9 Cyrillic |
| Entities with a name in original script | 41 of 274: 38 Arabic, 2 Cyrillic, 1 Latin |
| Arabic-script names by list | Iraq 65 of 65; Taliban 135 of 135 (Pashto and Dari); Al-Qaida 117 of 248 (122 have no original-script field at all); Yemen 8 of 10; Libya 4 of 28; every other list 0 |
| Aliases of individuals | 2,163 in total: 2,158 Latin, 5 Arabic. Arab-world cohort: 688 aliases, 685 Latin and only 3 Arabic |

The UN gives one Arabic spelling per person (when it gives any) and about 3.5 Latin aliases per person in the Arab-world cohort. A customer name held in Arabic in the CBS therefore has to be compared with Latin aliases through a script bridge (section 7), not only with the Arabic original.

**Table 8.1b. Name length (tokens)**

| Name set | n | Mean tokens | Distribution (tokens: number of names) |
|---|---|---|---|
| UN Arabic-script names, all lists | 329 | 3.79 (median 4, max 8) | 1:1, 2:59, 3:86, 4:88, 5:60, 6:27, 7:7, 8:1 |
| UN Iraq (1518) list, Arabic | 65 | 4.02 | 2:5, 3:14, 4:26, 5:15, 6:5 |
| UN Iraq (1518) list, Latin primary name | 65 | 3.83 | 2:6, 3:18, 4:24, 5:15, 6:2 |
| UN Arab-world cohort, Latin primary name | 194 | 4.00 | not tabulated |
| UN all individuals, Latin primary name | 736 | 3.23 | 1:5, 2:219, 3:279, 4:124, 5:67, 6:31, 7:9, 8:1, 9:1 |
| Iraqi national list (this research, section 6.5) | 6,253 rows | core chain 5.05 (median 5), 5.80 with the alias segment | core chain 3:108, 4:978, 5:3,844, 6:1,133, 7 or more: 188; given name, father, grandfather, great-grandfather, tribe or family |

For the 329 records that have both an Arabic and a Latin name, the token counts are equal in 224 (68%), the Latin name has one token fewer in 63 (19%), one more in 33 (10%) and differs by two or more in 9 (3%). **In about one record in three the two scripts do not split the name into the same number of words** (Abd al-Rahman written as two words or one, Abu as a separate word or glued, a missing grandfather), so token alignment must not assume equal counts.

**Table 8.1c. How concentrated are the tokens?**

- All 736 individuals, Latin primary names: 2,377 token occurrences, 1,342 distinct tokens; 1,072 tokens (80%) occur once; the 10 most frequent tokens cover 15.0% of occurrences, the top 50 cover 31.4%, the top 100 cover 39.5%. The distribution has a very long tail: most tokens are rare, a few are everywhere.
- Arab-world cohort: 776 occurrences, 426 distinct tokens, token entropy 7.93 bits. Self-information of a token: 4.85 bits for the most frequent given name in the cohort, 5.60 bits for one spelling of Muhammad or for Ali, 9.60 bits for a token seen once. A common token is worth roughly half the evidence of a rare one in this list, and much less than that in a bank's customer base where Muhammad, Ali and Hussein are far more frequent (section 8.2).

**Table 8.1d. Document frequency: share of individuals whose primary name or any Latin alias contains the token family**

| Token family | Arab-world cohort (n=194) | Iraq 1518 list (n=65) | All individuals (n=736) |
|---|---|---|---|
| any token starting "AL-" | 65.5% | 78.5% | 25.3% |
| Abd- compound (Abd, Abdul, Abdel, or merged) | 36.1% | 23.1% | 24.5% |
| Abu / Abou (kunya) | 33.5% | 4.6% | 19.4% |
| Muhammad family (Muhammad, Mohammad, Mohamed, Mohammed, Mohamad, Muhammed, Mohd) | 29.4% | 9.2% | 25.7% |
| Ali | 16.5% | 10.8% | 7.6% |
| Ahmad / Ahmed | 16.0% | 9.2% | 8.6% |
| Hasan / Hassan | 15.5% | 32.3% | 5.4% |
| Ibrahim | 14.9% | 27.7% | 6.2% |
| Hussein family | 5.7% | 10.8% | 3.0% |
| Mahmud family (Mahmoud, Mahmud, Mahmood) | 3.6% | 3.1% | 1.8% |

One city-derived nisba (a toponymic that thousands of ordinary Iraqis carry) is present in 34 of the 65 Iraq-list names (52%). A customer who carries that nisba shares a token with half of the 1518 list, which is exactly why a nisba or tribe token must never be allowed to create an alert on its own.

**Table 8.1e. Spellings of the same name inside one file (names plus Latin aliases, all individuals)**

| Name | Spellings found (occurrences) |
|---|---|
| Muhammad (محمد) | 13 spellings in 427 occurrences: Muhammad 127, Mohammad 123, Mohamed 81, Mohammed 62, Muhammed 10, Mohamad 7, Muhamad 5, Mohd 4, Mouhamad 3, Mohmad 2, and three single-use forms |
| Abd al-Rahman (عبد الرحمن) | 15 written forms in 78 occurrences: "Abd al-Rahman" 22, "Abdul Rahman" 15, "Abdurrahman" 12, "Abdelrahman" 5, "Abdulrahman" 4, "Abderrahman" 4, "Abdurahman" 3, "Abderahman" 3, "Abdel Rahman" 2, "Abdal-Rahman" 2, "Abd-al-Rahman" 2, and four single-use forms |
| Hussein (حسين) | 11 spellings in 49 occurrences: Hussein 18, Husayn 10, Hussain 6, Husain 4, Hissene 2, Hosein 2, Hosain 2, Hossein 2, Hissein 1, Husein 1, Hussayn 1 |
| Hasan (حسن) | Hassan 58, Hasan 40, Hassen 2, Hassaan 2, Hassane 1 |
| Ahmad (أحمد) | Ahmed 83, Ahmad 67 |
| Omar (عمر) | Omar 47, Umar 31, Oumar 7, Omer 1, Umer 1 |
| Khalid (خالد) | Khaled 13, Khalid 8 |
| Yusuf (يوسف) | Yusuf 10, Youssef 6, Yusef 4, Yusif 3, Joseph 3, Yousef 1, Yousif 1 |
| Mustafa (مصطفى) | Mustafa 18, Mostafa 11, Mustapha 4 |
| Abdullah (عبد الله) | Abdullah 46, Abdallah 33, Abdellah 5, Abdillah 2, Abdulla 2, Abdalla 2, Abdoullah 2, and Abdallah with a leading apostrophe for the letter ayn 7 |

There is **no canonical Latin spelling to normalise to**: Ahmed against Ahmad is 55:45, Hassan against Hasan 59:41. Choosing one spelling per name would be wrong for about half of all records. A bank's SWIFT data will use all of these forms plus typing errors.

**Table 8.1f. Particles and compounds in Latin and Arabic (UN file)**

| Pattern | Counts |
|---|---|
| Definite article in Latin (all individuals, names and aliases, about 735 tokens) | "AL-X" hyphenated 530 (72%); "AL" as a separate token 97 (13%); "EL" as a separate token 51 (7%); "EL-X" 4; glued "ALX/ELX" about 52 (7%, an upper bound); assimilated forms (AS-, AR-, AD-) 1 |
| Abd compounds in Latin (438 occurrences) | separate token plus next word 212 (48%: ABDUL 140, ABD 59, ABDEL 9, ABDUR 3, ABDAL 1); glued into one token 199 (45%); hyphenated "ABD-AL-X" 27 (6%) |
| Abd compounds in Arabic names | "عبد" as a separate word 66 (70%), glued to the next word 28 (30%) |
| "al-Din" compounds in Arabic names | "الدين" as a separate word 11, never glued in this file |
| Nasab and kunya markers, Arab-world names and aliases (Latin) | Abu 88, Abou 35, Abo 2 (kunya); Ben 56, Bin 14, Ibn 1, Walad 1, Ould 23 ("son of": Maghreb and Mauritania use Ben and Ould, the Mashriq uses Bin and Ibn) |
| Arabic tokens in the 329 Arabic names | 256 tokens carry an attached "ال"; "الله" stands alone 45 times; "بن / ابن / بنت" 36; "أبو" 6 (4 separate, 2 glued); "آل" 0 |

**Table 8.1g. Orthography inside the 373 Arabic-script strings of the UN file (329 individual names, 38 entity names, 6 aliases; 87 distinct code points)**

| Feature | Result |
|---|---|
| Alef forms | plain ا 922, أ 43, إ 29, آ 23. In the 194 Arab-world names, hamza and madda forms are 58 of 555 alef-type letters (10.5%) |
| Ya and alif maqsura | ي 455 and ى 29 (6%) |
| Persian ya (ی U+06CC) and keheh (ک U+06A9) | 156 and 21 occurrences. They sit in 89 of 135 Taliban names (Pashto and Dari), in 9 of 117 Al-Qaida names, and in none of the 65 Iraq names; 12 uses of Persian ya occur inside names of the Arabic-language lists |
| Other non-Arabic letters | peh پ 4, tcheh چ 2, gaf گ 8, heh doachashmee ھ 3, heh goal ہ 2, rreh ڑ 1 |
| Arabic Presentation Forms (U+FB50-FDFF, U+FE70-FEFF) | 6 names (pasted from PDF or legacy systems). NFC changes none, **NFKC changes exactly these 6** |
| Tatweel (U+0640) | 2 names |
| Combining marks (harakat) | 1 name (a damma) |
| Line break inside a name | 1 name; 1 more has a double or trailing space |
| Not seen | ZWNJ/ZWJ, Arabic-Indic digits, Latin letters inside individual names (2 entity names mix Latin letters into Arabic text) |

Even a curated international list shows these defects in about 3% of its Arabic strings (and Persian letters in many of the non-Arab ones). CBS data typed by branch staff on different keyboards and migrated from older systems will be much dirtier: **normalisation is not optional** (section 6).

**Table 8.1h. Aliases, kunyas, dates and identifiers**

| Item | Iraq 1518 (n=65) | Arab-world cohort (n=194) | All (n=736) |
|---|---|---|---|
| Individuals with at least one alias | 23 (35%) | 147 (76%) | 567 (77%) |
| Aliases in total (mean per individual) | 55 (0.85) | 688 (3.55, max 17) | 2,163 (2.94, max 23) |
| Alias quality Good / Low | 49 / 6 | 480 / 208 (30% Low) | 1,535 / 628 (29% Low) |
| Aliases that start with a kunya particle (Abu, Abou, Abo, Abi, Umm, Um, Oum) | 3 | 108 (15.7%); 70 of them Low | 265 (12.3%); 168 Low |
| Individuals with a kunya alias | 2 (3%) | 65 (34%) | 140 (19%) |
| Latin aliases that are pure spelling variants of the primary name (same consonant skeleton) | 20 of 55 | 66 of 685 (9.6%) | 220 of 2,158 (10.2%) |
| Single-token aliases | 3 (5%) | 70 (10%) | 266 (12%) |
| Date of birth: full date | 15 (23%) | 114 (59%) | 419 (57%) |
| Date of birth: year only (exact 34 or approximate 9 on the Iraq list) | 43 (66%) | 70 (36%) | 240 (33%) |
| Date of birth: year range (BETWEEN) | 0 | 1 | 24 (3%) |
| Date of birth: none | 7 (11%) | 9 (5%) | 53 (7%) |
| Place of birth with a city or province | 42 (65%) | 138 (71%) | 487 (66%) |
| At least one identity document record (passport or national ID number) | 1 (2%) | 71 (37%) | 302 (41%) |

Reading: (1) **in the Arab-world cohort about 90% of the Latin aliases are not pure spelling variants of the primary name** (the share is 64% on the Iraq list alone): they are partial names, reordered or shortened token sets, kunyas and noms de guerre, so a spelling-variant generator does not replace alias data (the "pure variant" test is a crude one, equal consonant skeleton of the whole string, so the true share of non-variants is a little lower); (2) a third of Arab-world individuals carry a kunya alias, and 70 of the 108 kunya aliases are rated Low by the UN itself; (3) kunyas are weak identifiers: the 74 distinct kunya strings (first two tokens) in the cohort are shared by at most 3 individuals each, but those shared strings are generic ones such as "Abu Ali" and "Abu Ahmad" that any customer can carry; (4) the Iraq list has year-only birth dates for two thirds of its entries and almost no document numbers, which is the same situation as the Iraqi national list (birth year only, file 01 section 6); (5) **the UN list has no structured mother's name**: 16 of 736 narrative comments (2.2%) say "mother's name is" and 17 say "father's name is", while the Iraqi national list carries the mother's name in every sampled row.

**Table 8.1i. Skeleton keys over-merge**

A consonant skeleton (drop vowels and semi-vowels, collapse doubles) is a popular way to bridge Latin spellings. On this file it merges different names:

| Skeleton | What it merges |
|---|---|
| MHMD | Muhammad (محمد) and Mahmud (محمود): 19 of the 153 Arab-world tokens with this skeleton are Mahmud forms (12%) |
| HSN | at least four different Arabic names (حسن Hasan, حسين Husayn, إحسان Ihsan, أحسن Ahsan), together spelled in 30 different ways in the file (nisba forms included) |
| HMD | Ahmad, Hamid, Hamad, Hammad, Hamdi, Hammud |
| BDLH | Abdullah (عبد الله), Abd al-Hayy (عبد الحي) and Ubaydullah (عبيد الله) |

A skeleton is useful as a **candidate-generation key** (high recall) but never as a score (low precision); the score must see the vowels, the long-vowel letters of the Arabic original and the other tokens.

**Entities.** 274 entities have a mean of 2.2 aliases (max 27); 38 have an Arabic name. 122 of 274 primary names (45%) contain a generic or legal-form word; the most frequent entity tokens are COMPANY (32), TRADING (22), GROUP (20), CORPORATION (20), SHIPPING (18), ISLAMIC (18), INDUSTRIES (16), CO (14), BANK (12), LTD (11), AL (11). Entity matching needs the same token weighting, plus legal-form and generic-word handling in both scripts (شركة, مؤسسة, مصرف, المحدودة, للتجارة العامة and similar).

### 8.2 Measured on the Iraqi national list: how concentrated are Iraqi names?

The national list (6,253 individual rows, 5,698 distinct persons after removing exact duplicates; read from the Excel export, section 6.5) is the best available sample of Iraqi name structure. It is **not a random sample of Iraqis**: its persons are young (birth-year deciles 1974 to 1998, median about 1990; 48% born in the 1990s) and concentrated by region and family, so figures below show the *order of magnitude* of the problem, not exact population rates. Family and tribe tokens are reported in aggregate only, because a tribe is shared by millions of ordinary Iraqis and a share on a list says nothing about them.

**Table 8.2a. Concentration of tokens (core chain, alias segments removed)**

| Measure | Iraqi national list | UN list (Table 8.1c) |
|---|---|---|
| Name tokens per person | core chain 5.05 on average (median 5; 61.5% have exactly 5; 95.2% have 4 to 6) | Latin 3.23, Arabic 3.79 |
| Token occurrences / distinct tokens | 31,593 / 1,857 | 2,377 / 1,342 (Latin) |
| Token entropy | 7.69 bits | 7.93 bits (Arab-world cohort) |
| The 5 / 10 / 20 / 50 / 100 most frequent tokens cover | **24.7% / 36.1% / 46.6% / 60.3% / 70.6%** of all token occurrences | 9.3% / 15.0% / 21.6% / 31.4% / 39.5% |
| Tokens that occur only once | 45.0% of distinct tokens | 79.9% |
| Given name (first token): top five shares | 9.5%, 8.5%, 5.5% (the Abd compounds), 3.7%, 2.4%; top 10 cover 38.3%, top 25 cover 51.0%; 572 distinct | n/a |
| Last token of the core chain (family, tribe or nisba) | 731 distinct; the commonest covers 12.2% of persons, the top 5 cover 29.7%, the top 15 cover 47.2%; 77.2% begin with ال | n/a |

Iraqi chains are far more concentrated than the international UN list: **ten tokens make up more than a third of all name tokens**, so most fuzzy token matches are matches on one of a few very common words.

**Table 8.2b. How many different people share a key, and how many bits does each identifier carry?** (distinct persons N = 5,698; "P(pair)" is the probability that two random persons share the key; the bits are Shannon entropy, which saturates at log2(5,698) = 12.48 for long keys because of the sample size)

| Key | Distinct values | P(pair shares it) | Entropy (bits) | Persons sharing it with at least one other | Largest group |
|---|---|---|---|---|---|
| given name (token 1) | 572 | 2.49% | 7.05 | 95.9% | 540 |
| father (token 2) | 601 | 2.45% | 6.96 | 95.6% | 485 |
| grandfather (token 3) | 619 | 2.68% | 6.76 | 94.9% | 502 |
| last token (family, tribe) | 731 | 2.78% | 6.91 | 92.5% | 695 |
| **given name + father** | 3,567 | **0.106%** | 11.15 | 49.5% | 100 |
| **given + father + grandfather** | 5,234 | **0.0055%** | 12.28 | 13.6% | 16 |
| first four tokens | 5,609 | 0.0006% | 12.44 | 3.0% | 5 |
| full core chain | 5,687 | 0.00007% | 12.47 | 0.4% | 2 |
| birth year | 62 | 3.48% | 5.08 | 99.9% | 316 |
| mother's first name | 900 | 0.53% | 8.42 | 92.2% | 120 |
| **mother's full name** | 5,033 | **0.0067%** | 12.21 | 20.8% | 24 |
| 3-token chain + birth year | 5,611 | 0.00055% | 12.45 | 3.0% | 3 |
| 3-token chain + mother's full name | 5,667 | 0.00019% | 12.47 | 1.1% | 2 |
| 3-token chain + mother + birth year | 5,669 | 0.00018% | 12.47 | 1.0% | 2 |

What this means in customer-file terms, if the customer base had the same name distribution (extrapolated, and indicative only): for a bank with **1 million customers**, a listed person's given name plus father's name is shared on average by about **1,060 customers** (the single commonest pair by about 17,500), given name plus father plus grandfather by about **55**, four tokens by about 6, the full chain by under 1. The 3-token chain is shared by 13.6% of the list's persons even inside this small list. **A name alone does not identify a person in Iraq below four or five tokens**, and ordinary payment messages carry two or three. Birth year is a weak identifier (5 bits, and 80% of the listed persons were born between 1974 and 1998), the mother's full name is as strong as the whole three-token chain (12 bits), and the two together with the chain reach the sample's ceiling.

### 8.3 Token weighting: what the literature and open-source tools do

- **Frequency weighting is the classical answer.** Fellegi and Sunter showed that agreement on a rare value is stronger evidence than agreement on a common one; in the words of Cohen, Ravikumar and Fienberg (2003), under Fellegi and Sunter's assumptions "the incremental score for the odds ratio associated with AGREE(s, t, w) is simply − log P_D(w). In information retrieval terms, this is a simple un-normalized IDF weight." Splink's documentation puts it this way: "a match on an uncommon value provides stronger evidence of a true match than a match on a common value", implemented as term-frequency adjustments of the u-probabilities.
- **The best general-purpose name matcher in that comparison is a hybrid.** Cohen, Ravikumar and Fienberg (IIWeb 2003, IJCAI) compared edit distances, Jaro and Jaro-Winkler, token methods and hybrids on name-matching tasks: "Overall, the best-performing method is a hybrid scheme combining a TFIDF weighting scheme ... with the Jaro-Winkler string-distance scheme" (SoftTFIDF, Jaro-Winkler as the token similarity with threshold 0.9). They also note that Jaro and Jaro-Winkler "seem to be intended primarily for short strings (e.g., personal first or last names)".
- **OpenSanctions logic-v2** (nomenklatura, MIT licence) scores a name pair as a weighted average over aligned name parts and applies the following defaults (read from `logic_v2/model.py` and `names/match.py`): unmatched parts of the *query* weigh 0.8, unmatched parts of the *candidate* weigh 0.2, stop words 0.7, the **family name weighs 1.3 times**, and a qualifier layer adds or subtracts for supporting or contradicting evidence (country mismatch -0.2, birth-year disjoint -0.15, birth-day disjoint -0.25, gender mismatch -0.2). It tags name parts as GIVEN, MIDDLE, FAMILY, TRIBAL, PATRONYMIC, MATRONYMIC, HONORIFIC, NICK, STOP and others, which is exactly the vocabulary Iraqi names need (section 10.5).
- **Experiment in this research (section 12.4)**: in a prototype on the national list, IDF weights and the asymmetric treatment of unmatched parts changed name-only recall very little at a fixed alert budget (mean recall 0.43 to 0.45 across variants at a 2% alert budget); what changed it was **secondary identifiers** (0.84 with birth year and mother's name). Weights are still useful for explanation and for rare-token rules, but they must not be sold as the main defence against common names.

### 8.4 AKR-TMS hooks

- **Evidence model, not just a similarity score.** Express every alert as accumulated evidence in bits: each aligned token adds its information content (from the token frequencies of the target list and of the bank's own customers), each secondary identifier adds or subtracts, and an alert needs enough bits for the bank's customer count. A bank with N customers needs roughly log2(N / tolerated false matches per listed entry) bits: about 20 bits for 1 million customers and one false match per listed entry, about 27 bits for 0.01. The three-token chain carries about 12 bits in this sample, so **name-only matches on short chains cannot meet the bar for a large bank, which is the quantitative reason to require secondary identifiers or longer chains** (illustrative arithmetic with an independence assumption, to be validated on bank data).
- **Frequency tables per bank**: compute token and chain frequencies from the bank's own customer base (monthly), with the national-list tables as a prior for small banks; version them and show them in the explanation ("this token appears in 9% of your customers").
- **Minimum evidence rules** (configurable, documented): at least two aligned tokens (unless the query has one token), at least one token above an information threshold, and no alert on a family or tribe token plus one common given name alone.
- **Common-name watch list**: the 200 most frequent tokens per bank form the "stop-weight" list used for partial-chain rules; refresh quarterly.
- **Entity names** use the same logic with legal-form and generic-word weights (section 8.1, entities).
- **Do not use frequency to suppress true hits silently**: a rare list entry whose chain is made only of common tokens (for example Ahmad Muhammad Ali) is *more* dangerous, not less; route such entries to the "needs secondary identifiers" tier rather than lowering their score (section 13.5).


## 9. Secondary identifiers that cut false positives

### 9.1 What each list and each data source can offer

| Identifier | UN list (measured, Table 8.1h) | Iraqi national list (measured, section 8.2) | Bank side (CBS) | Payment messages |
|---|---|---|---|---|
| Date of birth | Arab-world cohort: full date 59%, year only 36%; Iraq list: full date 23%, year only 66%; **8.3% of full dates fall on 1 January** (40 of 483), against 0.27% expected by chance, a sign of default dates | **Year only**, present for 95.9% (two impossible values: 198 and 28138) | Unknown per bank (discovery) | R.16 revised (June 2025, to be implemented by end-2030): for cross-border payments above the threshold "originator date of birth if a natural person (year of birth if the full date is unknown)" (file 03). Beneficiary DOB not required |
| Place of birth | City or province for 65% of the Iraq list (free text such as "village X, near city Y", with qualifiers in 7 of 44 entries) | **Absent** | Often a governorate or city text | Not carried |
| Nationality | Present for 188 of 194 Arab-world individuals; 100% of the Iraq list | Not listed (all Iraqi) | "Iraqi" by default for most customers | Not carried (country of the bank and address only) |
| Mother's name | **No structured field**; narrative only (2.2%) | **99.6%**, 2.87 tokens on average | Unknown (discovery); standard on the national card and Iraqi KYC forms (section 3) | Never carried |
| Identity document numbers | Passport or national ID record for 2% of the Iraq list and 37% of the Arab-world cohort; **12 document records for the 76 Iraqi nationals** (4 passports, 8 national ID numbers) | **None** (no ID number column) | National card number, family number, passport number, residence card number (section 3) | Rarely (passport number in free text) |
| Gender | Field populated for only 61 of 736 individuals (4 of 194 Arab-world) | Not listed | Often present | Not carried |
| Aliases | Many (3.55 per Arab-world individual) | 3 of 6,253 in the alias field; **26.9% inside the name field** | Rare | Free text |

The Iraqi national list therefore offers **birth year and mother's name only**; the UN list offers birth date (mostly), nationality and sometimes a document number; neither offers a family number or a place of birth that a bank can verify. Sanctions lists of other regimes (OFAC, EU, UK) carry more fields (file 05), but for Iraqi customers the two lists that matter most are the weakest in identifiers.

### 9.2 How to use each identifier

**Date of birth.** Parse and store the precision: full date, year only, approximate ("circa"), range. Rules: equal full dates add evidence; year-only on one side is compared by year (equal, or within one year to allow for default dates and registry shifts); approximate or range dates use their tolerance; **treat 1 January and 1 July as probable default dates** (partial evidence only; 8.3% of the UN full dates are 1 January); a day-and-month swap (05/11 against 11/05) and a one-digit slip are soft matches. A clear disagreement (more than the tolerance, both dates verified) subtracts strongly. This follows the Wolfsberg PEP guidance: "auto-discounting only on reliable clear differences (e.g., birth year)" (file 04 section 2n).

**Mother's name.** Compare chains with the same alignment logic as the person's own name (section 13.5), because the mother's name is often given with fewer parts than the list holds. Agreement on a full three-token mother's chain is strong positive evidence (the pair-collision probability is 0.0067% in the national list, as strong as a full three-token chain); disagreement is strong negative evidence; a missing mother's name is neutral. In the Mandaean tradition the mother's name belongs to the person's own chain (section 4.1).

**Place of birth.** Resolve to an Iraqi governorate or district with a gazetteer (names in Arabic, Kurdish and Latin: GeoNames is CC BY) and use it as soft evidence only (a mismatch between two resolved governorates subtracts a little; agreement adds a little); free-text qualifiers ("near", "village") reduce confidence. Link nisbas to places (al-Mawsili to Mosul) to explain, not to score.

**Nationality.** Strong only when it contradicts: an Iraqi customer against a list entry of another nationality is weak evidence against; a foreign customer against an Iraqi-only list entry is stronger; "Iraqi" as a default value carries little evidence in the other direction (see logic-v2's country mismatch qualifier of -0.2).

**Document numbers.** Normalise (strip spaces and hyphens, map Eastern digits, upper-case letters), compare type and number exactly, and treat a match as the strongest positive evidence (OpenSanctions logic-v2 gives an identifier match weight 0.85, a name 1.0). A mismatch of the same document type is strong negative evidence. Passport numbers on Iraqi A-series passports are 9 alphanumeric characters (Wikipedia, Iraqi passport); the 18-character **family number** links relatives (file 01 section 7) and is used for family-cluster alerts, not for identity.

**Gender.** Arabic given names are gendered: a lexicon of Iraqi given names with a gender and a "unisex" flag lets a gender mismatch (قاسم listed, a woman customer) subtract evidence. Unisex names (for example علا) must be flagged to avoid wrong penalties; the gender field itself is almost never populated in the UN data.

**Age plausibility.** A listed person born in 1937 against a customer aged 25, or a PEP flagged deceased, is a contradiction; apply with care.

### 9.3 Measured effect of secondary identifiers (prototype, section 12.4)

At a fixed alert budget of 2% of ordinary customers (synthetic worst-case customers drawn from the national list's name distribution), mean recall on noisy variants of list persons rose from **0.44** (name only) to **0.69** with birth year, **0.71** with the mother's name, and **0.84** with both; for three-token names (given, father, grandfather) recall rose from **0.16 to 0.94**, and for two-token names from **0.02 to 0.71**. The same ranking holds at a 1% budget (0.36 against 0.79). Secondary identifiers are the main lever against common names, much more than token weighting.

### 9.4 Rules for combining and for missing data

1. **Tiers of evidence**: (a) document number match: escalate regardless of name score above a floor; (b) name chain plus mother's name plus birth year agreeing: alert at a low name threshold; (c) name only: alert only at a high threshold and only with enough tokens (section 8.4); (d) contradiction on a *reliable* identifier: auto-discount with a logged reason, never silently.
2. **Missing is neutral, contradiction is negative**: never penalise a missing field; never reward a default value ("Iraqi", 1 January).
3. **Track the source and quality** of every identifier (verified from the national card, typed by staff, copied from a payment) because the tolerance depends on it.
4. **Wolfsberg**: identifying information "helps adjudicate but need not be screened" (file 04 section 2d): show the identifiers side by side to the analyst and use them in the score, but do not turn a missing or weak identifier into a reason to suppress.

### 9.5 AKR-TMS hooks

- **Identifier model** with precision, source and quality for date of birth, place of birth, nationality, mother's chain, document numbers, family number, gender; the same model for list entries.
- **Normalisers** for dates (Gregorian only; default-date flags; day and month swap), Eastern digits in ID numbers and dates, document numbers by type.
- **Iraqi place gazetteer** (governorates, districts, cities; Arabic, Kurdish, Latin variants) and a nisba-to-place table.
- **Given-name gender lexicon** with unisex flags.
- **Qualifier weights as configuration** (documented, maker-checker), starting from the prototype values (year disagreement 3 or more years -0.20, 2 years -0.08, 1 year -0.02; mother's chain agreement +0.08, disagreement -0.20) and re-tuned on each bank's labelled alerts.
- **Capture plan**: ask the bank to supply, for payments, the structured ISO 20022 fields that will carry originator birth date and address once R.16 applies (end-2030 at the latest), and to keep mother's name and birth date structured in the CBS.

## 10. Algorithms and tools (string metrics, phonetic keys, ML vendors, OpenSanctions, open-source NLP, Rust crates)

All measurements below were run on 2026-10-06 with Python 3.14 (rapidfuzz 3.14.6, jellyfish 1.2.1, Metaphone package), Rust 1.97 and PostgreSQL 16 (Docker) on a 24-thread laptop-class CPU (AMD Ryzen AI 9 HX 370); scripts and outputs are in the scratchpad (`n22/`).

### 10.1 Edit distance, Jaro-Winkler and token methods

| Method | What it does | Strength | Where it fails on Iraqi and Arabic names (measured) |
|---|---|---|---|
| Levenshtein, Damerau | edits needed to turn one string into the other | simple, explainable, fast (bit-parallel versions) | Treats a missing hamza (احمد against أحمد, similarity 0.75) like a different letter; cannot tell a spelling variant from a different name |
| Jaro, **Jaro-Winkler** | matching characters and transpositions, bonus for a common prefix | designed for short personal names (Cohen et al.) | The prefix bonus rewards the shared prefixes of Arabic names: **Abdullah against Abdulrahman 0.90**, Abdulkarim against Abdulkadir 0.94, "mohammed ali" against "mohammed hussein" 0.89 (different people) |
| Token sort and **token set** ratios (RapidFuzz, FuzzyWuzzy style) | compare sorted or de-duplicated token sets | order-insensitive | Token set ratio is **100 whenever one name's tokens are a subset of the other's**: "mohammed ali" against "mohammed ali hussein ahmed al jubouri" scores 100 (the plain ratio is 49). This is the mechanism behind the false-positive flood of section 12.3 |
| q-gram Jaccard or Dice, TF-IDF cosine on n-grams | overlap of letter n-grams | good candidate generation, language independent | Spaces split trigrams: pg_trgm similarity of عبد الله against عبدالله is 0.545 and of محمد against محمود 0.375 |
| Monge-Elkan, **SoftTFIDF** | best token pairing with a secondary metric (SoftTFIDF adds IDF weights, Jaro-Winkler, threshold 0.9) | best general name matcher in Cohen et al. (2003) | Needs the chain-alignment and missing-token rules of section 13 on top |

Selected pair scores (Latin; RapidFuzz 3.14.6): Hussein against Husayn (same name) Levenshtein 0.57, Jaro-Winkler 0.82; Hassan against Hussein (different) 0.57 and 0.77; Qasim against Kassem (same) 0.50 and 0.70; **Jasim against Qasim (different) 0.80 and 0.87**; **Salih against Salah (different: صالح and صلاح) 0.80 and 0.91**; Ahmed against Mahmoud (different) 0.57 and 0.71. In Arabic script: صالح against صلاح Jaro-Winkler 0.92, محمد against محمود 0.95, حسن against حسين 0.93, احمد against أحمد (same name) only 0.83. Two different names routinely score higher than two spellings of the same name, so **no single threshold on a string metric separates variants from different names**.

### 10.2 Phonetic keys and why they fail on Arabic

- **They need Latin letters.** `jellyfish.soundex("محمد")` returns "م000" (first letter plus zeros: every name starting with the same Arabic letter gets one code) and `jellyfish.metaphone("محمد")` returns an empty string. PostgreSQL's `fuzzystrmatch.soundex('محمد')` returns an empty string.
- **On Latin transliterations they merge different names and split variants.** Soundex gives Hasan, Husayn, Hassan and Hussein the same code H250 (حسن and حسين are different names), Salih and Salah the same code S400 (صالح and صلاح), Mahmoud and Mohammed the same code M530; but Qasim, Gasim and Kassem get Q250, G250 and K250, and Kadhim and Kazim get K350 and K250. Double Metaphone gives Kazim the key KSM, the same as Qasim.

**Table 10.2a. Small illustrative test: 10 pairs that are the same name and 6 pairs that are different names** (Latin spellings of Iraqi names; the pairs are in the scratchpad):

| Method | Same-name pairs matched (of 10) | Different-name pairs matched (of 6) |
|---|---|---|
| Soundex equal | 6 | 4 |
| Metaphone equal | 8 | 3 |
| Double Metaphone (primary) equal | 8 | 3 |
| NYSIIS equal | 4 | 2 |
| Levenshtein similarity 0.80 or more | 5 | 2 |
| Jaro-Winkler 0.90 or more | 4 | 1 |
| Jaro-Winkler 0.85 or more | 7 | 2 |
| Beider-Morse (rphonetic 4.0.0, generic rules, approximate, auto language, sets intersect) | 4 | 0 |
| Own consonant key, equal | 4 | 3 |
| Own consonant key, similarity 0.80 or more | 10 | 4 |

Reading: **every method either misses variants or merges different names**; Beider-Morse separates the different-name pairs but missed 6 of 10 variants in this setting (a run restricted to the Arabic rule set returned empty codes and was not usable, so it was not evaluated; its rule files `gen_rules_arabic.txt` exist in Apache Commons Codec). Vowel-dropping keys, including ours, cannot separate names that differ only by a vowel (Hussein and Hassan, Salih and Salah), which Arabic script does separate (حسين and حسن, صالح and صلاح).

**Table 10.2b. Larger test: clustering Latin spellings by key** (Wikidata: 10,820 aligned Arabic and Latin token pairs of Iraqi persons, 2,434 Arabic name classes; two spellings count as the same name when their Arabic tokens are equal after normalisation; labels contain some alignment noise)

| Key | Precision | Recall | F1 | Distinct keys |
|---|---|---|---|---|
| exact lower-case ASCII | 0.958 | 0.534 | 0.686 | 3,507 |
| Soundex | 0.493 | 0.850 | 0.624 | 914 |
| Metaphone | 0.652 | 0.843 | 0.735 | 1,739 |
| Double Metaphone (primary) | 0.559 | 0.852 | 0.675 | 1,253 |
| NYSIIS | 0.630 | 0.804 | 0.706 | 1,887 |
| consonant skeleton (vowels, y, w dropped) | 0.498 | 0.849 | 0.628 | 1,456 |
| own digraph-aware key, strict | 0.571 | 0.821 | 0.673 | 1,851 |
| own key, loose (q, g, k merged; th, t; dh, d; w, y, ayn dropped) | 0.499 | 0.854 | 0.630 | 1,317 |

Phonetic keys reach recall of about 0.85 but precision of only 0.5 to 0.65: half of the pairs that share a key are different names. They are **useful as a blocking key and harmful as a decision rule**.

- **Arabic-specific phonetic keys** are reported in the literature and in some products (not verified here), but none is standardised and none of the Rust crates examined in 10.8 implements one; the consonant key of section 7.4 plays that role and is documented here so it can be audited. (Published Arabic Soundex variants were not evaluated: web search was exhausted during this research.)

### 10.3 N-grams, trigram indexes and PostgreSQL

- **pg_trgm silently returns nothing for Arabic in a database whose LC_CTYPE is C.** Tested on PostgreSQL 16 (Debian image): in a database created with `LC_CTYPE 'C'`, `show_trgm('محمد علي')` returns an **empty set** and `similarity('محمد','محمود')` is 0; in databases with `C.utf8` or `en_US.utf8` the same calls give nine trigrams and 0.375. The PostgreSQL documentation says only that "pg_trgm ignores non-word characters (non-alphanumerics)" and does not mention locale. The Alpine image's default database worked. **The installer must create the database with a UTF-8 ctype and a startup self-test must fail loudly when `show_trgm` of an Arabic test string is empty.**
- **Latency of a trigram index on a large watchlist** (1.5 million synthetic Arabic watchlist names drawn from the national list's token distribution, GIN trigram index 54 MB, built in 6.4 s; 400 queries, `name % q ORDER BY similarity LIMIT 50`): similarity threshold 0.3: p50 684 ms, p95 2,255 ms, p99 2,739 ms; threshold 0.5: p50 115, p95 1,089, p99 1,533 ms; threshold 0.7: p50 26, p95 170, p99 491 ms. **Too slow for an inline payment check**, because common tokens make the candidate sets huge; adequate for batch screening of small lists and for analyst search.
- **Spaces and compounds** change trigram sets (0.545 for عبد الله against عبدالله); normalise compounds before indexing.

### 10.4 Commercial and learned name matchers

Vendor statements, as published on their own pages (not tested; cross-reference file 14 for the screening vendors):

| Vendor product | Statements found (verbatim or close) | Not found |
|---|---|---|
| **NetOwl NameMatcher** (netowl.com, page read 2026-10-06) | Languages "Arabic, Chinese (traditional and simplified), Cyrillic, Greek, Hebrew, Japanese ..., Korean (Hangul), and Persian as well as languages written in the Latin alphabet"; "matches directly between the two scripts, avoiding the need for a Latin transliteration of the non-Latin script"; "Traditional approaches such as Soundex and Edit Distance require transliteration of foreign scripts to a Latin representation before matching is done"; variants handled: "transliteration variants, misspellings, nicknames, missing or extra name tokens, tokenization variations, name order variations, initials, orthographic variations"; scores from 0.0 to 1.0, "around 0.80 ... a good starting point"; "on Premise or in the Cloud", Docker and Kubernetes; "against hundreds of millions of names with sub-second response times"; "won the MITRE Multicultural Name Matching Challenge" | Independent accuracy figures, Iraqi test results |
| **Babel Street Match** (formerly Rosette name matching; babelstreet.com/modules) | "AI-powered matching that accurately resolves names, addresses, and dates across languages — reducing false positives and enabling fast, explainable decisions"; "NLP that understands 40+ languages and scripts"; "Deploy on premises, in the cloud, or integrate via APIs and SDKs" | Arabic is not named on the page read; the technical documentation (docs.babelstreet.com) needs JavaScript and could not be read |
| **IBM InfoSphere Global Name Recognition** | Not found in the IBM Information Server 11.7 documentation index reachable on 2026-10-06 | Everything; treat as unverified |

Consequences: (1) vendor claims of "sub-second" and "direct cross-script matching" are plausible but must be tested on **Iraqi data with the bank's own labelled pairs** (section 12); (2) learned matchers are opaque, which collides with the Wolfsberg and CBI expectation of documented, testable configuration (file 04 section 2d) unless the vendor provides score explanations; (3) a bank that already licenses one of them can still use AKR-TMS's pipeline for candidate generation, secondary identifiers and the pair memory, and call the vendor's scorer as one feature.

### 10.5 OpenSanctions: yente, nomenklatura and rigour

(Source code read from GitHub on 2026-10-06; all three are MIT-licensed.)

- **yente** (FastAPI, Python; pushed 2026-10-05): "an open source data match-making API"; needs **Elasticsearch or OpenSearch** (`YENTE_INDEX_URL`), numpy, scikit-learn and PyICU; "can also be run on-premises as a KYC appliance so that no customer data leaves your infrastructure". Not part of the AKR-TMS stack (Rust, PostgreSQL, Valkey), and the OpenSanctions **data is CC BY-NC 4.0**: commercial use needs a "Screening License", a "Reseller/OEM License" or the hosted API (opensanctions.org/licensing; prices not published on that page). Cross-reference file 15.
- **nomenklatura** (pushed 2026-10-01) ships seven scoring algorithms: `logic-v1`, `logic-v2`, `name-based`, `name-qualified`, `ofac`, `regression-v1` (the default) and `er-unstable` (entity-resolution regression). `logic-v2` is described as "a rule-based matching system that generates a set of basic scores via name and identifier-based matching, and then qualifies that score using supporting or contradicting features ... a versatile matching algorithm that uses cultural reference data for precise and explainable cross-language and cross-script matching". The shared comparison functions (used by `logic-v1`, `regression-v1` and parts of `logic-v2`) use Jaro-Winkler on ASCII-folded name parts with a Levenshtein plausibility gate, and Metaphone and Soundex of transliterated tokens; a further rule treats an exact match against a **weak alias** as evidence (the code comment cites OFAC's FAQ topic 1646; `logic-v2` gives it weight 0.8).
- **rigour** (pushed 2026-10-01; the "Rust core" `rigour-core` 2.5.0 with `rphonetic`, `icu` 2 with unstable transliteration, `rapidfuzz`, `aho-corasick`, `regex`, `lru`, `zstd`): name analysis with a **part-tag vocabulary** UNSET, AMBIGUOUS, TITLE, GIVEN, MIDDLE, FAMILY, **TRIBAL, PATRONYMIC, MATRONYMIC**, HONORIFIC, SUFFIX, NICK, STOP, NUM, LEGAL, and **symbol categories** ORG_CLASS, SYMBOL, DOMAIN, INITIAL, NAME, NICK, NUMERIC, LOCATION, PHONETIC; `align_person_name_order`, `pair_symbols` and a cost-folded dynamic-programming `compare_parts` with defaults such as separator-drop cost 0.2 and confusable cost 0.7. Its own notes say that "broad-script transliteration (Han / Arabic / Devanagari / ...) is deliberately **not** in Rust" and the fast transliteration path admits only Latin, Cyrillic, Greek, Armenian, Georgian and Hangul.
- **OFAC emulation** in `name_based/ofac.py`: "reverse-engineered from the public tool ... plus FAQ 249" (section 11.3).

What to take from it: the **vocabulary of name parts and the explainable alignment** (parts, symbols, extras weights, family-name boost, qualifiers) are the right shape for AKR-TMS and are MIT-licensed, but the Arabic and Iraqi cultural reference data (tribes, Abd compounds, kunyas, Kurdish folds) and the Arabic-Latin bridge are exactly what is missing, so AKR-TMS must build them.

### 10.6 Open-source Arabic NLP

- **CAMeL Tools** (NYU Abu Dhabi CAMeL Lab; MIT; Obeid et al., LREC 2020, pp. 7022-7032): normalisation utilities (`normalize_unicode` = NFKC after mapping U+FDFC and U+FDFD; `normalize_alef_ar` maps U+0625, U+0623, U+0671, U+0622 to ا; `normalize_alef_maksura_ar` ى to ي; `normalize_teh_marbuta_ar` ة to ه), `dediac`, `arclean` (folds Persian, Urdu and Sindhi letters, for example پ to ب), Buckwalter, Safe Buckwalter, XML Buckwalter and Habash-Soudi-Buckwalter transliteration maps, morphological analysis, dialect identification, NER. **Installation needs Python 3.11 to 3.14, a Rust compiler, CMake and Boost on Linux, and a separate `camel_data` download**; it is a research toolkit. The normalisation functions are a few lines each: re-implement them in Rust rather than depend on Python at run time; use CAMeL Tools in the test pipeline as a cross-check.
- **PyArabic** (Taha Zerrouki; `pip install pyarabic`, 0.6.15 installed for tests): `strip_tashkeel`, `strip_harakat`, `strip_tatweel`, `normalize_hamza`, `normalize_ligature`, `tokenize`. **Licence GPLv3**: do not link it into a product distributed to banks; use only as a reference.
- Morphological analysers are not needed for names (names are not inflected) except for splitting glued prefixes; the lexicon approach of section 6.3 is simpler and auditable.

### 10.7 ICU and CLDR transliterators

- The CLDR transform `Arabic-Latin` states in its header: "Generally follows UNGEGN ... Occasionally deviates in the direction of ISO 233 ... Does *not* do assimilation of 'al', nor hyphenation. While it could be done, we need to determine whether a prefix 'al' could occur other than as the definite article (since no space is used)." That last sentence is the glued-article problem of section 6.3 in the standard's own words. CLDR also ships `Arabic-Latin-BGN` and `Persian-Latin-BGN`.
- **ICU4X** (`icu` 2.3.1, `icu_experimental` 0.6.0, "pre-release components under active development; all code in this crate is unstable"): the Rust `Transliterator` works. Output for Arabic names (tested): محمد to "mḥmd", أحمد to "ạ̉ḥmd", عبد الرحمن to "ʿbd ạlrḥmn", حسين to "ḥsyn", قاسم to "qạsm", كاظم to "kạẓm", الجبوري to "ạljbwry", فاطمة to "fạṭmẗ", مصطفى to "mṣṭfy̱", عبدالله to "ʿbdạllh": **unvocalised, with diacritics and the article glued**. Kurdish letters are left untransliterated (گۆران gives "gۆrạn", پەرێز gives "pەrێz"), and the `Arabic-Latin/BGN` variant is not in the compiled data (error `IdentifierNotFound`). Useful for display and debugging, not as a matching key without post-processing.
- `any_ascii` 0.3.3 and `deunicode` 1.6.2 (Rust) give vowel-less ASCII skeletons ("mhmd", "'hmd" for أحمد, "gorn" for گۆران), with inconsistent letter case in deunicode ("fTm@" for فاطمة): fine for logs, not for matching.

### 10.8 Rust crates for the AKR-TMS stack (crates.io metadata read 2026-10-06)

| Crate | Version (last release) | Licence | Role in AKR-TMS | Caveat |
|---|---|---|---|---|
| `unicode-normalization` | 0.1.25 (2025-10-30) | MIT or Apache-2.0 | NFC, NFD, NFKC, NFKD for N2 and N4 | none; pin the Unicode version |
| `icu_normalizer`, `icu` (ICU4X) | 2.3.0 and 2.3.1 (Aug 2026) | Unicode-3.0 | normalisation with compiled data, properties, collation; alternative to the above | transliteration is in `icu_experimental` 0.6.0 (unstable) |
| `unicode-segmentation` | 1.13.3 (2026-06-01) | MIT or Apache-2.0 | grapheme and word boundaries for display and truncation | do not tokenise names with it (6.3) |
| `strsim` | 0.11.1 (2024-04-02) | MIT | Levenshtein, Damerau, Jaro, Jaro-Winkler, Sorensen-Dice on `&str` | allocation per call; fine for scoring, not for bulk |
| `rapidfuzz` (rapidfuzz-rs) | 0.5.0 (2023-12-01) | MIT | bit-parallel Levenshtein, Jaro-Winkler; used by rigour-core | last release 2023; check maintenance |
| `triple_accel` | 0.4.0 (2021-06-18) | MIT | SIMD edit distance | last release 2021 |
| `rphonetic` | 4.0.0 (2026-09-03) | Apache-2.0 | port of Apache Commons Codec: Soundex, Metaphone, Double Metaphone, NYSIIS, Daitch-Mokotoff, **Beider-Morse** | Latin letters only; rule files needed for Beider-Morse |
| `aho-corasick` | 1.1.5 (2026-08-03) | Unlicense or MIT | lexicon and particle tagging (tribes, titles, Abd compounds) in one pass | none |
| `fst` | 0.4.7 (2021-06-06) | Unlicense or MIT | compact vocabulary sets and Levenshtein automata for fuzzy token lookup | last release 2021 |
| `symspell` | 0.5.2 (2026-03-22) | MIT | deletion-neighbourhood fuzzy lookup (our prototype implements the same idea) | small crate, 185k downloads |
| `tantivy` | 0.26.2 (2026-09-08) | MIT | full search engine library if an index with n-gram tokenisers is preferred | heavier than a custom token index |
| `ort` | 2.0.0-rc.13 (2026-07-28) | MIT or Apache-2.0 | ONNX Runtime 1.28 binding to run a trained matcher model in Rust | release candidate |
| `any_ascii`, `deunicode` | 0.3.3, 1.6.2 | ISC, BSD-3-Clause | diagnostic ASCII skeletons | lossy, see 10.7 |
| `whatlang` | 0.18.0 (2025-10-16) | MIT | script and language hints for free text | not needed for names |

Everything the pipeline needs is available in pure Rust; the gaps are **data** (lexicons, fold tables, costs) and **tests**, not libraries.

### 10.9 AKR-TMS hooks

- **Own the Arabic layer**: normalisation (6.4), Latin normalisation (7.2), consonant key (7.4), lexicons and chain alignment (13.5) as one Rust crate with a golden test file; use `unicode-normalization`, `aho-corasick`, a deletion-neighbourhood index and a bit-parallel edit distance.
- **Use phonetic and n-gram keys only for candidate generation**; scoring is aligned token comparison plus secondary identifiers.
- **Treat OpenSanctions, CAMeL Tools and vendor matchers as references and cross-checks**, wired into the test harness (agreement reports), not as run-time dependencies; record licences (CC BY-NC data, GPLv3 PyArabic) in the third-party register.
- **PostgreSQL**: use `pg_trgm` for analyst search and small lists only, with the ctype self-test of 10.3; the screening service holds the watchlist index in memory (sections 13.4 and 13.9).
- **Vendor matcher plug-in**: a trait `NameScorer` so a bank's licensed matcher can be called for the final score while AKR-TMS keeps blocking, identifiers, thresholds and the audit trail.

## 11. What regulators and Wolfsberg say; how OFAC scores names

No regulator prescribes a name-matching algorithm. What they prescribe is a **documented, tested, explainable and periodically re-validated** configuration. The statements below set the bar that the pipeline of section 13 has to clear. Items marked (file 04), (file 05), (file 03) and (file 01) are read in those files.

### 11.1 Iraq (CBI) and FATF

- CBI AML/CFT/CPF supervisory controls, 8 January 2025 (file 01 section 2d): the screening platform must "support search in Arabic and English", update local and international lists "every 12 hours at least", cover all parties dealt with directly or indirectly and beneficial owners, stop transactions that match scenarios "until the MLRO verifies", and screen "through the World-Check system" (named explicitly).
- FATF R.16 footnote 49: R.16 "does not say how to screen payment data against sanctions lists; any effective mechanism is acceptable" (file 03). Effectiveness is therefore judged by outcomes (misses and false positives), which is why section 12 matters.

### 11.2 Wolfsberg (file 04)

- **Sanctions screening guidance (2019, file 04 section 2d)**: screening is "the comparison of one string of text against another"; hard parts are "alphabets, languages, cultures, spelling, abbreviations, acronyms, aliases, fuzzy algorithms"; the risk assessment decides "which attributes, when, which lists and how fuzzy"; **exact against fuzzy is set per list**; configuration must be documented and "transparent about limitations"; testing must be backed by metrics; weak aliases (short strings, digits, nickname words such as an "Ahmed the Tall" style name, geographic references, a common prefix plus one name) should not be screened on their own; suppression ("good guys") lists need governance; after a miss, **lookback** analysis.
- **PEP guidance (2017, file 04 section 2n)**: "close-match (not exact) screening with tolerance for transpositions, variants and typos; auto-discounting only on reliable clear differences (e.g. birth year)"; delta screening both ways; or a no-match list keyed on the customer and PEP pair, invalidated when either record changes materially.
- **Auditing for effectiveness (2024) and the Monitoring statements (file 04 sections 2m, 2a, 2b)**: effectiveness over volume; outcome-based testing; governance of tuning.

### 11.3 OFAC and its Sanctions List Search tool

OFAC's own FAQs (ofac.treasury.gov, read 2026-10-06):

| FAQ | Date | What it says |
|---|---|---|
| 246 "How does Sanctions List Search work?" | 28 Apr 2021 | exact matches at a 100% threshold; broader results "using fuzzy logic ... character and string matching as well as phonetic matching"; **only the name field uses fuzzy logic**, other fields use character matching |
| 247 "What does the Sanctions List Search Score mean?" | 28 Apr 2021 | the score indicates similarity using "two matching logic algorithms: one based upon phonetics, and a second based upon the similarity of the characters in the two strings"; the slider defaults to 100 (exact) |
| 248 "How do I use the Minimum Name Score field and score slider bar?" | 5 Feb 2013 | 100 returns "only names that exactly match the characters entered"; 50 returns "all names that are deemed to be 50% similar"; lowering the threshold returns a broader result set |
| **249** (score calculation) | 28 Apr 2021 | the tool "will first look for potential matches based on the first letter of input search terms and by checking for matches at least 50% or more similar based on edit distance"; it then uses **Jaro-Winkler** (string difference) and **Soundex** (phonetic), each through two techniques: comparing the **entire name** against full list names, and **splitting names into parts** and comparing each part; it "calculates a score for each name part entered, and a composite score" and "returns the higher of the two scores" |
| 122 "What are weak aliases (AKAs)?" | 18 Jan 2011 | a weak AKA is "a term for a relatively broad or generic alias that may generate a large volume of false hits when such names are run through a computer-based screening system"; judged by five criteria: character length, presence of numbers, common nickname words, geographic location references, very common prefixes in two-string names |
| 124 "Am I required to screen for weak aliases?" | 18 Jan 2011 | "OFAC does not expect that persons will screen for weak AKAs, but expects that such AKAs may be used to help determine whether a 'hit' arising from other information is accurate" |
| 125 | 18 Jan 2011 | OFAC will typically not issue civil penalties if the sole sanctions reference is a weak AKA, the processor had no reason to know, and a rigorous risk-based compliance program existed |
| 5 "How do I determine if I have a valid match?" | 30 Jan 2015, updated 9 Sep 2026 | "compare all of the details in the OFAC listing with the information available to you about the data in your transaction or activity that led to the 'alert' or 'hit'"; many alerts are false positives; make risk-based determinations |

The FAQs do not say whether the tool transliterates or searches non-Latin scripts. **How the score really behaves** has been reverse-engineered by OpenSanctions (nomenklatura `name_based/ofac.py`, MIT licence, "an emulation rather than an exact reimplementation - expect small deviations", tuned on 164 positive examples):

1. **Whole-string Jaro-Winkler gated on the first letter**: a two-word test name typed as "SURNAME GIVEN" returns no hit while "GIVEN SURNAME" returns three, because the whole-string technique requires the first characters to be equal (the emulator's test cases use well-known persons; the names are omitted here).
2. **Jaro-Winkler without the 0.7 boost threshold** (the SimMetrics variant): a two-word personal name against a long entity name that begins with a similar word scores 82 in OFAC and 64 with standard Jaro-Winkler.
3. **Per-token best pairing with a floor**: each query token takes its best Jaro-Winkler match among the candidate's tokens; a pair below 0.5 contributes 0; the mean over kept query tokens is the score; **query tokens of two characters or fewer are dropped** (a three-word name whose last word has two letters loses it and then matches five different listed individuals at 100).
4. The final score is the maximum of the two techniques, and an entity is scored by its best alias.

The emulation's own docstring is blunt: "This is not a 'good' name matcher in the academic sense - it inherits OFAC's quirks (single-token explosions, token-order asymmetry, the per-token-pair scoring cliff at JW 0.5)." Two consequences for AKR-TMS: **do not copy the OFAC tool as the screening algorithm** (our experiment in section 12.3 reproduces its flooding on Iraqi chains: the per-name-part approach cannot get below a 9% to 22% alert rate on ordinary names even at a threshold of 1.00); but **correspondent banks and examiners can use the public tool to test a bank** (an inference from its public availability, not verified), so the regression suite must include an *OFAC-emulation oracle* ("would the public tool have returned this name at 85 or more?") for every name the bank's engine decided not to alert on.

### 11.4 United States and European supervisory expectations (file 05)

- **FFIEC BSA/AML Examination Manual**: banks must "understand their filtering criteria"; "management should periodically review and test the filtering criteria and thresholds"; the system's "programming methodology and effectiveness should be independently validated"; examiners check for "reasonable filtering criteria whose programming has been independently verified".
- **NYDFS Part 504**: the filtering program needs "end-to-end, pre- and post-implementation testing, including data matching, whether the OFAC list and threshold settings map to the institution's risks, the logic of the matching technology, model validation, and data input and output", ongoing analysis of the logic and performance of the name matching technology, and documentation of the intent and design of the filtering tools.
- **EBA/GL/2024/15** (applies from 30 December 2025, to payment service providers including European banks and therefore to the correspondents of Iraqi banks): the institution must "understand and document the screening system's capabilities and limitations"; **customer names must be screened "in the original and/or transliteration"**; matching parameters are set "by checking the thresholds of true positive results associated with different percentages of matching", neither too sensitive nor too lax, with **fuzzy matching** (spelling, pattern or sound) and documented rationale; alerts "must show which element of the measure was matched"; secondary identifiers resolve alerts; unresolved alerts block the service; good-guy lists need documented reasons and review when data changes; performance review at least yearly.

### 11.5 What the regulators agree on

1. Fuzziness is a **configured, per-list, documented** property, not a hidden default.
2. Thresholds are **calibrated on measured true-positive rates** and re-validated; below-the-line testing (what the system did not alert on) is expected.
3. **Independent validation** of the matching logic, with a record of data lineage and test cases.
4. Alerts must be **explainable**: which names, which parts and which identifiers matched.
5. **Secondary identifiers** resolve alerts; weak aliases are not screened alone.
6. Names are screened **in the original script and in transliteration**, which is the whole point of this file.

### 11.6 AKR-TMS hooks

- **Screening configuration object** per list (exact or fuzzy, thresholds per tier, weak-alias policy, transliteration set, lookback policy), versioned with maker-checker, exportable as the "documented configuration" (file 04 section 2d, file 29).
- **Validation pack generator**: for each release, the golden test set, the synthetic corpus results (section 12), the recall and alert-rate curves, the OFAC-emulation agreement report and the data lineage from the CBS replica.
- **Below-the-line sampling**: weekly sample of names scoring just under each threshold, reviewed by compliance, feeding threshold review tasks.
- **Explanation record per alert** (section 13.7) with the matched parts, symbols, folds applied, identifiers and score components.
- **OFAC-emulation oracle** and **list-lag report** in the CBDDQ evidence pack (file 04 section 2c).

## 12. Evaluation: test sets, precision and recall, building a synthetic Iraqi corpus

### 12.1 What to measure

| Measure | Definition | Why it matters |
|---|---|---|
| **Alert rate** and **alerts per 1,000 screened** | share of ordinary names (customers or payment parties) that produce at least one alert, and the mean number of alerts | this is the operating cost; it is set by the bank's analyst capacity |
| **Recall at a fixed alert budget** | share of true matches (noisy variants of list entries) that still alert when the threshold is set so that the alert rate on ordinary names equals the budget | the fair way to compare algorithms: precision and recall at one arbitrary threshold hide the trade-off |
| **Recall by variant class** | recall separately for clean, orthographic noise, truncated chain, reordered, typo, combined | shows *where* misses happen (a single average hides truncated names) |
| **Precision of alerts** | true matches among alerts, on a corpus with injected true matches | needs labelled data; estimate from the bank's own closed alerts |
| **Below-the-line miss rate** | true matches found only by lowering the threshold | what examiners ask for (section 11) |
| **Time to alert** | latency of the scoring path | phase-2 budget (section 13.9) |

**Cost of false positives against misses.** A false positive costs analyst time (illustratively 10 to 20 minutes per alert at a bank with a manual four-eyes process); a miss can cost a penalty, the loss of a correspondent (file 01 section 4) or an asset-freeze failure. Because the cost of a miss is not bounded and not comparable, the engineering rule is: **fix a recall floor by policy on each variant class, then minimise the alert rate subject to that floor**, and show leadership the alert volume that results. Illustrative arithmetic: at a 1% alert rate, screening a 1 million customer base for the first time yields 10,000 alerts, at 5% it yields 50,000, which no bank can review in a few weeks; a bank onboarding 300 customers a day produces 3 alerts a day at 1%; 5,000 cross-border payments a day with two parties each and a 0.5% alert rate produce 50 alerts a day. The first full screening (migration) therefore needs staged thresholds and secondary identifiers, not a single setting.

### 12.2 Data sets that exist, and what was used here

| Data set | What it offers | Licence and status | Used here |
|---|---|---|---|
| **UN consolidated list**: Arabic original and Latin names and aliases | 328 individuals with an Arabic-script name (all 736 records carry 2,894 Latin names and aliases) | UN, public | Yes (cross-script retrieval, token statistics) |
| **Iraqi national list** (AML/CFT Office export) | 6,253 Arabic names with mother's name and birth year | public official list; do not republish names | Yes, aggregates only (distribution, identifiers, candidate pool) |
| **Wikidata** Iraqi persons with Arabic and English labels and birth dates | 5,042 label pairs (3,022 born 1930-2007 with a birth date) | CC0 | Yes (Latin variants, ordinary-name queries) |
| **OpenSanctions judged pairs** (nomenklatura `JudgedPair`, JSON lines with `left`, `right`, `judgement` positive or negative) | training and benchmark pairs; `bench_matcher` times an algorithm over a pairs file | data CC BY-NC; format and code MIT | Format only; the Arabic coverage was not assessed |
| **MITRE Multicultural Name Matching Challenge** (cited by NetOwl) | multicultural name pairs | availability and licence not verified | No |
| **NEWS (Named Entities WorkShop) transliteration shared tasks** | Arabic-English name transliteration pairs | not verified in this research | No |
| **GeoNames** | place names in Arabic, Kurdish and Latin | CC BY | Not yet (hook in 9.5) |
| **The bank's own data** | its customers, payments and closed alerts | confidential; stays in the bank | Planned (below) |

### 12.3 Experiment E1: how many ordinary Iraqi names does each approach alert on?

**Setup.** Candidate pool: 5,698 distinct persons of the Iraqi national list (core chain, alias segments removed, normalised). Queries: **E1a**, 5,000 synthetic customers drawn from the list's own marginal name distributions (given name, father, grandfather, tribe; 20% two tokens, 40% three, 25% four, 15% five; orthographic noise; birth year uniform 1950-2005; mother's chain for 75%): a deliberately **worst-case** population because every token is in the list's vocabulary; **E1b**, 3,007 notable Iraqis born 1930-2007 from Wikidata with two or more tokens in the Arabic label (mean 2.46 tokens): a **best-case** population with a broad vocabulary. For each query the highest score against any candidate is recorded. Algorithms (Python prototype, scoring on Arabic script):

- **A0**: the OFAC-style method on raw tokens (best Jaro-Winkler per query token with a 0.5 floor, mean over query tokens, ignoring tokens of two letters or fewer, maximum with a first-letter-gated whole-string Jaro-Winkler); **A**: the same after our normalisation (section 6.4); **B**: token set ratio after normalisation.
- **C**: chain-aware alignment: one-to-one token alignment with edit-distance gating (no edits up to 3 letters, one edit up to 7, two edits above), IDF weights, unmatched query tokens weighted 0.8 and unmatched list tokens 0.2, at least two aligned tokens (or one rare token).
- **D**: C plus secondary identifiers: birth year (3 or more years apart -0.20, 2 years -0.08, 1 year -0.02) and mother's chain (agreement +0.08, disagreement -0.20); missing identifiers are neutral. (In E1b only the birth year exists.)

**Table 12.3a. Share of ordinary names with at least one alert, by threshold (mean number of alerts per query in brackets)**

| Algorithm | >= 0.85 | >= 0.90 | >= 0.95 | = 1.00 |
|---|---|---|---|---|
| **E1a worst case** | | | | |
| A0 OFAC-style, raw tokens | 88.3% (145) | 55.1% (55) | 24.7% (21) | **10.9%** (8.6) |
| A OFAC-style, normalised | 92.1% (159) | 61.4% (67) | 35.2% (32) | **22.5%** (15.5) |
| B token set ratio | 40.1% (8.5) | 27.7% (6.7) | 26.1% (6.6) | **25.9%** (6.6) |
| C chain-aware | 13.9% (0.27) | 5.6% (0.07) | 1.2% (0.01) | 0.2% (0.00) |
| D chain-aware plus identifiers | 0.5% (0.01) | 0.2% | 0.0% | 0.0% |
| **E1b best case** | | | | |
| A0 | 74.7% (64) | 47.9% (16) | 18.9% (3.6) | **9.2%** (1.6) |
| A | 75.6% (49) | 44.5% (15) | 21.6% (5.1) | **11.7%** (2.2) |
| B | 14.2% (1.7) | 11.9% (1.6) | 11.6% (1.6) | **11.6%** (1.6) |
| C | 5.0% (0.07) | 1.3% (0.02) | 0.2% | 0.0% |
| D (birth year only) | 0.5% | 0.1% | 0.0% | 0.0% |

**Findings.** (1) **The OFAC-style per-name-part method cannot reach an alert rate below about 9% to 22% on ordinary Iraqi names at any threshold, because a two- or three-token customer name is fully contained in many five-token list chains and scores 100.** Normalising first made it worse (more tokens agree). (2) A token-set method has the same floor (11.6% to 25.9%). (3) Chain-aware alignment brings the alert rate under control at high thresholds but still alerts 14% of worst-case names at 0.85, because short names match long chains partially. (4) **Adding birth year and mother's name removes almost all of the remaining alerts** (0.5% at 0.85). These are the numbers behind the statement "a name alone does not identify a person in Iraq" (section 8.2).

Limits: the OFAC-style algorithm here is an emulation of the per-name-part technique on Arabic script (no Soundex branch, no first-letter prefilter), and the synthetic population is an extreme case; the *ranking* of methods is the finding, not the absolute rates.

### 12.4 Experiment E2: recall on noisy variants at equal alert budgets

**Setup.** 1,500 random list persons; for each, eight customer-record variants: N0 full chain (clean), N1 full chain with orthographic noise (hamza flips, final ya and ha swaps, glued or split Abd compounds, tatweel), N2 first three tokens, N3 first two tokens, N4 family name first plus given name and father, N5 full chain with one typo, N6 first three tokens with orthographic noise and a typo, N7 first four tokens with orthographic noise. Secondary identifiers: birth year present 90% (exact 90%, off by one 6%, wrong by 3 or more 4%); mother's name present 70% (two tokens only in 40% of those; 10% with a typo). The threshold of each algorithm is the lowest value whose alert rate on E1a (or E1b) stays within the budget.

**Table 12.4a. Recall by variant class, thresholds set so that the alert rate on ordinary names (E1a, worst case) is within the budget**

| Budget | Algorithm | Threshold | N0 | N1 | N2 (3 tokens) | N3 (2 tokens) | N4 | N5 typo | N6 | N7 | Mean |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 5% | A0, A, B | not reachable (floor 10.9%, 22.5%, 25.9%) | | | | | | | | | |
| 5% | C | 0.905 | 1.00 | 1.00 | 0.38 | 0.05 | 0.50 | 0.54 | 0.08 | 0.91 | 0.56 |
| 5% | **D** | 0.690 | 1.00 | 1.00 | 0.97 | 0.86 | 0.98 | 0.81 | 0.51 | 0.99 | **0.89** |
| 2% | C | 0.940 | 1.00 | 1.00 | 0.17 | 0.01 | 0.24 | 0.35 | 0.03 | 0.70 | 0.44 |
| 2% | **D** | 0.760 | 1.00 | 1.00 | 0.94 | 0.73 | 0.96 | 0.68 | 0.47 | 0.98 | **0.84** |
| 1% | C | 0.955 | 1.00 | 1.00 | 0.11 | 0.01 | 0.14 | 0.24 | 0.02 | 0.53 | 0.38 |
| 1% | **D** | 0.810 | 0.97 | 0.98 | 0.90 | 0.57 | 0.93 | 0.61 | 0.42 | 0.97 | **0.79** |

(Using the E1b population, the best case, gives the same picture at lower thresholds: at a 1% budget C reaches mean recall 0.54 and D 0.79; at 0.5% C 0.47 and D 0.75.)

**Table 12.4b. Ablation at a 2% alert budget (E1a 2,500 queries; E2 1,000 persons)**

| Variant | Mean recall | N2 | N3 | Notes |
|---|---|---|---|---|
| C0: no IDF, symmetric extras (0.5, 0.5) | 0.45 | 0.03 | 0.00 | |
| C1: IDF, symmetric extras (1.0, 1.0) | 0.43 | 0.12 | 0.01 | |
| **C2: IDF, asymmetric extras (0.8, 0.2)** | 0.44 | 0.16 | 0.02 | the C of 12.3 |
| C3: C2 without the two-token rule | 0.44 | 0.16 | 0.02 | rule does not bind on this data |
| C4: C2 with exact tokens only (no fuzzy) | 0.43 | 0.20 | 0.03 | typos (N5) fall from 0.34 to 0.17 |
| D1: C2 plus birth year only | 0.69 | 0.75 | 0.24 | |
| D2: C2 plus mother's name only | 0.71 | 0.79 | 0.30 | |
| **D3: C2 plus birth year and mother's name** | **0.84** | **0.94** | **0.71** | the D of 12.3 |
| D0: D3 without IDF | 0.85 | 0.94 | 0.67 | IDF adds nothing measurable here |

**Findings.** (1) At the same alert budget, the combination of aligned chain matching and secondary identifiers keeps **0.79 to 0.89 mean recall at a 1% to 5% alert rate**, while name-only matching keeps **0.38 to 0.56**, and the OFAC-style methods cannot reach those budgets at all. (2) The classes where names alone fail are exactly the ones that matter in practice: **truncated chains (N2, N3)**, where recall rises from 0.01 to 0.17 to 0.73 to 0.94 once identifiers are present. (3) **Token weighting (IDF) and the asymmetric treatment of extra tokens made no measurable difference** to recall at a fixed budget in this setting; the value of the lever is **secondary identifiers**, then **fuzzy token matching** (typo recall doubles). (4) Variants with a typo plus truncation plus noise (N6) stay hard (0.42 to 0.51 at best): these need a human-reviewed below-the-line process. These results come from a Python prototype with a synthetic noise model; thresholds are not transferable, relative effects are.

### 12.5 Cross-script and key-based evaluation (summary of 7.3 and 10.2)

Arabic to Latin and Latin to Arabic retrieval with the consonant key (98.5% and 99.5% top-1 with full names, 31% to 33% with two tokens); phonetic keys have recall about 0.85 and precision 0.5 to 0.65; no phonetic key or string threshold separates variants from different names (Tables 10.2a and 10.2b). Together with 12.3 and 12.4 these define the **baseline the production pipeline must beat** on the same corpus.

### 12.6 Building the synthetic Iraqi name corpus

**Purpose**: a versioned, shareable, privacy-free corpus that every release of the matcher is tested against, and that a bank can extend with its own pairs. Real customer names never leave the bank; the corpus is generated from public distributions and published lists.

1. **Chain generator** from measured parameters (sections 8.2 and 6.1): number of tokens (core chain mean 5.05; customers 2 to 5), token frequencies for given name, father, grandfather, great-grandfather and tribe from the national list and the bank's own file (frequency tables, not names), the article on tribe tokens (77% of last tokens), Abd compounds (26% of names), mother's chain (2.87 tokens), birth years (national-list deciles for the listed side, an adult distribution for customers), place of birth from a governorate list.
2. **Variant generators** with documented rates: orthographic (hamza, ya and maksura, ta marbuta and ha, tatweel, diacritics, presentation forms, ZWNJ, Persian and Kurdish letters, Eastern digits), compound spacing (عبد الله or عبدالله), article attached or separate, kunya segments (المكنى أبو X, parentheses), truncation (4, 3, 2 tokens), family-first order with or without a comma, typos (deletion, substitution with keyboard neighbours, transposition), transliteration to Latin by several Iraqi styles (the variant table of 5.3, Table 8.1e) with truncation to 35 characters and uppercase, Arabizi, Kurdish and Turkmen spellings (section 4).
3. **Hard negatives**: pairs of different names that are close in Latin or in Arabic (Hasan and Husayn, Salih and Salah, Jasim and Qasim, Ahmad and Mahmoud, Abdullah and Abdulrahman, Ali and Aly, the same chain with a different mother's name or different birth year), plus relatives (same father and grandfather, different given name).
4. **Positive injection**: insert N synthetic watchlist persons and noisy variants of them into the synthetic customer base, with ground-truth links; also inject variants of real (published) list entries *by token pattern only* when the bank approves.
5. **Labels and splits**: every pair carries a label, variant class, script pair, identifier availability and a seed; split into tuning and held-out sets by *list entry*, not by pair, so tuning cannot memorise entries.
6. **Reporting**: recall by variant class and script pair at fixed alert budgets (12.4), alert rate on negatives (12.3), latency, and a diff against the previous release; acceptance targets proposed here for discussion with CBI-supervised banks: recall at or above 99.5% on N0 and N1, at or above 95% on N2, N4, N5 and N7 *when secondary identifiers are present*, and an alert rate on ordinary names that the bank's compliance head signs off.
7. **Format**: JSON lines compatible with nomenklatura's `JudgedPair` (`left`, `right`, `judgement`, `weight`) so the same corpus can be run through logic-v2 and the OFAC emulation as independent cross-checks.
8. **Governance**: the corpus, generator, seeds and results are versioned and kept as part of the model validation pack (file 29); the generator parameters are re-estimated from each bank's data quality profile (section 6.6).

### 12.7 AKR-TMS hooks

- **Evaluation harness** as a first-class product component: corpus loader, algorithm plug-ins (own matcher, OFAC emulation, nomenklatura logic-v2, a vendor scorer), recall and alert-rate curves, release comparison, below-the-line sampler.
- **Shadow mode**: run a new configuration in parallel on live data for two to four weeks, compare alerts, and promote only with a signed change record.
- **Closed-alert learning** (pair memory, section 13.8): every analyst decision becomes a labelled pair feeding threshold review, the bank-specific variant table (5.4) and the hard-negative set.
- **Lookback** after any miss: re-run the corrected configuration over the history held in the CBS replica and report (Wolfsberg, file 04 section 2d).
- **Corpus bundle shipped with the product** with the pipeline's expected results for each release, so a bank's validator can reproduce them on site.

## 13. Matching pipeline design for AKR-TMS

### 13.1 Design principles

1. **Arabic is the system of record**; Latin names are matched through generated forms and the consonant key, never Latin to Latin alone (sections 5 to 7).
2. **Evidence, not a bare score**: a name match contributes evidence in bits; secondary identifiers add or subtract; an alert needs a similarity above a per-list threshold *and* enough evidence for the bank's customer count (sections 8.4 and 12).
3. **Missing is neutral, contradiction is negative, a default value is not evidence** (section 9.4).
4. **Deterministic and explainable**: same input, same list version, same configuration gives the same score, and the alert carries the aligned parts, the folds applied and the identifiers compared.
5. **Configuration is governed data**: thresholds, weights, fold penalties, lexicons, weak-alias policy per list, versioned with maker-checker (section 11.6).
6. **Two latency modes, one algorithm**: an *inline* mode for phase-2 payment checks and a *batch* mode for customer screening, differing only in blocking limits and fuzzy gates (13.9).
7. **Humans decide; the system remembers**: every decision becomes a pair record that prevents re-alerting only while nothing has changed (13.8).

### 13.2 Pipeline overview

```
ingest (CBS replica, payments, list feeds)
  -> N1-N11 Arabic normalisation / L1-L10 Latin normalisation        [6.4, 7.2]
  -> parse into typed parts: given, father, grandfather, tribe, nisba, laqab, kunya,
     title, mother's chain; split alias segments                       [2.4, 6.1]
  -> forms: raw | light | folded | consonant key                       [6.4, 7.4]
  -> identifiers: DOB (precision), POB, nationality, documents, family no.   [9]
INDEX (per list version, immutable snapshot, in memory)
  token index with fuzzy expansion + key index + birth-year buckets    [13.4]
QUERY (customer, counterparty, payment party)
  normalise -> expand tokens -> candidates (rarest-two intersection) ->
  aligned scoring (13.5) -> identifier adjustments -> evidence bits ->
  per-list thresholds and risk modifiers (13.6) -> pair memory check (13.8) ->
  alert + explanation (13.7) or no alert (with below-the-line sample)
```

### 13.3 Data model (per name, customer or list side)

`NameRecord { id, source (customer|list|payment), script, raw, light, folded, key, parts[ {text, folded, key, role (given|father|grandfather|great-grandfather|tribe|nisba|laqab|kunya|title|particle|mother-chain|unknown), idf_bits} ], alias_class (strong|weak|kunya|former), fold_flags, quality_flags (truncated, mixed-script, meaningless, default-date) }` and `Identifiers { dob {value, precision, default_flag}, pob {text, governorate}, nationality[], documents[ {type, number_norm} ], family_no, mother_chain }`. List entries add `entry_id, list, regime, measure (freeze|vigilance|PEP), version, effective_from, fingerprint`.

### 13.4 Index and candidate generation (blocking)

- **Token index** per list snapshot: vocabulary of normalised tokens with document frequencies (IDF), posting lists of entry ids, and a **deletion-neighbourhood index** (SymSpell-style) for fuzzy expansion: one deletion for tokens of 6 to 8 letters, two for 9 or more in inline mode (4 to 7 and 8 or more in batch mode).
- **Key index** for cross-script queries: loose consonant keys of the Arabic vocabulary with their own deletion index; a Latin token is expanded by key, then verified with the weighted key distance (7.4); only expansions with similarity 0.75 or more survive.
- **Candidates**: intersect the sorted posting lists of the **two rarest** aligned query tokens (galloping when the sizes differ by more than 32 times); apply the birth-year bucket (plus or minus 1) when the query has a year; cap at 100,000 candidates and **flag truncation** (never silent).
- **Insufficient-information outcome**: a query with fewer than two tokens, or only tokens above an information ceiling (for example the 200 commonest tokens) and no usable identifier, is returned as `needs_more_data` (policy: review, hold or pass) instead of enumerating huge candidate sets. This is both a latency and a false-positive control.
- **Kunya and alias entries** are indexed as separate names with their class; weak and kunya-only aliases are indexed for scoring support but excluded from candidate generation.
- Measured behaviour of this blocking: section 13.9.

### 13.5 Scoring (aligned chain score)

**Token similarity** s between a query token and a list token: 1.0 for equal folded tokens; for different tokens 1 minus edit distance over the longer length, allowed only within the gate (batch mode: no edit up to 3 letters, one up to 7, two above; inline mode: no edit up to 5, one up to 8, two above); lexicon-listed variants (for example Hussein spellings, Abd compounds) score 0.97; cross-script pairs use the weighted key similarity capped at 0.95; each name that needed a fold loses a small penalty (0.02 to 0.05, section 6.4).

**Alignment**: one-to-one assignment of query tokens to list tokens maximising the sum of s times weight (greedy for the prototype; Hungarian for up to eight tokens is cheap); roles are used as soft constraints (a given name should not align with a tribe), never as hard filters, because roles are often misparsed.

**Weights**: w = role weight times clipped IDF (clip 1 to 6 bits). Starting role weights: given name 1.0, father 0.9, grandfather 0.8, great-grandfather 0.7, tribe or nisba 0.9, kunya 0.5, laqab 0.5, title and particle 0. (IDF made no measurable difference in the prototype, section 12.4, so it stays a switch; role weights are untested starting values.)

**Name score** = sum of s·w over aligned pairs divided by [sum of aligned w + 0.8 × (unmatched query weight) + 0.2 × (unmatched list weight)] (the prototype's C; unmatched query parts are costlier because a customer name that has extra parts the list lacks points to a different person, while a list entry with extra parts usually means the query is truncated).

**Evidence bits** E_name = sum of IDF bits of aligned tokens (times s); with identifiers, E = E_name + bits for each agreeing identifier (from Table 8.2b, indicatively 5 for a birth year, 12 for a full mother's chain, 8 for a mother's first name, 20 or more for a document number) minus bits for contradictions.

**Identifier adjustments to the score** (prototype values, section 12.4): birth year 3 or more years apart -0.20, 2 years -0.08, 1 year -0.02; mother's chain agreement +0.08, disagreement -0.20; document number equal with equal type: score raised to at least 0.95 and escalated; same type but different number -0.30; nationality contradictions and place-of-birth contradictions -0.05 each (not validated); gender contradiction -0.10 with the unisex flag respected (not validated).

**Minimum evidence rules** (starting values, to be calibrated): at least two aligned tokens (one only if its IDF is 7.5 bits or more); E at least 10 bits for freeze lists and 8 bits for PEP lists; a family or tribe token plus one common given name never alerts alone; a kunya-only or weak-alias match never alerts alone.

**Alias handling**: strong alias at full weight; weak aliases (UN "Low" quality, OFAC's five criteria: short length, digits, common nickname words, geographic references, a very common prefix in a two-word name; kunya-only) cap at 0.6 of the weight and may only add to evidence from other fields; former names at 0.9.

### 13.6 Thresholds per list and per risk (starting values, to be calibrated per bank)

The prototype gave name-only thresholds of 0.905 to 0.955 and name-plus-identifier thresholds of 0.69 to 0.81 for alert budgets of 5% to 1% on a worst-case population (Table 12.4a). The proposal below turns that into configuration; **every number is a starting point to be set against an alert budget agreed with the MLRO and tested on the corpus of 12.6**.

| Context | Lists | Name only | With usable identifiers | Evidence floor | Outcome |
|---|---|---|---|---|---|
| Onboarding and KYC refresh (individuals) | UN, national list, OFAC, EU, UK | 0.95 | 0.78 | 10 bits | alert before the account is opened; four-eyes |
| Delta re-screen after a list change | same | 0.95 | 0.78 | 10 bits | within hours of publication; freeze workflow (file 04 section 4d) |
| Phase 1 transaction and counterparty screening (post-event) | same | 0.93 | 0.78 | 10 bits | near-real-time queue |
| Phase 2 pre-release payment screening | same | 0.93 | 0.78 | 9 bits | hold or release recommendation within the latency budget |
| PEP screening | PEP register | 0.90 | 0.75 | 8 bits | EDD workflow, not a freeze |
| Adverse media | media-derived entities | 0.92 | 0.78 | 9 bits | analyst review |
| Former-regime list (1518) with relatives in scope | UN Iraq list | chain-prefix mode: given, father and grandfather equal at 0.85 | 0.75 | 8 bits | human confirmation of family links |
| Internal lists | bank-defined | list-specific | list-specific | list-specific | as configured |

Risk modifiers (additive to the threshold): high-risk customer rating -0.03; payments in corridors with an Iran or other high-risk nexus -0.03; simplified due diligence customers +0.02; Arabic against Arabic exact-after-normalisation matches need no modifier. A **review band** of 0.05 below each threshold is sampled for below-the-line testing (section 11.6).

### 13.7 Explanation shown to the analyst

Every alert carries a structured explanation (fields below) and a plain-language rendering in Arabic and English. The example uses **invented names**.

```
ALERT  customer C-100234  vs  National list entry (decision 55 of 2026)      Score 0.84   Evidence 29 bits
Name (Arabic against Arabic, exact after normalisation)
   customer  : أحمد    جاسم    محمد    التميمي
   list entry: احمد     جاسم    محمد    عبدالله    التميمي
   alignment : given 1.00 (1.9 bits) | father 1.00 (4.1) | grandfather 1.00 (1.6) | tribe 1.00 (4.6)   = 12.2 bits
   not matched: list great-grandfather عبدالله (extra on the list, weight 0.2)
   folds     : hamza on the first letter of the given name (penalty 0.02); article stripped on the tribe token
Identifiers
   birth year: customer 1991 | list 1991                         agree (+5 bits)
   mother    : customer فاطمة علي حسن | list فاطمة علي حسن          agree (full chain, +0.08 score, +12 bits)
   document  : customer national card 1234... | list none          not comparable
   place of birth: customer Mosul | list not published            not comparable
Frequency context: given name + father + grandfather is shared by 0.3% of your customers (about 520 people);
   the tribe token is carried by 1.1% of your customers.
Score components: name 0.76, mother +0.08, birth year 0.00, fold penalty 0.00 -> 0.84 (threshold 0.78 with identifiers)
Pair memory: no earlier decision for this customer and list entry.
Would change if: a different mother's name or birth year more than 2 years apart -> score 0.56 (no alert).
```

For a cross-script alert the alignment lines show both forms, the consonant key and the key distance ("HSN against HSYN 0.10"), and the screen states plainly "Arabic against Latin alias, graded match (not exact)". The side-by-side layout, the highlighted distinguishing attributes (birth year, mother's name, nationality) and the one-click rationale picker follow Wolfsberg (file 04 section 2d) and EBA/GL/2024/15 ("must show which element of the measure was matched").

### 13.8 Pair-keyed false-positive memory

**Purpose**: once an analyst has decided that customer X is not listed person Y, the system must not raise the same alert again at every list update or daily run, but must raise it again as soon as anything that mattered has changed. Wolfsberg (PEP guidance, file 04 section 2n): a "no-match list keyed on the customer-PEP pair, invalidated when either record changes materially"; EBA/GL/2024/15: good-guy lists "with documented reasons" and reviewed "when measures change or customer data changes" (file 05).

**Key**: `(party_key, entry_key, scope, matching_version)` where `party_key` = customer id (or, for a payment party, a hash of the normalised name plus account number), `entry_key` = list entry id, `scope` = customer or payment or both, and `matching_version` = the major version of the matching semantics.

**Fingerprints stored with the decision**: `party_fp` = hash of the normalised fields that supported the match (name chain, mother's chain, birth date and precision, documents, nationality); `entry_fp` = hash of the same kind of fields of the list entry (names and aliases, birth data, documents, nationality, regime and measure).

**Validity rules**: the memory suppresses the alert only if the entry's `entry_fp` and the party's `party_fp` are unchanged, the matching major version is unchanged and the decision has not expired (default 12 months for sanctions lists, 24 months for PEP; configurable). It is **invalidated** when the list entry gains or changes any identifying field (new alias, new birth date, new document), when the customer's name, mother's name, birth data or documents change, when the decision was made under a configuration that has since been replaced by a major version, or on expiry. A new designation under a different list for the same person is a new entry and gets no memory.

**Decision record**: decision (false positive, true match, escalate), reviewer and approver (four-eyes for freeze lists), time, rationale code and free text, score and evidence snapshot, explanation hash.

**Controls**: every suppression is logged ("suppressed by decision D-...") and a random 5% sample plus all suppressions on freeze lists are re-reviewed weekly; bank-level rules that suppress by name pattern (not by pair) are separate "good-guy rules" with maker-checker and mandatory expiry; no automatic propagation to other customers with the same name; export of the memory is part of the CBDDQ and audit evidence (file 04 section 2c).

**Why pair-keyed**: the same common name will legitimately appear under many customers; only the pair (this person, that entry) with its evidence is a decision that can be remembered.

### 13.9 Phase-2 latency budget and measured headroom

Measured on a laptop-class CPU (AMD Ryzen AI 9 HX 370, 24 threads), Rust 1.97, single thread unless stated, **synthetic Arabic watchlist names drawn from the national list's token distribution with a long-tail vocabulary** (30% of tokens replaced from a pool of rare synthetic tokens: 300,000 for the 1.5-million-name list, 40,000 for 150,000 names and 6,000 for 15,000 names), 400 queries repeated three times, in-process (no network, no I/O), name normalisation, fuzzy expansion, candidate generation, aligned scoring of all candidates and top-10 ranking; identifiers except birth year and explanations not included.

| Watchlist size | Mode | p50 ms | p95 ms | p99 ms | max ms | Queries per second: 1 thread / 16 threads |
|---|---|---|---|---|---|---|
| 15,000 names | batch gate (3,7) | 0.01 | 0.06 | 0.43 | 4.8 | 29,500 / 326,000 |
| 150,000 | batch gate | 0.05 | 0.71 | 4.3 | 5.5 | 5,290 / 38,800 |
| 150,000 | inline gate (5,8) | 0.01 | 0.13 | 0.35 | 0.57 | 28,800 / 245,000 |
| 1,500,000 | batch gate | 0.49 | 9.3 | 22.0 | 27.6 | 550 / 4,260 |
| 1,500,000 | **inline gate (5,8)** | 0.09 | 1.9 | **6.2** | 11.8 | 2,310 / 22,700 |
| 1,500,000 | inline gate plus birth-year filter | 0.07 | 1.1 | **2.2** | 4.3 | not run |
| 150,000 | Latin query through the consonant key | 0.15 | 1.0 | 5.6 | 8.2 | not run |
| 1,500,000 | Latin query through the consonant key | 1.18 | 11.6 | 28.6 | 35.5 | not run |
| 1,500,000 | PostgreSQL `pg_trgm` GIN, threshold 0.3 (for comparison) | 684 | 2,255 | 2,739 | 3,013 | not applicable |

Other measurements: normalisation about 0.65 microseconds per name (1.5 million names in 1.0 s); index build 1.4 to 1.6 s for 1.5 million names; prototype memory about 0.75 GB resident (unoptimised, including build-time structures). The inline gate (exact up to 5 letters) lowers the 1.5-million-name p99 from 22 ms to 6 ms; its price is typo recall on short names (at a 2% alert budget the combined matcher's recall on one-typo variants falls from 0.65 to 0.48, section 12.4 and the gate ablation), which is why short common names rely on the variant lexicon and normalisation rather than fuzzy matching in inline mode.

**Budget for a pre-release payment check** (proposal, to be confirmed with each bank's gateway team: the timeout the gateway allows is **not verified**): TLS and authentication on the LAN 1 to 3 ms; request parsing and validation under 0.1 ms; normalisation of up to six names under 0.05 ms; candidate generation and scoring p99 under 10 ms at 1.5 million names (measured 6 ms in the prototype); identifiers, explanation and pair-memory lookup (Valkey) about 1 ms; response under 0.1 ms; audit write asynchronous. **Service levels proposed: p50 under 5 ms, p99 under 25 ms, hard timeout 250 ms, 99.9% of requests under 100 ms**, with a **margin of more than 100 times at list sizes up to 150,000 names** (the national list plus UN plus OFAC, EU and UK together are well below that; the large PEP files are the 1.5-million-name case).

**Design rules for meeting it**: immutable in-memory snapshots per list version, swapped atomically after a 1 to 2 second rebuild (no locks on the query path); one index per instance with identical results across instances; per-request time budget with graceful degradation (lower the candidate cap, set a `partial` flag, never skip silently); inline and batch modes selected by the caller; messages with several parties scored in parallel; backpressure and rate limits; fail-closed or fail-open on timeout is a **bank policy decision** recorded in the configuration (hold the payment for manual review, or release with post-event screening); startup self-tests (the pg_trgm ctype test of 10.3, normalisation golden file, a known-hit canary query) and a canary query on every snapshot swap; deterministic results independent of load.

### 13.10 Phase 1 operation

- **Delta screening both ways** (file 04 sections 2n and 4d): a new or amended list entry is queried against the customer index (and recent counterparties); a new or changed customer or counterparty is queried against all lists. A change set of a few dozen entries costs milliseconds.
- **Full re-screening** of 1 million customers against a 1.5-million-name watchlist takes about 7 minutes single-threaded in inline mode (2,310 queries per second), about 30 minutes in batch mode, and under 1 minute to about 4 minutes with 16 threads: **compute is not the constraint for the CBI's 12-hour refresh, analyst capacity is** (section 12.1).
- **Staged migration**: the first screening of a customer base runs at a high name-only threshold with identifier-backed alerts first, then lowers per segment as the alert queue allows, with each stage documented as a configuration change.
- **Lookback** over history from the read-only replica after any correction (file 04 section 2d).

### 13.11 Entities, special cases and failure modes

- **Entities**: same pipeline with the ORG tag: a lexicon of legal forms and generic words in Arabic and Latin (شركة, مؤسسة, مصرف, صرافة, المحدودة, ذ.م.م, للتجارة العامة, Co., Ltd, Est., Trading, General) with low weights; brand tokens high; numbers exact; vessel names with IMO numbers exact; in the UN list 45% of entity names contain a generic or legal-form word (section 8.1).
- **One-token and two-token names**: `needs_more_data` unless identifiers are present; do not alert on one common token.
- **Meaningless or default values** ("N/A", "unknown", dots, repeated characters, default dates): data-quality flag, no scoring credit (FATF R.16 rule pack, file 03).
- **Mixed-script tokens, homoglyphs, bidirectional control characters** in names are removed or flagged (Cf characters, Latin or Cyrillic lookalikes inside Arabic words); mixed-script names are scored per script.
- **Very long names**: more than 140 characters (ISO 20022 text limit) or 35-character line cuts are flagged as truncated and scored with the truncation rule (extra weight on missing parts is 0).
- **Reproducibility**: every alert stores the list version, the configuration version, the algorithm version and the lexicon version, so it can be replayed.
- **Open questions** for pilot banks: which CBS fields exist, how names are typed, how Latin names are produced, what gateway timeout applies, which lists the bank must screen beyond those named in the 2025 controls (UN, OFAC, EU and the national list).

### 13.12 AKR-TMS hooks (summary of what the pipeline needs the product to provide)

Normalisation library (Rust) with golden tests; typed name parser with Iraqi lexicons; Latin normaliser and consonant key; token and key indexes with snapshot swap; aligned scorer with evidence bits and identifier adjustments; per-list threshold configuration with risk modifiers and review band; explanation renderer (Arabic and English); pair memory with fingerprints and invalidation; evaluation harness and synthetic corpus; per-bank data-quality profile; bank-specific variant learning; OFAC-emulation oracle; `NameScorer` plug-in for vendor matchers; monitoring of alert rates, suppression rates and latency.

## 14. What it means for AKR-TMS

### 14.1 The ten findings that matter most

1. **Iraqi identity is a chain plus a mother's name plus a birth year, not "first name, last name".** The national card, the Iraqi KYC form and the national sanctions list all carry given name, father, grandfather (often great-grandfather and a tribe), the **mother's chain** and the **birth year** (sections 3, 6.5). The UN list has no structured mother's name, and its Iraq list has almost no document numbers.
2. **Common names make name-only matching unsafe.** Ten tokens are more than a third of all Iraqi name tokens; given name plus father is shared by about 1 customer in 1,000 and, in a bank of a million customers, by about 1,000 people per listed entry; the three-token chain still by about 55 (section 8.2).
3. **The OFAC-style per-name-part method, the model most screening tools copy, cannot get below a 9% to 22% alert rate on ordinary Iraqi names** because short customer names are contained in long list chains and score 100; token-set matching has the same floor (section 12.3).
4. **Secondary identifiers are the lever, not token weighting.** At a 2% alert budget, adding birth year and mother's name lifted mean recall on noisy variants from 0.44 to 0.84 (three-token names from 0.16 to 0.94), while IDF weights changed nothing measurable (section 12.4). This is why the mother's name and a structured birth date must be captured and why the explanation must show them.
5. **Arabic is the system of record; Latin is a family of spellings.** The same Arabic name appears in up to 13 Latin spellings in one UN file and 5 to 10 on Wikidata; two different names often score higher against each other than two spellings of one name (Salih and Salah 0.91 against Qasim and Kassem 0.70), and no Latin phonetic key separates them (sections 5.3, 10.2). Matching has to go through the Arabic form.
6. **Orthographic noise is everywhere and Unicode normalisation alone does not remove it.** NFC and NFKC leave ي, ى, ی, ك, ک, ه, ە, ة as different letters; 25.6% of the national list's core names contain a letter that the fold rewrites; the UN's own Arabic strings need NFKC for 1.8% of names; Kurdish and Persian keyboards produce different code points for the same visible word (sections 6, 4.2).
7. **The Iraqi national list embeds aliases in the name field** (26.9% of rows, "المكنى أبو X") and its public paging API **misses about 29% of the list**; the Excel export returns everything. The integration needs a parser for the alias format and a formal bulk feed from the AML/CFT Office (sections 6.1, 6.5).
8. **Cross-script matching works on full chains and fails on truncated ones.** A simple consonant key found the right Arabic record for a Latin name 99.5% of the time among 6,034 candidates, but only 33% of the time with two tokens; payment messages truncate at 35 characters and carry two or three tokens (sections 7.3, 5.2).
9. **A Rust in-memory pipeline meets a phase-2 budget with a wide margin**, where a PostgreSQL trigram index does not: p99 6 ms at 1.5 million names (2 ms with a birth-year filter) against p99 2.7 s for pg_trgm; and pg_trgm **returns nothing at all for Arabic in a database with the C ctype** (sections 13.9, 10.3).
10. **No off-the-shelf component covers the Arabic layer.** OpenSanctions' Rust core deliberately excludes Arabic transliteration; ICU4X's Arabic-Latin transform is unvocalised and ignores Kurdish letters; CAMeL Tools is a Python research toolkit and PyArabic is GPLv3. Everything needed exists as small Rust crates, but the **data** (lexicons, fold tables, key costs) and the **tests** (corpus, golden files) must be built by AKR-TMS.

### 14.2 Product decisions this file supports

- **Build the Arabic matching core in Rust as a product asset** (normalisation, parser, lexicons, key, indexes, aligner) with an evaluation harness and a shipped synthetic corpus; treat OpenSanctions, CAMeL Tools and vendor matchers as cross-checks and optional plug-ins.
- **Make secondary identifiers first-class in the data model and the UI**, and make a per-bank CBS discovery (which fields, how populated, how names are typed) the first step of every deployment.
- **Sell testability**: documented configuration, alert-budget curves, below-the-line samples, an OFAC-emulation oracle, a pair memory with audit trail and a validation pack that the bank's validator and its US and EU correspondents can reproduce.
- **Position the two latency modes** (inline for phase 2, batch for customer screening) as one algorithm with different blocking limits, and give banks the measured headroom and the explicit fail-open or fail-closed choice.
- **Plan data supply** with the AML/CFT Office (bulk or API-key feed of the national list with ids and versions) and a licensing path for PEP data (CC BY-NC data needs a commercial licence, file 15).

### 14.3 Risks and open items

- The experiments use **public and synthetic data**: the national list is not representative of a bank's customers, the noise model is assumed, and the OFAC-style comparator is an emulation on Arabic script. Banks' own labelled alerts are the real test (section 12.6).
- The per-list thresholds, role weights and evidence floors in section 13 are **starting values, not validated**; the parts validated by the prototype are the identifier adjustments and the alignment scores.
- **Kurdish, Turkmen, Assyrian, Armenian and Yazidi coverage** rests on secondary sources and general knowledge (section 4); it needs review by speakers of those communities and test data from banks in the Kurdistan Region and Nineveh.
- **The mother's name may not be structured in every CBS**; where it is missing, recall on short names falls back to the name-only curve (0.38 to 0.56 at a 1% to 5% alert budget).
- **Gateway timeouts, data licences, and the formal national-list feed** are unresolved external dependencies.
- Dates: the national list and the UN list change continuously; every number in this file is as of 2026-10-06.

## 15. Derived requirements

59 derived requirements. Source column: section of this file (and the external source where one is named).

| ID | Requirement | Source |
|---|---|---|
| N-01 | Store every customer and list name as a parsed chain with typed parts (given name, father, grandfather, great-grandfather, family or tribe, nisba, laqab, kunya, title, mother's chain, maternal grandfather) plus the raw string; never rely on a single surname field | 2.1-2.4; 3.1-3.2; IFLA 2018; UK Home Office profile; Uqudo IRQ_ID |
| N-02 | Parse the Iraqi list name formats (alias segments introduced by المكنى, الملقب, المعروف, المدعو or a parenthesis; 26.9% of national-list rows) into alias and kunya fields before matching | 6.1; 6.6 |
| N-03 | Maintain versioned Iraqi lexicons: tribes and families with variants, Abd and al-Din compounds (including Shia forms), titles, particles, kunya patterns, Kurdish, Syriac and Armenian given names and suffixes | 2.4; 4.4; 6.3 |
| N-04 | Treat matronymic chains (Mandaean bar or beth plus mother's name) as part of the person's own chain, not as a mismatch with a father-based chain | 4.1; 4.4 |
| N-05 | Never infer religion, sect or ethnicity from names; use name origin only to select transliteration and fold rules, and record this in the model-governance document | 3.4; 4.4 |
| N-06 | Implement one Rust normalisation library used by ingestion, batch screening and the phase-2 API, with a golden test file of 300 or more cases, idempotence tests and a pinned Unicode version | 6.2; 6.4; 6.6 |
| N-07 | Apply the Arabic normalisation steps N1 to N11 (NFKC with the U+FDFC and U+FDFD mapping, removal of Cf characters and tatweel, NFD and removal of marks, hamza fold, ى and ة folds, Persian and Kurdish folds, Eastern digits) and keep raw, light, folded and key forms | 6.2; 6.4; CAMeL Tools source |
| N-08 | Charge a small configurable fold penalty per name so that raw and light matches outrank folded ones | 6.4 |
| N-09 | Maintain the Kurdish and Persian letter fold table, the ZWNJ and Heh-plus-ZWNJ repair and the Turkish İ rule as versioned data validated with Kurdish and Turkmen speakers | 4.2; 4.3; 6.2 |
| N-10 | Join Abd, al-Din and kunya compounds canonically in Arabic and Latin through a closed lexicon, strip the definite article except in الله, and handle glued and separate forms identically | 6.3; 7.2; CLDR Arabic-Latin header |
| N-11 | Normalise Latin names with steps L1 to L10 (ASCII folding, ayn and hamza marks, hyphens, titles, nasab markers, article particles, Abd compounds, kunya, Arabizi digits) and reassemble multi-line SWIFT name fields with truncation flags | 5.2; 5.4; 7.2; Prowide Core field definitions |
| N-12 | Produce a per-bank name data-quality profile at onboarding (presentation forms, ZWNJ, Persian or Kurdish letters, double spaces, parentheses, Latin letters in Arabic fields, Eastern digits, compound forms, hamza and ya variants) and use it to set expected recall loss and fuzzy settings | 6.1; 6.6; 3.5 |
| N-13 | Use the normalised Arabic form as the system of record for matching; match Latin names through generated Arabic forms and the consonant key, never Latin against Latin by phonetic keys alone | 5.3; 5.4; 7.5; 10.2 |
| N-14 | Document the Arabic and Latin consonant key and its substitution costs as versioned configuration, and re-estimate the costs per bank from its own Arabic and Latin name pairs under data-governance approval | 7.4; 7.5 |
| N-15 | Index every name and alias with its normalised tokens, consonant key and script, with a per-list script plan (Arabic-only, Latin-only, both) | 7.5; 13.4 |
| N-16 | Allow Soundex, Metaphone, Double Metaphone and Beider-Morse only as optional blocking aids, never as decision rules, and never on Arabic script | 10.2; Tables 10.2a and 10.2b |
| N-17 | Build a given-name lexicon of about 3,000 Arabic, Kurdish, Syriac and Armenian names with Arabic spellings, Latin variants and keys, used for back-transliteration and as test oracle | 7.5; 5.3 |
| N-18 | Model identifiers with precision, source and quality (date of birth, place of birth, nationality, mother's chain, documents, family number, gender), identically for customers and list entries | 9.1; 9.5 |
| N-19 | Apply date-of-birth rules: year-only comparison with a tolerance of one year, default-date flags (1 January, 1 July), day and month swap as a soft match, contradiction only on reliable data | 9.2; Wolfsberg PEP guidance; UN DOB statistics |
| N-20 | Compare mother's chains with the same aligner as the person's own name; agreement adds, disagreement subtracts, missing is neutral | 9.2; 12.4 |
| N-21 | Normalise document numbers by type and match them exactly; a match escalates, a same-type mismatch is strong negative evidence | 9.2; 13.5 |
| N-22 | Provide an Iraqi place gazetteer (governorates, districts, cities in Arabic, Kurdish and Latin) and a nisba-to-place table as soft evidence | 9.2; 9.5 |
| N-23 | Provide a given-name gender lexicon with unisex flags for gender-contradiction evidence | 9.2; 9.5 |
| N-24 | Plan capture of the structured originator birth date and address fields that FATF R.16 (June 2025) requires by end-2030 at the latest | 9.1; 9.5; file 03 |
| N-25 | Score names by one-to-one token alignment with weights, with unmatched query parts costing more than unmatched list parts (0.8 and 0.2 as starting values) and explicit support for partial chains | 13.5; 12.4 |
| N-26 | Require minimum evidence for an alert: two aligned tokens (or one rare token), an evidence floor in bits, and no alert on a family or tribe token plus one common given name, a kunya alone or a weak alias alone | 8.4; 13.5 |
| N-27 | Classify aliases as strong, weak, kunya or former using the UN quality flag and OFAC's five weak-alias criteria, with a documented per-list policy (weak aliases never alert alone) | 2.3; 11.2; 11.3; 13.5 |
| N-28 | Compute token and chain frequency tables from the bank's own customer base monthly, with the national-list tables as a prior, and show them in the explanation | 8.2; 8.4; 13.7 |
| N-29 | Return a needs_more_data outcome for queries that are too short or consist only of the most common tokens and carry no usable identifier, instead of enumerating huge candidate sets | 13.4 |
| N-30 | Make the fuzzy-token gate by token length configurable per mode (inline and batch) and document its recall cost | 13.4; 13.9; 12.4 |
| N-31 | Hold thresholds per list and context with name-only and identifier tiers, an evidence floor, risk modifiers and a review band as governed configuration with maker-checker | 13.6; 11.6; file 04 section 2d |
| N-32 | Set thresholds against an explicit alert budget agreed with the MLRO and a recall floor per variant class, and show the resulting alert volumes | 12.1; 12.4 |
| N-33 | Export the screening configuration as the documented configuration requested by examiners and correspondents | 11.2; 11.4; 11.6 |
| N-34 | Attach a structured explanation to every alert (aligned parts with scores and information bits, folds applied, identifiers compared with agree, disagree or not comparable, frequency context, score components, what would change the decision), rendered in Arabic and English | 13.7; EBA/GL/2024/15; Wolfsberg 2019 |
| N-35 | Implement a pair-keyed false-positive memory (party, list entry, scope, matching version) with fingerprints, invalidation on material change, expiry, four-eyes for freeze lists, logging of every suppression and QA sampling | 13.8; Wolfsberg PEP 2017; EBA/GL/2024/15 |
| N-36 | Keep name-pattern good-guy rules separate from pair memory, with maker-checker and mandatory expiry | 13.8; 11.4 |
| N-37 | Ship an evaluation harness with a corpus loader, algorithm plug-ins, recall-at-alert-budget curves by variant class and release-to-release comparison | 12.1; 12.7 |
| N-38 | Generate and version a synthetic Iraqi name corpus (chain generator, variant generators, hard negatives, injected positives, labels, nomenklatura-compatible JSONL) and ship it with each release | 12.6 |
| N-39 | Adopt proposed acceptance targets for discussion: recall of at least 99.5% on clean and orthographic variants, at least 95% on truncated, reordered, typo and four-token variants when secondary identifiers are present | 12.6 |
| N-40 | Include an OFAC-emulation oracle and a list-lag report in the regression suite and the CBDDQ evidence pack | 11.3; 11.6; nomenklatura ofac.py |
| N-41 | Run weekly below-the-line sampling and provide a lookback tool over the read-only CBS history | 11.6; 12.7; Wolfsberg 2019 |
| N-42 | Support shadow-mode runs of a new configuration on live data with a signed promotion record | 12.7 |
| N-43 | Cross-check against nomenklatura logic-v2 and CAMeL Tools normalisation in the test pipeline only, not at run time | 10.5; 10.6 |
| N-44 | Feed every closed alert back as a labelled pair into threshold review, the bank-specific variant table and the hard-negative set | 12.7; 13.8 |
| N-45 | Ingest the Iraqi national list from a bulk or API-key feed negotiated with the AML/CFT Office (or the Excel export as a stop-gap), reconcile with rowCount, de-duplicate identical rows, detect deltas by (name, mother, birth year, decision), and keep every version | 6.5; file 01 section 6 |
| N-46 | Map UN alias quality, Arabic originals, date tolerance and name-part alignment (Arabic and Latin token counts differ in about one record in three) into the unified watchlist model | 8.1; file 04 section 4d |
| N-47 | Match entity names with the same pipeline, an Arabic and Latin legal-form and generic-word lexicon with low weights, exact numbers and exact vessel identifiers | 8.1; 13.11 |
| N-48 | Keep a third-party licence register: OpenSanctions data CC BY-NC (commercial licence needed), PyArabic GPLv3 (not linkable), CAMeL Tools MIT, rigour and nomenklatura MIT, Wikidata CC0, GeoNames CC BY | 10.5; 10.6; 12.2 |
| N-49 | Serve matching from immutable in-memory snapshots swapped atomically on list updates, with no locks on the query path | 13.9 |
| N-50 | Meet proposed inline service levels (p50 under 5 ms, p99 under 25 ms, hard timeout 250 ms, to be confirmed against each bank's gateway timeout) with a margin of more than 100 times at list sizes up to 150,000 names | 13.9; measured prototype |
| N-51 | Enforce a per-request time budget with a partial flag and make fail-open or fail-closed on timeout a recorded bank policy | 13.9 |
| N-52 | Run startup self-tests: an Arabic pg_trgm check that fails on an empty trigram set, the normalisation golden file, and a known-hit canary query on every snapshot swap | 10.3; 13.9 |
| N-53 | Use PostgreSQL pg_trgm only for analyst search and small lists, in a database created with a UTF-8 ctype | 10.3 |
| N-54 | Plan full re-screening compute and a staged migration of thresholds so that the first customer screening stays within analyst capacity | 13.10; 12.1 |
| N-55 | Stamp every alert with the list, configuration, algorithm and lexicon versions so that it can be replayed | 13.11 |
| N-56 | Run a CBS discovery per bank (name and mother's-name fields, Latin names, card and family numbers, keyboards, migrations, completeness) before go-live | 3.5 |
| N-57 | Learn a bank-specific Latin variant table from the Arabic names in the CBS and the Latin names in its payments and KYC fields, with governance approval | 5.4; 7.5 |
| N-58 | Treat a change of name on an existing customer as a data-quality alert and a re-screening trigger | 3.5; UK Home Office profile |
| N-59 | Reassemble multi-line payment name fields, split name from address by line position and flag names truncated at 35 characters | 5.4; 7.2; 13.11 |

## 16. Sources

All URLs were opened between 2026-10-06 and the time of writing; "blocked" means automated access was refused and the content was not read.

**Arabic and Iraqi names, identity documents**
- [IFLA, Names of Persons: Arabic Names (2018)](https://repository.ifla.org/handle/20.500.14598/3933), PDF at https://repository.ifla.org/bitstreams/e41641d3-309d-4108-ada8-86cf282ae354/download: elements ism, nasab, kunya, laqab, khitab, nisba, shuhra; modern usage
- [The Cultural Atlas, Iraqi culture: naming](https://theculturalatlas.org/iraqi-culture/iraqi-culture-naming) (redirected from culturalatlas.sbs.com.au): Iraqi chain, women keep names, Kurdish, Assyrian and Chaldean naming, "only names written in Arabic characters are considered official"
- [Mironova, "The Names of Jihad", The Majalla, 21 July 2017](https://en.majalla.com/node/40446): structure of jihadist noms de guerre
- [UK Home Office, Iraq knowledge base profile](https://www.gov.uk/government/publications/iraq-knowledge-base-profile/iraq-knowledge-base-profile): name construction, names on passports, no name change, civil status card
- [Sayari, Understanding Iraqi Naming Conventions (undated vendor article)](https://learn.sayari.com/?p=1041)
- [Uqudo docs, IRQ_ID fields](https://docs.uqudo.com/docs/kyc/uqudo-api/scan/country-specific-ids/irq_id-iraqi-id): national card name parts in Arabic, transliterated and "Kurdish to standard Arabic"
- [Iraqi Depository Center, individual account-opening and KYC form (Arabic, March 2026)](https://rs.iq/wp-content/uploads/2026/03/IDC-Contract-Individual_-ARABIC.pdf): four-part name with laqab slot and mother's name
- Wikipedia (secondary): [Iraqi passport](https://en.wikipedia.org/wiki/Iraqi_passport); [Kurdish alphabets](https://en.wikipedia.org/wiki/Kurdish_alphabets); [Kurdish typography](https://en.wikipedia.org/wiki/Kurdish_typography); [Arab tribes of Iraq](https://en.wikipedia.org/wiki/Arab_tribes_of_Iraq); [Armenian name](https://en.wikipedia.org/wiki/Armenian_name); [Mandaean name](https://en.wikipedia.org/wiki/Mandaean_name); [Iraqi Turkmen](https://en.wikipedia.org/wiki/Iraqi_Turkmen); [Yazidis](https://en.wikipedia.org/wiki/Yazidis); [Romanization of Arabic](https://en.wikipedia.org/wiki/Romanization_of_Arabic); [ISO 233](https://en.wikipedia.org/wiki/ISO_233); [Arabizi](https://en.wikipedia.org/wiki/Arabizi); page discovery through https://en.wikipedia.org/w/api.php
- [CTC Sentinel, jihadist presence in Syria: martyrdom notices](https://ctc.westpoint.edu/deciphering-the-jihadist-presence-in-syria-an-analysis-of-martyrdom-notices/): checked, no passage on name structure, not used

**Transliteration systems, Unicode, ICU**
- [ALA-LC Arabic Romanization Table, 2012 version, Wayback copy](https://web.archive.org/web/2024id_/https://www.loc.gov/catdir/cpso/romanization/arabic.pdf) (original https://www.loc.gov/catdir/cpso/romanization/arabic.pdf blocked)
- [UNGEGN Working Group on Romanization Systems, Arabic](https://arhiiv.eki.ee/wgrs/rom1_ar.htm)
- ICAO Doc 9303 Part 3 (https://icao.int/publications/Documents/9303_p3_cons_en.pdf) and TAG-MRTD papers (https://www.icao.int/Meetings/TAG-MRTD/Documents/Tag-Mrtd-18/TagMrtd18_wp02.pdf): blocked, not read
- [CLDR `Arabic-Latin.xml`](https://raw.githubusercontent.com/unicode-org/cldr/main/common/transforms/Arabic-Latin.xml) and [transform listing](https://api.github.com/repos/unicode-org/cldr/contents/common/transforms)
- [ICU4X `icu_experimental::transliterate`](https://docs.rs/icu_experimental/latest/icu_experimental/transliterate/index.html)
- Python 3.14.4 `unicodedata` (Unicode 16.0.0) for all normalisation tests in section 6.2

**Arabic NLP tools**
- [CAMeL Tools repository](https://github.com/CAMeL-Lab/camel_tools) (MIT); [normalize docs](https://camel-tools.readthedocs.io/en/latest/api/utils/normalize.html); [transliterate docs](https://camel-tools.readthedocs.io/en/latest/api/utils/transliterate.html); [normalize.py](https://raw.githubusercontent.com/CAMeL-Lab/camel_tools/master/camel_tools/utils/normalize.py); [ar2bw_map.json](https://raw.githubusercontent.com/CAMeL-Lab/camel_tools/master/camel_tools/utils/charmaps/ar2bw_map.json); [arclean_map.json](https://raw.githubusercontent.com/CAMeL-Lab/camel_tools/master/camel_tools/utils/charmaps/arclean_map.json)
- [PyArabic](https://github.com/linuxscout/pyarabic) (GPLv3)

**Algorithms, libraries, databases**
- [Cohen, Ravikumar, Fienberg (2003), A Comparison of String Distance Metrics for Name-Matching Tasks](https://www.cs.cmu.edu/~wcohen/postscript/ijcai-ws-2003.pdf)
- [Splink, term frequency adjustments](https://moj-analytical-services.github.io/splink/topic_guides/comparisons/term-frequency.html)
- [PostgreSQL pg_trgm documentation](https://www.postgresql.org/docs/current/pgtrgm.html); tests on the `postgres:16` and `postgres:16-alpine` Docker images
- [Apache Commons Codec Beider-Morse rule files](https://github.com/apache/commons-codec/tree/master/src/main/resources/org/apache/commons/codec/language/bm); crate [rphonetic](https://crates.io/crates/rphonetic)
- crates.io API metadata for strsim, rapidfuzz, triple_accel, unicode-normalization, icu, icu_normalizer, icu_experimental, unicode-segmentation, rphonetic, aho-corasick, fst, tantivy, symspell, any_ascii, deunicode, whatlang, ort and regex (https://crates.io/api/v1/crates/NAME)
- Python packages in the tests: rapidfuzz 3.14.6, jellyfish 1.2.1, Metaphone, PyArabic 0.6.15 (PyPI)
- [Prowide Core, SWIFT field 50K definition (SRU2025)](https://raw.githubusercontent.com/prowide/prowide-core/master/src/generated/java/com/prowidesoftware/swift/model/field/Field50K.java), and Field59.java and Field70.java in the same directory

**OpenSanctions**
- [Licensing](https://www.opensanctions.org/licensing/); [matching API documentation](https://www.opensanctions.org/docs/api/matching/); the pairs page (https://www.opensanctions.org/docs/pairs/) returned 404
- [nomenklatura](https://github.com/opensanctions/nomenklatura) (matching code under `nomenklatura/matching`, including [ofac.py](https://github.com/opensanctions/nomenklatura/blob/main/nomenklatura/matching/name_based/ofac.py), `logic_v2/`, `compare/names.py`, `pairs.py`); [rigour](https://github.com/opensanctions/rigour) (`rust/`, `CLAUDE.md`, `_core.pyi`); [yente](https://github.com/opensanctions/yente); all MIT

**OFAC**
- FAQs [5](https://ofac.treasury.gov/faqs/5), [122](https://ofac.treasury.gov/faqs/122), [124](https://ofac.treasury.gov/faqs/124), [125](https://ofac.treasury.gov/faqs/125), [246](https://ofac.treasury.gov/faqs/246), [247](https://ofac.treasury.gov/faqs/247), [248](https://ofac.treasury.gov/faqs/248), [249](https://ofac.treasury.gov/faqs/249); [weak aliases topic 1646](https://ofac.treasury.gov/faqs/topic/1646)

**Vendors (public pages only)**
- [NetOwl name matching](https://www.netowl.com/name-matching) and [NetOwl NameMatcher](https://www.netowl.com/name-matching-software/); [Babel Street modules](https://www.babelstreet.com/modules); Babel Street documentation (https://docs.babelstreet.com/Entity/en/name-similarity.html) is a JavaScript application and could not be read; IBM Information Server 11.7 documentation index (https://www.ibm.com/docs/en/iis/11.7.0?topic=name-recognition-overview) did not contain Global Name Recognition

**Data sets**
- UN Security Council consolidated list XML (https://scsanctions.un.org/resources/xml/en/consolidated.xml), parsed 2026-10-06 (file 04 section 4a)
- Iraqi national list: web application https://aml.iq, JSON `https://api.aml.iq/api/localsanctionslist/getall/{page}` and the export `https://api.aml.iq/api/localsanctionslist/getall/1?Type=0&Name=&IsExcelExport=true` (undocumented; section 6.5)
- [Wikidata SPARQL service](https://query.wikidata.org/sparql) (CC0): persons with citizenship Iraq and Arabic and English labels

**Other research files**
- Files 01 (sections 2d, 6, 7), 03 (R.16), 04 (sections 2c, 2d, 2m, 2n, 4), 05 (FFIEC, NYDFS 504, EBA/GL/2024/14 and 15) in `/home/haithem/Projects/AKR-TMS/research/`
