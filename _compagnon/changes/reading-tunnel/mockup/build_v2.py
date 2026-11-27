"""Build version 2 of the tunnel mockup: real pages in the approved modern style.

Run from anywhere: python3 build_v2.py. Writes index.html next to this file
(version 1 is kept as index-v1.html). It takes the logo from
docs/assets/compagnon-logo.svg and the diagrams from the chapters that own
them, so the mockup draws what the books will draw.
"""
import html
import pathlib
import re

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[3]
e = html.escape


def figure(path):
    md = (ROOT / path).read_text()
    return re.search(r'<figure class="diagram">.*?</figure>', md, re.S).group(0)


logo = (ROOT / "docs/assets/compagnon-logo.svg").read_text()
inner = re.search(r"<title[^>]*>.*?</title>(.*)</svg>", logo, re.S).group(1)
inner = inner.replace('fill="#000000"', 'style="fill:var(--logo-ink)"').replace(
    'stroke="#004c66"', 'style="stroke:var(--logo-c)"'
)
LOGO = f'<svg width="0" height="0" aria-hidden="true" style="position:absolute"><symbol id="cp-logo" viewBox="0 0 1188 250">{inner}</symbol></svg>'

STEPS = [
    ("Discover", ["Compagnon in two minutes"]),
    ("Get started", ["Installation", "Your first change"]),
    ("Every day", ["Asking Claude", "Start from a Jira ticket", "Reviewing a plan", "Approving a plan on GitHub", "Building and filing away"]),
    ("Understand", ["How it works", "Concepts", "Decisions", "Roles", "Review record"]),
    ("For your team", ["Adopting Compagnon on an existing project", "Deploying Compagnon for a team", "Inherited sources", "Artifact language", "Release notes and CHANGELOG", "Custom schemas"]),
]
COLORS = ["#ff5331", "#ffab03", "#00c39b", "#3a9ec8", "#8fa8b1"]


def sidebar(cur_step, cur_page):
    out = ['<ol class="stepper">']
    for i, (name, pages) in enumerate(STEPS):
        state = "done" if i < cur_step else "current" if i == cur_step else "todo"
        mark = "✓" if state == "done" else str(i + 1)
        items = ""
        if i == cur_step:
            items = "<ol class='pages'>" + "".join(
                f'<li class="{"here" if j == cur_page else ("read" if j < cur_page else "")}"><a href="#">{e(p)}</a></li>'
                for j, p in enumerate(pages)
            ) + "</ol>"
        out.append(
            f'<li class="st {state}" style="--c:{COLORS[i]}"><span class="dot" aria-hidden="true">{mark}</span>'
            f'<span class="st-name">{e(name)}<small>{len(pages)} page{"s" if len(pages) > 1 else ""}</small></span>{items}</li>'
        )
    out.append("</ol>")
    out.append('<p class="appx">Appendices · Reference · Glossary · FAQ</p>')
    return "\n".join(out)


def progress(cur_step, cur_page):
    segs = "".join(
        f'<span class="seg {"done" if i < cur_step else "now" if i == cur_step else ""}" style="--c:{COLORS[i]}"></span>'
        for i in range(5)
    )
    name, pages = STEPS[cur_step]
    dots = "".join(f'<i class="{"on" if j <= cur_page else ""}"></i>' for j in range(len(pages)))
    return (
        f'<div class="progress" aria-label="Step {cur_step + 1} of 5, {e(name)}, page {cur_page + 1} of {len(pages)}">'
        f'<div class="segs" aria-hidden="true">{segs}</div>'
        f'<p><b>Step {cur_step + 1} of 5 · {e(name)}</b><span class="dots" aria-hidden="true">{dots}</span>'
        f'<span>page {cur_page + 1} of {len(pages)}</span></p></div>'
    )


def next_card(title, purpose, step_label, prev):
    return f"""<nav class="next" aria-label="Next step">
<a class="next-card" href="#"><span class="kicker">Next step · {e(step_label)}</span><span class="next-title">{e(title)}</span><span class="next-purpose">{e(purpose)}</span><span class="arrow" aria-hidden="true"></span></a>
<a class="prev" href="#">← {e(prev)}</a>
</nav>"""


JIRA = f"""
<article class="page" id="jira" data-view>
{progress(2, 1)}
<h1>Start from a Jira ticket</h1>
<p class="skip-step"><a href="#review">No Jira? Skip to Reviewing a plan</a></p>
<p class="lede">Your team already writes the work down in Jira. Name the ticket in your request and Claude starts the plan from it — no copy and paste, and the plan says where it comes from.</p>
<ol class="three">
<li><b>You say</b> “Prepare a plan for PROJ-123”.</li>
<li><b>Claude reads the ticket</b> once, without changing anything in Jira.</li>
<li><b>You get a plan that cites it</b> on its first line.</li>
</ol>
{figure("docs/en/src/guides/mcp.md")}
<div class="example"><p class="ex-label">Example</p>
<div class="chat"><p class="you">Prepare a plan for PROJ-123</p>
<p class="claude">I read <b>PROJ-123 — Dark mode for the dashboard</b> (In Progress). The plan is in <code>_compagnon/changes/dark-mode/</code>: why, what changes, how, and 7 tasks. Nothing is built until you approve it.</p></div>
<pre class="file"><span class="file-name">proposal.md</span><code># Proposal: Dark mode for the dashboard

&gt; Source: ticket **PROJ-123** — "Dark mode for the dashboard" (In Progress)</code></pre></div>
<details class="hood"><summary>Under the hood: set it up</summary><p>Compagnon needs the name of your Jira connector's ticket tool in <code>_compagnon/config.yaml</code>; <code>compagnon init</code> fills it in when it finds a Jira or Atlassian server in your Claude Code setup.</p></details>
<details class="hood"><summary>Under the hood: what Claude may do with Jira</summary><p>One read of the first ticket named. Never a search, a comment or a status change.</p></details>
{next_card("Reviewing a plan", "What to check in a plan before you say yes, file by file.", "page 3 of 5", "Asking Claude")}
</article>"""

FIRST = f"""
<article class="page" id="first" data-view hidden>
{progress(1, 1)}
<h1>Your first change</h1>
<p class="skip-step"><a href="#jira">Already made a change? Skip to Asking Claude</a></p>
<p class="lede">One whole change, in ten minutes: you ask, you read the plan, Claude builds it, and the change is filed away.</p>
{figure("docs/en/src/quickstart.md")}
<div class="example"><p class="ex-label">Example</p>
<div class="chat"><p class="you">Add a dark mode that follows the system setting</p>
<p class="claude">The plan is ready in <code>_compagnon/changes/dark-mode/</code>. Read <code>proposal.md</code> first — it says why and what changes.</p>
<p class="you">Implement it</p>
<p class="claude">✓ 7 of 7 tasks done · 2 review notes, nothing to decide.</p>
<p class="you">Archive it</p>
<p class="claude">Filed away in <code>_compagnon/changes/archive/2026-11-27-dark-mode/</code>; the project's specs now say the app follows the system theme.</p></div></div>
<details class="hood"><summary>Under the hood: the commands behind each request</summary><p><code>/compagnon-propose</code>, <code>/compagnon-apply</code>, <code>/compagnon-archive</code> — the same steps, typed as commands.</p></details>
{next_card("Asking Claude", "What to say to Claude at each step, in your own words.", "step 3 · Every day", "Installation")}
</article>"""

OVERVIEW_CARDS = "".join(
    f'<li class="ov" style="--c:{COLORS[i]}"><span class="ov-n">{i + 1}</span><h3>{e(n)}</h3><ul>'
    + "".join(f"<li>{e(p)}</li>" for p in pages)
    + "</ul></li>"
    for i, (n, pages) in enumerate(STEPS)
)
OVERVIEW = f"""
<article class="page" id="overview" data-view hidden>
<h1>The whole tunnel</h1>
<p class="lede">Five steps, read in order. Beginners can stop after step 3; experts go on.</p>
<ol class="ov-row">{OVERVIEW_CARDS}</ol>
<p class="appx-row">Appendices, looked up when needed: command-line interface · configuration · file formats · JSON output · error codes · glossary · FAQ · annotated example plan</p>
</article>"""


def dg_tokens():
    css = (ROOT / "docs/theme/mairistem.css").read_text()
    blocks = re.findall(r"(\.light,\nhtml:not\(\.js\)|\.navy) \{([^{}]*--dg-ink:[^{}]*)\}", css)
    light = [b for sel, b in blocks if sel.startswith(".light")][0]
    dark = [b for sel, b in blocks if sel == ".navy"][0]
    return light, dark


LIGHT, DARK = dg_tokens()

CSS = f"""
:root {{
  /* Layout: a book page in the approved Mairistem look — sticky top bar,
     a stepper sidebar, a progress band, the page, a next-step card. */
  --font: "IBM Plex Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Arial, sans-serif;
  --mono: "IBM Plex Mono", ui-monospace, "SF Mono", Menlo, Consolas, monospace;
  --bg: #ffffff; --fg: #2b3f47; --heading: #00263a; --muted: #4f6670; --surface: #f4f8f9; --line: #dce5e8;
  --link: #c4361b; --focus: #c4361b; --chip: #e7f0f3; --you: #00263a; --you-fg: #ffffff; --claude: #f4f8f9;
  --logo-ink: #000000; --logo-c: #004c66;
  {LIGHT}
  color-scheme: light;
}}
@media (prefers-color-scheme: dark) {{ :root:not([data-theme="light"]) {{
  --bg: #0b1c24; --fg: #d5e2e7; --heading: #ffffff; --muted: #a3b8c0; --surface: #10262f; --line: #22404c;
  --link: #ff8a70; --focus: #ffab03; --chip: #16343f; --you: #ffab03; --you-fg: #1a1a1a; --claude: #12303d;
  --logo-ink: #ffffff; --logo-c: #ffffff;
  {DARK}
  --dg-bg: #0b1c24; color-scheme: dark; }} }}
:root[data-theme="dark"] {{
  --bg: #0b1c24; --fg: #d5e2e7; --heading: #ffffff; --muted: #a3b8c0; --surface: #10262f; --line: #22404c;
  --link: #ff8a70; --focus: #ffab03; --chip: #16343f; --you: #ffab03; --you-fg: #1a1a1a; --claude: #12303d;
  --logo-ink: #ffffff; --logo-c: #ffffff;
  {DARK}
  --dg-bg: #0b1c24; color-scheme: dark; }}
* {{ box-sizing: border-box; }}
[hidden] {{ display: none !important; }}
body {{ margin: 0; background: var(--bg); color: var(--fg); font: 400 16px/1.65 var(--font); -webkit-font-smoothing: antialiased; }}
a {{ color: var(--link); text-underline-offset: 3px; }}
:focus-visible {{ outline: 3px solid var(--focus); outline-offset: 2px; border-radius: 8px; }}
code {{ font-family: var(--mono); font-size: .86em; background: var(--chip); padding: .1em .35em; border-radius: 5px; }}
body::before {{ content: ""; position: fixed; inset: 0 0 auto; height: env(safe-area-inset-top, 0px); z-index: 20; background: var(--bg); }}
.top {{ position: sticky; top: env(safe-area-inset-top, 0px); z-index: 10; display: flex; align-items: center; gap: 16px; padding: 12px max(16px, env(safe-area-inset-right, 0px)) 12px max(16px, env(safe-area-inset-left, 0px));
  background: color-mix(in srgb, var(--bg) 88%, transparent); backdrop-filter: blur(10px); border-bottom: 1px solid var(--line); }}
.top svg {{ height: 30px; width: auto; aspect-ratio: 1188/250; }}
.search {{ margin-left: auto; display: flex; align-items: center; gap: 10px; min-width: 0; color: var(--muted); border: 1px solid var(--line); border-radius: 10px; padding: 6px 10px; font-size: 14px; }}
.search kbd {{ font: 500 11px/1 var(--mono); border: 1px solid var(--line); border-radius: 5px; padding: 3px 6px; }}
.views {{ display: flex; gap: 6px; flex-wrap: wrap; }}
.views button {{ font: 500 12px/1 var(--mono); color: var(--heading); background: var(--surface); border: 1px solid var(--line); border-radius: 999px; padding: 7px 11px; cursor: pointer; }}
.views button[aria-pressed="true"] {{ background: var(--heading); color: var(--bg); border-color: var(--heading); }}
.shell {{ display: grid; grid-template-columns: 280px minmax(0, 1fr); max-width: 1240px; margin: 0 auto; }}
aside {{ border-right: 1px solid var(--line); padding: 28px 20px; position: sticky; top: 56px; align-self: start; height: calc(100vh - 56px); overflow: auto; }}
.stepper {{ list-style: none; margin: 0; padding: 0; display: grid; gap: 4px; position: relative; }}
.st {{ position: relative; display: grid; grid-template-columns: 30px 1fr; gap: 0 12px; padding: 6px 0; }}
.st:not(:last-child)::after {{ content: ""; position: absolute; left: 14px; top: 38px; bottom: -6px; width: 2px; background: var(--line); }}
.st.done:not(:last-child)::after {{ background: var(--c); }}
.dot {{ width: 30px; height: 30px; border-radius: 50%; display: grid; place-items: center; font: 600 13px/1 var(--mono); border: 2px solid var(--line); color: var(--muted); background: var(--bg); z-index: 1; }}
.st.done .dot {{ background: var(--c); border-color: var(--c); color: #1a0a05; }}
.st.current .dot {{ border-color: var(--c); color: var(--heading); box-shadow: 0 0 0 4px color-mix(in srgb, var(--c) 25%, transparent); }}
.st-name {{ font-weight: 600; color: var(--heading); line-height: 30px; display: flex; flex-direction: column; }}
.st-name small {{ font: 500 11px/1.2 var(--mono); color: var(--muted); font-weight: 400; }}
.st.todo .st-name {{ color: var(--muted); }}
.pages {{ grid-column: 2; list-style: none; margin: 8px 0 4px; padding: 0; display: grid; gap: 2px; }}
.pages a {{ display: block; padding: 5px 10px; border-radius: 8px; text-decoration: none; color: var(--fg); font-size: 14px; }}
.pages .read a {{ color: var(--muted); }}
.pages .here a {{ background: var(--surface); color: var(--heading); font-weight: 600; box-shadow: inset 3px 0 0 var(--c, var(--link)); }}
.appx {{ margin: 22px 0 0; font: 500 12px/1.5 var(--mono); color: var(--muted); border-top: 1px solid var(--line); padding-top: 14px; }}
main {{ min-width: 0; padding: 28px max(16px, env(safe-area-inset-right, 0px)) 80px max(16px, env(safe-area-inset-left, 0px)); }}
.page {{ max-width: 760px; margin: 0 auto; display: grid; gap: 20px; overflow-wrap: anywhere; }}
.progress {{ display: grid; gap: 8px; }}
.segs {{ display: grid; grid-template-columns: repeat(5, 1fr); gap: 6px; }}
.seg {{ height: 6px; border-radius: 99px; background: var(--line); }}
.seg.done {{ background: var(--c); }}
.seg.now {{ background: linear-gradient(90deg, var(--c) 40%, var(--line) 40%); }}
.progress p {{ margin: 0; display: flex; flex-wrap: wrap; align-items: center; gap: 10px; font: 500 12px/1.2 var(--mono); color: var(--muted); text-transform: uppercase; letter-spacing: .05em; }}
.progress b {{ color: var(--heading); font-weight: 600; }}
.dots {{ display: inline-flex; gap: 4px; }}
.dots i {{ width: 7px; height: 7px; border-radius: 50%; background: var(--line); }}
.dots i.on {{ background: var(--heading); }}
h1 {{ margin: 4px 0 0; font-size: clamp(2rem, 4vw, 2.6rem); line-height: 1.1; letter-spacing: -.02em; color: var(--heading); text-wrap: balance; }}
.lede {{ margin: 0; font-size: 1.15rem; color: var(--fg); }}
.skip-step {{ margin: 0; }}
.skip-step a {{ display: inline-flex; align-items: center; gap: 10px; padding: 8px 16px; border-radius: 999px; background: var(--surface); border: 1px solid var(--line); color: var(--heading); text-decoration: none; font-weight: 600; font-size: 14px; }}
.skip-step a::after {{ content: ""; width: 7px; height: 7px; border-top: 2px solid var(--link); border-right: 2px solid var(--link); transform: rotate(45deg); }}
.skip-step a:hover {{ border-color: var(--link); }}
.skip-step a:focus-visible {{ outline: 3px solid var(--focus); background: var(--bg); }}
.three {{ margin: 0; padding-left: 1.3em; display: grid; gap: 6px; }}
.three b {{ color: var(--heading); }}
figure.diagram {{ margin: 0; display: grid; gap: 10px; min-width: 0; }}
figure.diagram > .scroll {{ overflow-x: auto; border: 1px solid var(--line); border-radius: 16px; background: var(--dg-bg); padding: 14px; }}
figure.diagram svg {{ display: block; width: 100%; height: auto; min-width: 560px; }}
figure.diagram figcaption {{ color: var(--muted); font-size: .92rem; }}
figure.diagram text {{ font-family: var(--font); }}
figure.diagram .mono {{ font-family: var(--mono); }}
.example {{ border-radius: 16px; background: var(--surface); border: 1px solid var(--line); padding: 16px 18px; display: grid; gap: 12px; }}
.ex-label {{ margin: 0; font: 600 11px/1 var(--mono); letter-spacing: .08em; text-transform: uppercase; color: var(--muted); display: flex; gap: 8px; align-items: center; }}
.ex-label::before {{ content: ""; width: 8px; height: 8px; border-radius: 50%; background: #00c39b; }}
.chat {{ display: grid; gap: 8px; }}
.chat p {{ margin: 0; padding: 10px 14px; border-radius: 14px; max-width: 88%; }}
.chat .you {{ justify-self: end; background: var(--you); color: var(--you-fg); border-bottom-right-radius: 4px; font-weight: 500; }}
.chat .claude {{ justify-self: start; background: var(--bg); border: 1px solid var(--line); border-bottom-left-radius: 4px; }}
.file {{ margin: 0; border-radius: 12px; border: 1px solid var(--line); background: var(--bg); overflow: hidden; font: 13px/1.6 var(--mono); }}
.file-name {{ display: block; padding: 7px 12px; border-bottom: 1px solid var(--line); color: var(--muted); font-size: 12px; }}
.file code {{ display: block; padding: 10px 12px; background: none; white-space: pre-wrap; }}
.hood {{ border: 1px solid var(--line); border-radius: 12px; padding: 0 14px; }}
.hood summary {{ cursor: pointer; list-style: none; padding: 12px 0; font-weight: 600; color: var(--heading); display: flex; gap: 10px; align-items: center; }}
.hood summary::-webkit-details-marker {{ display: none; }}
.hood summary::before {{ content: ""; width: 7px; height: 7px; border-right: 2px solid var(--muted); border-bottom: 2px solid var(--muted); transform: rotate(-45deg); transition: transform .2s; }}
.hood[open] summary::before {{ transform: rotate(45deg); }}
.hood p {{ margin: 0 0 14px; color: var(--fg); }}
.next {{ display: grid; gap: 12px; margin-top: 12px; }}
.next-card {{ position: relative; display: grid; gap: 4px; padding: 20px 56px 20px 22px; border-radius: 18px; text-decoration: none; color: var(--fg);
  background: linear-gradient(var(--bg), var(--bg)) padding-box, linear-gradient(120deg, #ff5331, #ffab03, #00c39b) border-box; border: 2px solid transparent; }}
.kicker {{ font: 600 11px/1 var(--mono); letter-spacing: .08em; text-transform: uppercase; color: var(--muted); }}
.next-title {{ font-size: 1.35rem; font-weight: 700; color: var(--heading); }}
.next-purpose {{ color: var(--muted); }}
.arrow {{ position: absolute; right: 22px; top: 50%; width: 14px; height: 14px; border-top: 3px solid var(--link); border-right: 3px solid var(--link); transform: translateY(-50%) rotate(45deg); }}
.prev {{ font-size: 14px; color: var(--muted); }}
.ov-row {{ list-style: none; margin: 0; padding: 0; display: grid; grid-template-columns: repeat(5, minmax(0, 1fr)); gap: 12px; }}
.ov {{ position: relative; border: 1px solid var(--line); border-radius: 16px; padding: 16px; display: grid; gap: 8px; align-content: start; }}
.ov::before {{ content: ""; position: absolute; left: 16px; right: 16px; top: -1px; height: 3px; border-radius: 0 0 3px 3px; background: var(--c); }}
.ov-n {{ width: 28px; height: 28px; border-radius: 50%; display: grid; place-items: center; background: var(--c); color: #1a0a05; font: 700 13px/1 var(--mono); }}
.ov h3 {{ margin: 0; font-size: 1rem; color: var(--heading); }}
.ov ul {{ margin: 0; padding-left: 1em; font-size: 13.5px; color: var(--muted); display: grid; gap: 3px; }}
.appx-row {{ margin: 0; font: 500 12.5px/1.6 var(--mono); color: var(--muted); border: 1px dashed var(--line); border-radius: 12px; padding: 10px 14px; }}
@media (prefers-reduced-motion: no-preference) {{
  .next-card {{ transition: transform .2s ease, box-shadow .2s ease; }}
  .next-card:hover {{ transform: translateY(-2px); box-shadow: 0 16px 34px -22px rgba(0, 38, 58, .55); }}
}}
@media (max-width: 980px) {{ .ov-row {{ grid-template-columns: minmax(0, 1fr); }} }}
@media (max-width: 860px) {{ .shell {{ grid-template-columns: minmax(0, 1fr); }} aside {{ display: none; }} .search span {{ display: none; }} }}
"""

SCRIPT = """
(function () {
  var root = document.documentElement;
  function show(v) {
    document.querySelectorAll('[data-view]').forEach(function (p) { p.hidden = p.id !== v; });
    document.querySelectorAll('[data-show]').forEach(function (b) { b.setAttribute('aria-pressed', String(b.dataset.show === v)); });
  }
  document.querySelectorAll('[data-show]').forEach(function (b) { b.addEventListener('click', function () { show(b.dataset.show); window.scrollTo(0, 0); }); });
  document.querySelectorAll('.skip-step a').forEach(function (a) { a.addEventListener('click', function (ev) { ev.preventDefault(); show(a.getAttribute('href') === '#review' ? 'overview' : 'jira'); }); });
  document.querySelectorAll('[data-theme-set]').forEach(function (b) { b.addEventListener('click', function () { root.setAttribute('data-theme', b.dataset.themeSet); }); });
})();
"""


def page():
    return f"""<title>Compagnon Reading Tunnel</title>
<meta name="claude:viewport-fit-sides" content="cover">
<link rel="preconnect" href="https://fonts.googleapis.com"><link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500;600&family=IBM+Plex+Sans:wght@400;500;600;700&display=swap">
<style>{CSS}</style>
{LOGO}
<header class="top">
<svg viewBox="0 0 1188 250" role="img" aria-label="Compagnon by Mairistem"><use href="#cp-logo"></use></svg>
<div class="views" role="group" aria-label="Mockup views">
<button type="button" id="v-jira" data-show="jira" aria-pressed="true">A page: Jira</button>
<button type="button" id="v-first" data-show="first" aria-pressed="false">A page: first change</button>
<button type="button" id="v-ov" data-show="overview" aria-pressed="false">The whole tunnel</button>
<button type="button" id="t-l" data-theme-set="light">Light</button>
<button type="button" id="t-d" data-theme-set="dark">Dark</button>
</div>
<div class="search" aria-hidden="true"><span>Search the docs</span><kbd>/</kbd></div>
</header>
<div class="shell">
<aside aria-label="The five steps">{sidebar(2, 1)}</aside>
<main>
{JIRA}
{FIRST}
{OVERVIEW}
</main>
</div>
<script>{SCRIPT}</script>
"""


v1 = HERE / "index.html"
if v1.exists() and "stepper" not in v1.read_text():
    v1.rename(HERE / "index-v1.html")
(HERE / "index.html").write_text(page())
print("wrote", HERE / "index.html")
