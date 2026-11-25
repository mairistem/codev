# Generates the "Compagnon by Mairistem" logo on the Mairistem logo grid.
# Fonts: Outfit (SIL Open Font License), weights 400/600/700, from
# https://fonts.google.com/specimen/Outfit; needs fontTools.
# Usage: python generate-logo.py <out-dir> <fonts-dir>
import math, sys
from fontTools.ttLib import TTFont
from fontTools.pens.svgPathPen import SVGPathPen
from fontTools.pens.transformPen import TransformPen
from fontTools.pens.boundsPen import BoundsPen
out, fonts = sys.argv[1], sys.argv[2]
B, C, J, V = "#004c66", "#ff5331", "#ffab03", "#00c39b"

def mark(ink, s, crop=False):
    # The open C (the master) and the three compagnons, on a 250 grid scaled
    # to s. Cropped, the drawing's own bounds (x 19-238, y 26-224) fill the
    # height, as Mairistem's icon fills its logo's.
    if crop:
        k = s / 198
        return (f'<g transform="scale({k:.5f}) translate(-19,-26)">' + mark(ink, 250)[len('<g transform="scale(1.00000)">'):]) 
    k = s / 250
    a0, a1 = math.radians(40), math.radians(320)
    x0, y0 = 118 + 82*math.cos(a0), 125 + 82*math.sin(a0)
    x1, y1 = 118 + 82*math.cos(a1), 125 + 82*math.sin(a1)
    return (f'<g transform="scale({k:.5f})">'
            f'<path d="M{x0:.2f},{y0:.2f} A82,82 0 1 1 {x1:.2f},{y1:.2f}" fill="none" stroke="{ink}" stroke-width="34" stroke-linecap="round"/>'
            f'<circle cx="214" cy="88" r="16" fill="{C}"/><circle cx="222" cy="125" r="16" fill="{J}"/>'
            f'<circle cx="214" cy="162" r="16" fill="{V}"/></g>')

def outline(font_file, text, cap, x, baseline, tracking=0.0):
    f = TTFont(font_file); gs = f.getGlyphSet(); cmap = f.getBestCmap()
    capH = f["OS/2"].sCapHeight; s = cap / capH; hmtx = f["hmtx"]
    parts, cx = [], x
    for ch in text:
        g = cmap[ord(ch)]
        pen = SVGPathPen(gs)
        gs[g].draw(TransformPen(pen, (s, 0, 0, -s, cx, baseline)))
        parts.append(pen.getCommands())
        cx += hmtx[g][0]*s + tracking
    return " ".join(p for p in parts if p), cx - tracking

# Mairistem's grid, measured on its SVG: a 238 square icon; the wordmark's
# capitals (118 high) centred on the icon; the byline (capitals 35 high)
# on a baseline 10 below the icon's bottom, right-aligned with the
# wordmark. Compagnon has descenders (p, g) where Mairistem has none: the
# capitals shrink only as much as keeps them 6 above the byline.
S, BYCAP, GAP = 238, 35, 59
f = TTFont(f"{fonts}/Outfit-600.ttf")
os2 = f["OS/2"]
gs = f.getGlyphSet(); cmap = f.getBestCmap()
bp = BoundsPen(gs); gs[cmap[ord("g")]].draw(bp)
desc_ratio = -bp.bounds[1] / os2.sCapHeight
BYBASE = S + 10
by_top = BYBASE - BYCAP
CAP = 118
while S / 2 + CAP / 2 + desc_ratio * CAP > by_top - 6:
    CAP -= 1
BASE = S / 2 + CAP / 2
H = math.ceil(BYBASE + 2)
ICON_W = 219 * S / 198
x0 = ICON_W + GAP
word, word_end = outline(f"{fonts}/Outfit-600.ttf", "Compagnon", CAP, x0, BASE)
by_w = outline(f"{fonts}/Outfit-400.ttf", "BY", BYCAP, 0, BYBASE, 2)[1]
space = 0.32 * BYCAP
brand_w = outline(f"{fonts}/Outfit-700.ttf", "MAIRISTEM", BYCAP, 0, BYBASE, 2)[1]
start = word_end - (by_w + space + brand_w)
by, by_end = outline(f"{fonts}/Outfit-400.ttf", "BY", BYCAP, start, BYBASE, 2)
brand, _ = outline(f"{fonts}/Outfit-700.ttf", "MAIRISTEM", BYCAP, by_end + space, BYBASE, 2)
print("cap", CAP)
W = math.ceil(word_end) + 2
def svg(vb, w, h, body, label="Compagnon by Mairistem"):
    return (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="{vb}" width="{w}" height="{h}" role="img" '
            f'aria-labelledby="t"><title id="t">{label}</title>{body}</svg>\n')
for suffix, ink, text in (("", B, "#000000"), ("-dark", "#ffffff", "#ffffff")):
    body = (mark(ink, S, crop=True) + f'<path d="{word}" fill="{text}"/>'
            f'<path d="{by}" fill="{text}"/><path d="{brand}" fill="{text}"/>')
    open(f"{out}/compagnon-logo{suffix}.svg", "w").write(svg(f"0 0 {W} {H}", W, H, body))
    open(f"{out}/compagnon-mark{suffix}.svg", "w").write(svg("0 0 250 250", 250, 250, mark(ink, 250), "Compagnon"))
print(W, H)
