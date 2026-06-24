#!/usr/bin/env bash
# Regenerate the 6502 reference disassembly + maps under reference/asm/.
# Run from the repo root. Needs rom/lotw.nes and python3.
set -euo pipefail
cd "$(dirname "$0")/.."
ROM=rom/lotw.nes
OUT=reference/asm
mkdir -p "$OUT"

# Per-bank disassembly. MMC3 8KB banks: 0-13 are switchable (shown at $8000;
# the same bank can also map at $A000 — internal branches are base-relative so
# the structure is identical), bank 14 is fixed at $C000, bank 15 at $E000.
for n in $(seq 0 15); do
  case $n in 14) base=C000;; 15) base=E000;; *) base=8000;; esac
  printf '; LotW PRG bank %d (8KB), disassembled at $%s\n' "$n" "$base" > "$OUT/bank-$(printf '%02d' "$n").asm"
  LOTW_ROM=$ROM python3 tools/dis6502.py bank "$n" "$base" >> "$OUT/bank-$(printf '%02d' "$n").asm"
done

# Zero-page field map (offset -> GameState field) from the offset_of! asserts.
python3 - <<'PY'
import re
s=open('src/state.rs').read()
pairs=sorted((int(o,16),n) for n,o in re.findall(r'offset_of!\(GameState,\s*(\w+)\)\s*==\s*(0x[0-9A-Fa-f]+)',s))
open('reference/asm/fieldmap.txt','w').write(''.join(f'${o:04X} {n}\n' for o,n in pairs if o<0x800))
PY

# Zero-page cross-reference and JSR entry points from the disassembly.
python3 - <<'PY'
import re,glob,collections
lines=[l for f in sorted(glob.glob('reference/asm/bank-*.asm')) for l in open(f)]
jsr=sorted(set(m.group(1) for l in lines for m in [re.search(r'JSR \$([0-9A-F]{4})',l)] if m))
open('reference/asm/jsr_targets.txt','w').write('\n'.join('$'+t for t in jsr)+'\n')
xref=collections.defaultdict(list)
for l in lines:
    m=re.match(r'([0-9A-F]{4}): \w+ [#(]?\$([0-9A-F]{2})(?![0-9A-F])',l)
    if m: xref[m.group(2)].append(m.group(1))
with open('reference/asm/zp_xref.txt','w') as f:
    for zp in sorted(xref): f.write(f'${zp}: {len(xref[zp])} refs: {" ".join(xref[zp][:60])}\n')
PY
echo "regenerated reference/asm/ ($(ls reference/asm/*.asm | wc -l) banks)"
