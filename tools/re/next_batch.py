#!/usr/bin/env python3
"""Compute the next dependency-ordered porting batch: routines whose JSR/tail
callees are ALL already ported (so the C port can call the ported C versions),
excluding hardware-touching, indirect-jump (control-transfer), and already-ported
routines. Emits build/port_batch2.json and the callee name map agents need."""
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from worklist import build_mem, gather_entries, all_jsr_targets, analyze
import symbols

ROOT = Path(__file__).resolve().parents[2]
SPECS = ROOT / "port" / "specs"


def main():
    mem = build_mem()
    fix, b13 = gather_entries()
    targets = {t for t in all_jsr_targets(mem, fix, b13) if 0xA000 <= t < 0x10000}
    # Jump-table handlers are reached via indirect JMP, not JSR, so they aren't in
    # the JSR-target set. Add the curated code entries (disasm/entries.txt, which
    # the completeness audit populated with these handlers) to the routine universe.
    ent = ROOT / "disasm" / "entries.txt"
    if ent.exists():
        for ln in ent.read_text().splitlines():
            ln = ln.split("#", 1)[0].split()
            if ln:
                a = int(ln[0], 16)
                if 0xA000 <= a < 0x10000:
                    targets.add(a)

    # Ported = every src/ported/<name>.c (includes inspection-ported routines that
    # have no diff-test spec, e.g. statusbar_split). Resolve name -> address.
    name2addr = {v: k for k, v in symbols.ROUTINES.items()}
    ported = {}
    seen_names = set()
    for p in SPECS.glob("*.json"):          # specs carry the authoritative address
        s = json.loads(p.read_text())
        ported[int(s["addr"], 16)] = s["name"]
        seen_names.add(s["name"])
    for cf in (ROOT / "src" / "ported").glob("*.c"):   # spec-less inspection ports
        nm = cf.stem
        if nm in seen_names:
            continue
        a = int(nm[4:], 16) if nm.startswith("sub_") else name2addr.get(nm)
        if a is not None:
            ported[a] = nm
    name_of = {f"{a:04X}": symbols.ROUTINES.get(a, f"sub_{a:04X}") for a in targets}

    # Known not-portable-in-isolation: dispatchers, RTS-trampolines, and spin-waits
    # (loop until an NMI/PPU-driven var changes — the oracle has no such driver, so
    # they never terminate). $D36E = sprite-0-hit status-bar split (spin-wait).
    # Genuinely not portable as isolated Regs-ABI functions: the two indirect
    # far-call dispatchers (dissolved — callers inline them) and the RTS-
    # trampolines (manipulate the return stack). Everything else is fair game.
    EXCLUDE = {0xCC9C, 0xCCE4, 0xD64F, 0xE620, 0xE642}

    ready, blocked_hw, blocked_indirect, blocked_deps = [], 0, 0, 0
    for t in sorted(targets):
        if t in ported or t in EXCLUDE:
            continue
        r = analyze(mem, t, targets)
        # Note: reads_dyn (bounded reads of $4016/$2002/$2007) ARE diff-testable
        # under flat-memory register semantics (oracle and host read the same
        # NES_MEM[addr]); only spin-waits aren't, and those self-eliminate via
        # oracle timeout -> FAIL -> agent SKIP. So we no longer block on reads_dyn.
        if r["has_indirect"]:
            blocked_indirect += 1; continue
        deps = {c for c in r["callees"] if 0xA000 <= c < 0x10000}
        unmet = deps - set(ported)
        if unmet:
            blocked_deps += 1; continue
        ready.append({"addr": f"{t:04X}", "name": symbols.ROUTINES.get(t, f"sub_{t:04X}"),
                      "size": r["size"], "deps": sorted(f"{d:04X}" for d in deps)})

    ready.sort(key=lambda x: (len(x["deps"]), x["size"]))
    out = {"ready": ready, "callee_names": name_of}
    (ROOT / "build" / "port_batch2.json").write_text(json.dumps(out))
    print(f"ported so far: {len(ported)}")
    print(f"READY this round (all callees ported, non-hw, non-indirect): {len(ready)}")
    print(f"  still blocked: deps-not-ported={blocked_deps}, hardware={blocked_hw}, "
          f"indirect/control-transfer={blocked_indirect}")
    print("  sample ready:", [f"{r['name']}({r['size']}B,{len(r['deps'])}deps)" for r in ready[:12]])


if __name__ == "__main__":
    main()
