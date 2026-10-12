# Provenance: GTA San Andreas modding — state of the art

- **Date:** 2026-10-12
- **Rounds:** 8 across two increments.
  - *Increment 1 (classic SA SOTA):* (1) lead recon + 4 parallel researcher
    subagents; (2) verifier pass; (3) reviewer pass; (4) lead revision clearing
    2 FATAL + 4 MAJOR.
  - *Increment 2 (Linux / Steam / Proton, requested after delivery):* (5) lead
    recon + 3 parallel researcher subagents; (6) **lead independent
    re-verification via live curl** of the load-bearing endpoints; (7) verifier
    pass on the merged document (64 new URLs; 10 mis-targeted citations fixed);
    (8) reviewer pass (1 FATAL + 3 MAJOR + 6 MINOR) and lead revision clearing
    all of them.
- **Scale decision:** broad multi-faceted survey → 4 researcher subagents
  (tooling; overhauls/reverse-engineering; multiplayer; content+ecosystem).
  Recorded in `outputs/.plans/gta-san-andreas-modding-sota.md` before owners
  were assigned.
- **Sources consulted:** increment 1 — 4 per-facet research files (44 + 48 + 50 +
  58 numbered entries), 1 lead research note (13 searches + 6 direct fetches),
  2 court documents, 1 GitHub DMCA notice. Increment 2 — 3 Linux/Proton research
  files (48 + 35 + 33 numbered entries) plus a lead note; live curl reads of
  Valve's Deck compatibility endpoint, Valve's appdetails API, the ProtonDB
  summary API (both appids), the GE-Proton releases API, and the
  `ValveSoftware/wine` commit.
- **Sources accepted:** 48 unified plain `[S#]` keys in the delivered brief plus
  ~30 cross-file `[S#-T2/T3/T4]` keys resolving into the research files.
  Verified-primary set includes CLEO 5/4/Redux releases, ModLoader releases +
  README, Ultimate ASI Loader releases, SilentPatch home/releases/2026 update,
  Open Limit Adjuster releases, fastman92 changelog + repo, GTAMods Wiki format
  pages, open.mp site + releases, MTA releases, sa-mp.mp, the re3 DMCA notice,
  N.D. Cal. complaint + dismissal, and the [2024] FCA 1459 judgment.
- **Sources rejected / downgraded:**
  - `gtamods.com/wiki/Audio`, `/Sound_Effects`, `/Streaming_Audio` — HTTP 404
    (replaced with `Audio_stream` and `SFX_(SA)`).
  - `https://github.com/GTAmodding/re3` — HTTP 451; **kept intentionally as
    evidence**, not rejected.
  - `gtaforums.com`, `steamdb.info/app/12120/patchnotes/`, `insider-gaming.com`
    — HTTP 403 to our fetcher; recorded as accessible-but-blocked, not rejected
    as false.
  - `https://www.silentsblog.com/mods/gta-sa/` — resolves but serves **GInput**
    content; rejected as a SilentPatch citation target and replaced with the
    verified `https://www.silentsblog.com/silentpatch`.
  - "plugin-sdk-sa 100% reverse-engineered" — the source README does **not** say
    "100%"; the figure was removed.
  - "SA Community Patch" — no such project found; removed and replaced with the
    real nearest matches.
  - "SAMP Addon 2.8" (third-party mirror only) — dropped as unverified.
  - Mirror-lagging version numbers (SilentPatch "v.33.1", Ultimate ASI Loader
    "v.9.7.0") — superseded by GitHub Releases API values (34.1, 9.7.4).
  - "Multiplayer has fully migrated off SA-MP" — removed as unsupported
    (reviewer FATAL W1).
- **Verification:** **PASS WITH NOTES** (both increments)
  - Increment 2 was verified twice over: the verifier checked all 64 new `[L#]`
    URLs (61 live; L44–L51/L64 redirect to canonical equivalents) and fixed 10
    citations that pointed at the wrong page for their claim plus one
    commit-date conflation; the **lead independently reproduced** the four
    load-bearing values by direct curl — Deck `resolved_category: 2` for appids
    12120 and 1547000, ProtonDB Gold `0.71/290` and `0.69/53`, and HTTP 200 for
    the `ValveSoftware/wine` commit `09af566a…`.
  - Reviewer pass 2 returned **BLOCKED** on 1 FATAL (a §10.0 claim that the v1.0
    downgrade is a step "the Windows guides do not require", contradicted by the
    report's own §1.3) plus 3 MAJOR and 6 MINOR. **All 10 were fixed**; the
    headline correction is that only the DLL override is Linux-specific.
  - Verifier: 48/48 plain URLs fetched or cross-checked; all cross-file
    `[S#-T*]` keys resolve; duplicate keys split; one overclaim removed.
  - Reviewer: 2 FATAL + 4 MAJOR + 9 MINOR findings. FATALs fixed; MAJORs softened
    or carried in §8 Open questions; MINORs applied.
  - Residual unverified items (documented in-file): fastman92 FLA 7.6 release
    date; MoonLoader `026`/`0.27` label + date; SAMPFUNCS 5.7.1; Silent's ASI
    Loader v1.5.0 date; Magic.TXD v1.1; GTA: Recall release status; sa-mp.com
    exact shutdown day; open.mp backwards-compatibility vendor claim.
  - Not independently verified (blocked, recorded): Rockstar Newswire body;
    Federal Court of Australia judgment page (403); Ars Technica article (405);
    the two DOIs/PDFs noted in-file.
- **Plan:** `outputs/.plans/gta-san-andreas-modding-sota.md` (both increments,
  including the pre-assigned L1–L3 owner ledger)
- **Research files:**
  - `outputs/.drafts/gta-san-andreas-modding-sota-research-tooling.md`
  - `outputs/.drafts/gta-san-andreas-modding-sota-research-overhauls.md`
  - `outputs/.drafts/gta-san-andreas-modding-sota-research-multiplayer.md`
  - `outputs/.drafts/gta-san-andreas-modding-sota-research-content-ecosystem.md`
  - `outputs/.drafts/gta-san-andreas-modding-sota-research-lead.md`
- **Intermediate artifacts:**
  - `outputs/.drafts/gta-san-andreas-modding-sota-draft.md`
  - `outputs/.drafts/gta-san-andreas-modding-sota-cited.md`
  - `outputs/.drafts/gta-san-andreas-modding-sota-verification.md` (reviewer report)
  - `outputs/.drafts/gta-san-andreas-modding-sota-revised.md` (final candidate, inc. 1)
  - `outputs/.plans/gta-san-andreas-modding-sota-workflow.js`
  - **Increment 2:** `outputs/.drafts/gta-san-andreas-modding-sota-research-linux-{proton,native,deck-mp,lead}.md`;
    `-section10.md`; `-draft2.md`; `-cited2.md`; `-verification2.md` (reviewer report);
    `-revised2.md` (final candidate); `outputs/.plans/gta-san-andreas-modding-sota-workflow2.js`
    and briefs `-L1.md`, `-L2.md`, `-L3.md`

## Increment 2 (Linux / Steam / Proton) — summary

- **Question asked:** can GTA:SA be modded on Linux starting from the **Steam**
  version run via **Proton**?
- **Answer shipped in §10:** yes, and the ecosystem is broadly intact, but with
  exactly **one Linux-only step** (a `dinput8` DLL override) plus a v1.0
  downgrade that any Steam install needs on any OS. There is **no native-Linux
  modding path** (an unverified negative). Multiplayer has native Linux servers
  but no native Linux clients.
- **Largest limitation, stated three times in the report:** **nothing in §10 was
  reproduced on hardware.** No GTA install, Steam client or Wine prefix was
  available; every procedural claim is source-reported.
- **Correction the increment forced:** an earlier planning assumption that
  `re3`/`librw` might offer a native SA path was **wrong** — `re3`/`reVC` cover
  GTA III and Vice City only, and there is no `reSA`.
