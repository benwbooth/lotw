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

    # already-ported addresses + addr->name map (from specs, the source of truth)
    ported, name_of = {}, {}
    for p in SPECS.glob("*.json"):
        s = json.loads(p.read_text())
        a = int(s["addr"], 16)
        ported[a] = s["name"]
    name_of = {f"{a:04X}": symbols.ROUTINES.get(a, f"sub_{a:04X}") for a in targets}

    # Known not-portable-in-isolation (dispatchers, spin-waits, RTS-trampolines)
    # identified in the first round; their patterns aren't caught by has_indirect.
    EXCLUDE = {0xCC97, 0xCC8F, 0xE642, 0xE620, 0xD64F, 0xF01E, 0xEA94,
               0xCC9C, 0xCCE4, 0xCD08, 0xC833}

    ready, blocked_hw, blocked_indirect, blocked_deps = [], 0, 0, 0
    for t in sorted(targets):
        if t in ported or t in EXCLUDE:
            continue
        r = analyze(mem, t, targets)
        if r["reads_dyn"]:        # reads a dynamic input register (controller/PPU) — needs harness modelling
            blocked_hw += 1; continue
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
