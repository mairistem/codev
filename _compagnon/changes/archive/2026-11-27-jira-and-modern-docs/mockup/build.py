"""Assemble the mockup page from its source and the repository's logos.

Run from the repository root:  python3 _compagnon/changes/jira-and-modern-docs/mockup/build.py
It reads mockup/src.html, replaces @@CP_LOGO@@ by a <symbol id="cp-logo"> built
from docs/assets/compagnon-logo.svg (ink and C stroke through CSS custom
properties, so one symbol serves light and dark), @@MX_LOGO@@ by the Mairistem
<symbol> of docs/theme/header.hbs, and writes mockup/index.html.
"""
import pathlib
import re

root = pathlib.Path(__file__).resolve().parents[4]
here = pathlib.Path(__file__).resolve().parent

logo = (root / "docs/assets/compagnon-logo.svg").read_text()
inner = re.search(r"<title[^>]*>.*?</title>(.*)</svg>", logo, re.S).group(1)
inner = inner.replace('fill="#000000"', 'style="fill:var(--logo-ink,#000)"')
inner = inner.replace('stroke="#004c66"', 'style="stroke:var(--logo-c,#004c66)"')
cp_symbol = f'<symbol id="cp-logo" viewBox="0 0 1188 250">{inner}</symbol>'

header = (root / "docs/theme/header.hbs").read_text()
mx_symbol = re.search(r'<symbol id="mx-logo".*?</symbol>', header, re.S).group(0)

src = (here / "src.html").read_text()
out = src.replace("@@CP_LOGO@@", cp_symbol).replace("@@MX_LOGO@@", mx_symbol)
for key, name in [("JIRA", "jira"), ("WORKFLOW", "workflow"), ("PLAN", "plan")]:
    part = (here / f"{name}-diagram.html").read_text()
    n = 0
    while f"@@{key}_DIAGRAM@@" in out:
        n += 1
        # marker ids must stay unique when a diagram appears twice
        copy = part.replace('id="dg-arrow"', f'id="dg-arrow-{n}"').replace("url(#dg-arrow)", f"url(#dg-arrow-{n})")
        out = out.replace(f"@@{key}_DIAGRAM@@", copy, 1)
(here / "index.html").write_text(out)
print(f"wrote {here / 'index.html'} ({len(out)} bytes)")
