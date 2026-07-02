"""Align walkthrough video frames with their spoken captions.

Takes a video clip + its .srt subtitles (auto-subs from yt-dlp) and produces
captioned frames — one per subtitle cue, the caption burned in under the
frame — tiled into contact sheets. The result reads as an illustrated
route document: what the narrator says, paired with where he is on screen.
These sheets are the source for route waypoints in objectives.json.

    # fetch subs for the SAME time window as the clip:
    yt-dlp --skip-download --write-auto-subs --sub-lang en --convert-subs srt \
        --download-sections "*11:50-16:30" -o clipdir/subs "URL"
    python agent/video_align.py clip.mp4 subs.en.srt outdir [clip_start_sec]
"""

from __future__ import annotations

import os
import re
import subprocess
import sys

from PIL import Image, ImageDraw


def parse_srt(path: str) -> list[tuple[float, float, str]]:
    """-> [(start_s, end_s, text)] with duplicate rollups collapsed."""
    cues = []
    blocks = open(path, encoding="utf-8", errors="replace").read().split("\n\n")
    ts = re.compile(r"(\d+):(\d+):(\d+)[,.](\d+)\s*-->\s*(\d+):(\d+):(\d+)[,.](\d+)")
    for b in blocks:
        m = ts.search(b)
        if not m:
            continue
        g = [int(x) for x in m.groups()]
        t0 = g[0] * 3600 + g[1] * 60 + g[2] + g[3] / 1000
        t1 = g[4] * 3600 + g[5] * 60 + g[6] + g[7] / 1000
        text = " ".join(ln.strip() for ln in b.split("\n")
                        if ln.strip() and not ts.search(ln) and not ln.strip().isdigit())
        text = re.sub(r"<[^>]+>", "", text).strip()  # strip styling tags
        if text and (not cues or cues[-1][2] != text):
            cues.append((t0, t1, text))
    return cues


def grab(video: str, t: float, out: str):
    subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-ss", f"{t:.2f}",
                    "-i", video, "-frames:v", "1", "-vf", "scale=320:-1", out],
                   check=True)


def caption_frame(png: str, text: str, stamp: str) -> Image.Image:
    im = Image.open(png).convert("RGB")
    w, h = im.size
    pad = 46
    out = Image.new("RGB", (w, h + pad), "black")
    out.paste(im, (0, 0))
    d = ImageDraw.Draw(out)
    d.text((4, h + 2), stamp, fill="yellow")
    # naive wrap at ~52 chars, two lines max
    words, lines, cur = text.split(), [], ""
    for wd in words:
        if len(cur) + len(wd) + 1 > 52:
            lines.append(cur)
            cur = wd
            if len(lines) == 2:
                break
        else:
            cur = f"{cur} {wd}".strip()
    if cur and len(lines) < 2:
        lines.append(cur)
    for i, ln in enumerate(lines[:2]):
        d.text((4, h + 14 + i * 12), ln, fill="white")
    return out


CLIP_LEN = None


def main():
    video, srt, outdir = sys.argv[1], sys.argv[2], sys.argv[3]
    base = float(sys.argv[4]) if len(sys.argv) > 4 else 0.0  # video-time of clip t=0
    global CLIP_LEN
    CLIP_LEN = float(sys.argv[5]) if len(sys.argv) > 5 else None
    os.makedirs(outdir, exist_ok=True)
    cues = parse_srt(srt)
    tiles = []
    for i, (t0, t1, text) in enumerate(cues):
        mid = (t0 + t1) / 2 - base
        if mid < 0 or (CLIP_LEN and mid > CLIP_LEN):
            continue
        raw = os.path.join(outdir, f"_raw{i:03d}.png")
        try:
            grab(video, mid, raw)
        except subprocess.CalledProcessError:
            continue
        stamp = f"{int((base + mid) // 60)}:{int((base + mid) % 60):02d}"
        tiles.append(caption_frame(raw, text, stamp))
        os.remove(raw)
    # tile into sheets of 2x3
    per, w, h = 6, tiles[0].width, tiles[0].height
    for s in range(0, len(tiles), per):
        sheet = Image.new("RGB", (w * 2, h * 3), "black")
        for j, im in enumerate(tiles[s:s + per]):
            sheet.paste(im, ((j % 2) * w, (j // 2) * h))
        sheet.save(os.path.join(outdir, f"aligned{s // per:02d}.png"))
    print(f"{len(tiles)} captioned frames -> "
          f"{(len(tiles) + per - 1) // per} sheets in {outdir}")


if __name__ == "__main__":
    main()
