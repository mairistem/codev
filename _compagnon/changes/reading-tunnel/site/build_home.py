"""Build docs/site/index.html and docs/site/fr.html from the approved mockup
(reading-tunnel, version 3: the Mairistem design system).

Run from the repository root:
    python3 _compagnon/changes/reading-tunnel/site/build_home.py

It keeps from the current pages what is theirs (the head up to the styles,
the sprites, the copy script) and takes the diagrams from the chapters that
own them, so the home pages draw exactly what the books draw.
"""
import pathlib
import re

import html

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SITE = ROOT / "docs/site"
e = html.escape


def symbol(name):
    """A Mairistem symbol, drawn as a watermark (decorative)."""
    svg = (HERE.parent / "mockup" / name).read_text()
    svg = re.sub(r"<\?xml[^>]*\?>", "", svg).replace('id="Calque_2" ', "").replace(' id="Calque_1-2"', "")
    return svg.replace("<svg ", '<svg class="filigrane" aria-hidden="true" focusable="false" ', 1).strip()


def cc(label, caption, lines, hint):
    """A Claude Code session as the terminal shows it, framed as the design
    system frames an application capture."""
    out = []
    for line in lines:
        kind = line[0]
        if kind == "user":
            out.append(f'<div class="cc-user"><span aria-hidden="true">&gt;</span> {e(line[1])}</div>')
        elif kind == "say":
            out.append(f'<div class="cc-row"><span class="cc-dot" aria-hidden="true">⏺</span><span>{line[1]}</span></div>')
        else:
            res = "".join(f'<div class="cc-res"><span aria-hidden="true">⎿</span> {r}</div>' for r in line[3])
            out.append(f'<div class="cc-row"><span class="cc-dot ok" aria-hidden="true">⏺</span><span><b>{e(line[1])}</b>({e(line[2])})</span></div>{res}')
    return f"""<figure class="capture" aria-label="{e(label)}">
<div class="cc"><div class="cc-body">{"".join(out)}</div>
<div class="cc-input" aria-hidden="true"><div class="box"><span>&gt;</span> <i>{e(hint)}</i></div></div>
<div class="cc-foot" aria-hidden="true">? for shortcuts</div></div>
<figcaption>{caption}</figcaption></figure>"""


def figure(path, n=0):
    md = (ROOT / path).read_text()
    blocks = re.findall(r'<figure class="diagram">.*?</figure>', md, re.S)
    return blocks[n]


def head_of(page):
    return page[: page.index(":root {")]


def sprites_of(page):
    body = page.index("<body>") + len("<body>")
    skip = page.index('<a class="skip"')
    return page[body:skip].strip()


def script_of(page):
    # The copy button sits in the code block's header, beside the command:
    # look the command up from the block, and say so when it is missing.
    script = page[page.index("<script>") :]
    return script.replace(
        "var text = button.parentNode.querySelector('pre').textContent.trim();",
        "var block = button.closest('.codeblock');\n      var pre = block && block.querySelector('pre');\n      var text = pre ? pre.textContent.trim() : '';",
    ).replace(
        "      copy(text).then(function () {",
        "      (text ? copy(text) : Promise.reject()).then(function () {",
    )


CSS = """:root {
  /* Mairistem design system: brand colors (coral, deep blue, green,
     yellow), black, white and the system's greys; IBM Plex Sans; no
     gradient, no blur. Dark mode uses the system's dark surfaces. */
  --font: "IBM Plex Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Arial, sans-serif;
  --mono: ui-monospace, "SF Mono", SFMono-Regular, Menlo, Consolas, "Liberation Mono", monospace;
  --corail: #ff5331;
  --bleu: #004c66;
  --vert: #00c39b;
  --vert-ink: #00806a;
  --jaune: #ffab03;
  --page: #ffffff;
  --soft: #f4f8f9;
  --card: #ffffff;
  --strong: #004c66;
  --body: #40565e;
  --muted: #5f767e;
  --eyebrow: #6e858d;
  --eyebrow-corail: #c63a1c;
  --border: #e3eaec;
  --hairline: #e7edef;
  --link: #c4361b;
  --link-hover: #a82d15;
  --focus: #ff5331;
  --deep: #00394d;
  --dark-card: #00394d;
  --dark-card-border: transparent;
  --filigrane: .3;
  --code-bg: #f4f8f9;
  --code-fg: #00394d;
  --cp-logo-ink: #000000;
  --cp-logo-c: #004c66;
  color-scheme: light;
}

/* Diagram tokens (checked by contrast_problems in docs_checks.rs). */
:root {
  --dg-bg: #ffffff;
  --dg-box: #f4f8f9;
  --dg-file: #ffffff;
  --dg-ink: #00263a;
  --dg-muted: #4f6670;
  --dg-line: #567079;
  --dg-you: #e0421f;
  --dg-you-text: #c4361b;
  --dg-on-you: #1a0a05;
  --dg-claude: #004c66;
}

@media (prefers-color-scheme: dark) {
  :root {
    --page: #00394d;
    --soft: #004c66;
    --card: #004c66;
    --strong: #ffffff;
    --body: #e7edef;
    --muted: #9cbac4;
    --eyebrow: #9cbac4;
    --eyebrow-corail: #ffab03;
    --border: rgba(255, 255, 255, .14);
    --hairline: rgba(255, 255, 255, .14);
    --link: #ffab03;
    --link-hover: #ffc24d;
    --deep: #002a3a;
    --dark-card: #004c66;
    --dark-card-border: rgba(255, 255, 255, .14);
    --filigrane: .18;
    --code-bg: #002a3a;
    --code-fg: #e7edef;
    --cp-logo-ink: #ffffff;
    --cp-logo-c: #ffffff;
    color-scheme: dark;
  }
}

@media (prefers-color-scheme: dark) {
  :root {
    --dg-bg: #00394d;
    --dg-box: #0b4a61;
    --dg-file: #00394d;
    --dg-ink: #f2f7f8;
    --dg-muted: #b4c8cf;
    --dg-line: #a7c6d1;
    --dg-you: #ff8a70;
    --dg-you-text: #ffa390;
    --dg-on-you: #1a0a05;
    --dg-claude: #7cc8e2;
  }
}

* { box-sizing: border-box; }
body { margin: 0; background: var(--page); color: var(--body); font: 400 15.5px/1.5 var(--font); -webkit-font-smoothing: antialiased; }
h1, h2, h3 { color: var(--strong); margin: 0; text-wrap: balance; }
p { margin: 0; }
a { color: var(--link); text-decoration: none; text-underline-offset: 3px; }
a:hover { color: var(--link-hover); text-decoration: underline; }
code { font-family: var(--mono); font-size: .9em; background: var(--soft); border: 1px solid var(--hairline); color: var(--code-fg); padding: 0 .3em; border-radius: 4px; }
:focus-visible { outline: 3px solid var(--focus); outline-offset: 2px; border-radius: 2px; }
.visually-hidden { position: absolute; width: 1px; height: 1px; overflow: hidden; clip: rect(0 0 0 0); white-space: nowrap; }
.skip { position: absolute; left: 16px; top: -48px; z-index: 50; background: var(--strong); color: var(--page); padding: 10px 14px; border-radius: 6px; font-weight: 600; }
.skip:focus { top: 12px; }
.container { max-width: 1240px; margin: 0 auto; padding-left: max(16px, env(safe-area-inset-left, 0px)); padding-right: max(16px, env(safe-area-inset-right, 0px)); }

/* Eyebrow: 600 13px, tracking 1.17px, upper case */
.eyebrow { display: flex; align-items: center; gap: 10px; font: 600 13px/1 var(--font); letter-spacing: 1.17px; text-transform: uppercase; color: var(--eyebrow); }
.eyebrow.corail { color: var(--eyebrow-corail); letter-spacing: 1.04px; }
.eyebrow .sq { width: 10px; height: 10px; border-radius: 2px; background: var(--corail); flex: none; }

/* Header: a dark top band, then a white band; not sticky */
.topband { background: var(--deep); color: #b9cdd3; font-size: 13.5px; }
.topband .container { min-height: 44px; display: flex; align-items: center; justify-content: space-between; gap: 16px; flex-wrap: wrap; }
.topband b { color: #ffffff; font-weight: 600; }
.topband nav { display: flex; gap: 16px; align-items: center; }
.topband a { color: #ffffff; }
.topband .pill { padding: 6px 14px; background: var(--corail); border-radius: 4px; font-weight: 600; color: #1a0a05; }
.topband .pill:hover { color: #1a0a05; text-decoration: none; }
.mainband { background: var(--page); border-bottom: 1px solid var(--border); }
.mainband .container { min-height: 86px; display: flex; align-items: center; justify-content: space-between; gap: 20px; flex-wrap: wrap; padding-block: 10px; }
.brand { display: inline-flex; line-height: 0; border-radius: 6px; }
.cp-logo { display: block; height: 44px; width: auto; aspect-ratio: 1188 / 250; max-width: 100%; }
.site-nav { display: flex; gap: 4px; flex-wrap: wrap; }
.site-nav a { padding: 12px 11px; font-weight: 600; color: var(--strong); border-radius: 6px; }
.site-nav a:hover { background: var(--soft); text-decoration: none; }

/* Buttons: radius 6, primary coral */
.btns { display: flex; flex-wrap: wrap; gap: 14px; }
.btn { font: 600 16px/1 var(--font); border-radius: 6px; padding: 15px 26px; display: inline-flex; align-items: center; border: 1.5px solid transparent; }
.btn:hover { text-decoration: none; }
.btn.primary { background: var(--corail); color: #1a0a05; box-shadow: 0 6px 18px 0 rgba(255, 83, 49, .28); }
.btn.primary:hover { background: #e8431f; color: #1a0a05; }
.btn.secondary { color: var(--strong); border-color: var(--strong); }
.btn.secondary:hover { background: var(--strong); color: var(--page); }
.btn.sm { font-size: 15px; padding: 13px 22px; }
.link-arrow { font-weight: 600; display: inline-flex; gap: 6px; align-items: center; }

/* Product hero: soft surface, the symbol as a watermark */
.hero { position: relative; overflow: hidden; background: var(--soft); padding-block: 56px 64px; }
.filigrane { position: absolute; right: -30px; bottom: -60px; width: 300px; height: auto; opacity: var(--filigrane); pointer-events: none; }
.hero .container { position: relative; display: grid; grid-template-columns: minmax(0, 1fr) minmax(0, 1fr); gap: 56px; align-items: center; }
.hero-text { display: grid; gap: 18px; }
.hero h1 { font-size: clamp(34px, 5vw, 52px); line-height: 1.06; font-weight: 600; letter-spacing: -1.3px; }
.lead { font-size: 16px; line-height: 26px; color: var(--muted); max-width: 560px; }
.proofs { list-style: none; margin: 0; padding: 0; display: flex; flex-wrap: wrap; gap: 8px 18px; font-size: 14.5px; color: var(--muted); }
.proofs li::before { content: "✓"; color: var(--vert-ink); font-weight: 700; margin-right: 8px; }

/* A Claude Code capture, framed white with a 1px border, radius 12 */
.capture { margin: 0; display: grid; gap: 10px; min-width: 0; }
.capture .cc { background: #ffffff; border: 1px solid #e3eaec; border-radius: 12px; padding: 8px; }
.cc-body, .cc-input, .cc-foot { background: #1e1e1e; color: #e6e6e6; font: 13px/1.6 var(--mono); }
.cc-body { border-radius: 6px 6px 0 0; padding: 14px 16px 6px; display: grid; gap: 8px; overflow-x: auto; }
.cc-user { color: #b0b0b0; background: #2b2b2b; border-radius: 4px; padding: 4px 8px; }
.cc-row { display: grid; grid-template-columns: 16px 1fr; gap: 6px; }
.cc-dot.ok { color: #4eba65; }
.cc-row b { color: #ffffff; }
.cc-res { padding-left: 22px; color: #a8a8a8; }
.cc code { background: none; border: 0; padding: 0; color: #b1b9f9; font-size: 1em; }
.cc-input { padding: 8px 12px 0; }
.cc-input .box { border: 1px solid #5a5a5a; border-radius: 6px; padding: 6px 10px; }
.cc-input i { font-style: normal; color: #8a8a8a; }
.cc-foot { border-radius: 0 0 6px 6px; padding: 6px 14px 10px; color: #8a8a8a; font-size: 11.5px; }
.capture figcaption, figure.diagram figcaption { color: var(--muted); font-size: 13.5px; line-height: 20px; }

/* Stat bar: a deep blue band */
.statbar { background: var(--bleu); color: #ffffff; }
.statbar ul { list-style: none; margin: 0 auto; padding: 0; display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); }
.statbar li { padding: 26px 32px; border-left: 1px solid rgba(255, 255, 255, .14); display: grid; gap: 4px; }
.statbar li:first-child { border-left: 0; }
.statbar b { font-size: 40px; line-height: 40px; font-weight: 400; letter-spacing: -.8px; }
.statbar span { color: #b9cdd3; font-size: 14.5px; }

/* Sections */
.section { padding-block: 76px 84px; }
.section-alt { background: var(--soft); }
.section-head { max-width: 46em; display: grid; gap: 14px; margin-bottom: 32px; }
.section-head h2 { font-size: clamp(28px, 3.6vw, 38px); line-height: 1.15; font-weight: 600; letter-spacing: -.76px; }
.section-head p { color: var(--muted); font-size: 16px; line-height: 26px; }
.more { margin-top: 22px; display: flex; flex-wrap: wrap; gap: 8px 22px; font-weight: 600; }

/* The five steps: cards with a 5px colored top rule, no shadow */
.steps { list-style: none; margin: 0; padding: 0; display: grid; grid-template-columns: repeat(5, minmax(0, 1fr)); gap: 16px; }
.step { position: relative; border: 1px solid var(--border); border-radius: 12px; overflow: hidden; background: var(--card); display: flex; flex-direction: column; }
.step .rule { height: 5px; background: var(--c); }
.step > div { padding: 24px 22px 22px; display: flex; flex-direction: column; gap: 12px; flex: 1; }
.step h3 { font-size: 16.5px; line-height: 22px; font-weight: 600; }
.step h3 a { color: var(--strong); }
.step h3 a::after { content: ""; position: absolute; inset: 0; border-radius: 12px; }
.step h3 a:focus-visible { outline: none; }
.step:focus-within { outline: 3px solid var(--focus); outline-offset: 2px; }
.step p:not(.eyebrow) { font-size: 14.5px; line-height: 22px; color: var(--muted); flex: 1; }
.step .go { font-weight: 600; font-size: 15px; color: var(--link); }
.step:hover { border-color: var(--c); }

/* Ticket band */
.jira { display: grid; grid-template-columns: minmax(0, .8fr) minmax(0, 1.2fr); gap: 40px; align-items: center; }
.jira .say { display: grid; gap: 14px; align-content: start; }
.say-quote { font: 500 16px/1.5 var(--mono); color: var(--strong); background: var(--card); border: 1px solid var(--border); border-left: 4px solid var(--corail); border-radius: 6px; padding: 14px 16px; }
.checks { list-style: none; margin: 0; padding: 0; display: grid; gap: 10px; }
.checks li { padding-left: 28px; position: relative; }
.checks li::before { content: "✓"; position: absolute; left: 0; color: var(--vert-ink); font-weight: 700; }

/* Diagrams */
figure.diagram { margin: 0; display: grid; gap: 10px; min-width: 0; }
figure.diagram > .scroll { overflow-x: auto; border: 1px solid var(--border); border-radius: 12px; background: var(--dg-bg); padding: 14px; }
figure.diagram svg { display: block; width: 100%; height: auto; min-width: 560px; }
figure.diagram text { font-family: var(--font); }
figure.diagram .mono { font-family: var(--mono); }

/* Accordions: hairlines, + closed, − open */
.accs { border-top: 1px solid var(--hairline); margin-top: 18px; }
.acc { border-bottom: 1px solid var(--hairline); }
.acc summary { list-style: none; cursor: pointer; display: flex; align-items: center; gap: 12px; padding: 18px 0; font-size: 17px; font-weight: 600; color: var(--strong); }
.acc summary::-webkit-details-marker { display: none; }
.acc summary::after { content: "+"; margin-left: auto; font-size: 22px; font-weight: 400; color: var(--muted); }
.acc[open] summary::after { content: "−"; color: var(--corail); }
.acc p { margin: 0 0 12px; max-width: 60em; color: var(--muted); }
.acc a { display: inline-block; margin-bottom: 18px; font-weight: 600; }
.n { font: 600 12px/24px var(--font); width: 24px; height: 24px; border-radius: 6px 6px 6px 0; text-align: center; flex: none; }
.n.you { background: var(--dg-you); color: var(--dg-on-you); }
.n.claude { background: var(--dg-claude); color: var(--dg-box); }
.n.kept { background: var(--dg-line); color: var(--dg-box); }

/* Install */
.install-grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 16px; }
.codeblock { border: 1px solid var(--border); border-radius: 12px; overflow: hidden; background: var(--code-bg); min-width: 0; }
.codeblock header { display: flex; justify-content: space-between; align-items: center; gap: 8px; padding: 8px 12px; border-bottom: 1px solid var(--border); background: var(--card); }
.codeblock h3 { font: 600 13px/1 var(--font); letter-spacing: 1.17px; text-transform: uppercase; color: var(--eyebrow); }
.codeblock pre { margin: 0; padding: 14px 16px; overflow-x: auto; font: 13.5px/1.6 var(--mono); color: var(--code-fg); }
.codeblock pre code { background: none; border: 0; padding: 0; font-size: inherit; }
.copy { font: 600 13px/1 var(--font); color: var(--strong); background: var(--card); border: 1.5px solid var(--strong); border-radius: 6px; padding: 7px 10px; cursor: pointer; }
.copy[data-state="done"] { border-color: var(--vert-ink); }

/* What you get */
.features { list-style: none; margin: 0; padding: 0; display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 16px; }
.feature { border: 1px solid var(--border); border-radius: 12px; padding: 22px 24px; display: grid; gap: 8px; align-content: start; background: var(--card); }
.feature h3 { font-size: 16.5px; font-weight: 600; }
.feature p { color: var(--muted); font-size: 14.5px; line-height: 22px; }

/* Footer */
.site-footer { background: #00394d; color: #b9cdd3; padding-block: 40px; font-size: 14.5px; }
.site-footer .container { display: flex; flex-wrap: wrap; gap: 24px; justify-content: space-between; align-items: center; }
.site-footer a { color: #ffffff; }
.footer-brand { display: flex; flex-wrap: wrap; gap: 16px; align-items: center; }
.site-footer .logo { height: 30px; width: auto; aspect-ratio: 1113 / 249; --mx-logo-ink: #fff; --mx-logo-corail: #fff; --mx-logo-jaune: #fff; --mx-logo-bleu: #fff; --mx-logo-vert: #fff; }
.footer-links { list-style: none; margin: 0; padding: 0; display: flex; flex-wrap: wrap; gap: 8px 20px; }

@media (prefers-reduced-motion: no-preference) {
  .btn, .step, .link-arrow { transition: transform .2s ease-in-out, border-color .2s ease-in-out, background-color .2s ease-in-out; }
  .btn:hover { transform: translateY(-1px); }
  .step:hover { transform: translateY(-3px); }
}
@media (max-width: 1100px) {
  .steps { grid-template-columns: repeat(2, minmax(0, 1fr)); }
  .hero .container { grid-template-columns: minmax(0, 1fr); gap: 32px; }
}
@media (max-width: 860px) {
  .site-nav { display: none; }
  .jira, .install-grid, .features { grid-template-columns: minmax(0, 1fr); }
  .statbar ul { grid-template-columns: minmax(0, 1fr); }
  .statbar li { border-left: 0; border-top: 1px solid rgba(255, 255, 255, .14); }
}
@media (max-width: 560px) {
  .steps { grid-template-columns: minmax(0, 1fr); }
  .cp-logo { height: 34px; }
  .section { padding-block: 52px; }
  .filigrane { width: 160px; }
}
</style>
</head>
"""

T = {
    "en": {
        "lang": "en",
        "skip": "Skip to content",
        "brand_href": "./",
        "brand_label": "Compagnon, by Mairistem — home",
        "nav_label": "Main",
        "docs": "en/",
        "other": '<a href="fr.html" hreflang="fr" lang="fr">Français</a>',
        "topband": '<b>An open project by Mairistem</b> · Groupe JVS',
        "install_btn": "Install",
        "eyebrow": "Open source · for Claude Code",
        "h1": "Plan with Claude before it codes.",
        "lead": "Before any code is written, Claude writes down what it is about to build: a short plan you read, correct and approve. Then Claude builds exactly that, and the plan stays with your code.",
        "read_docs": "Read the documentation",
        "github": "View on GitHub",
        "proofs": ["Nothing is built without your approval", "The plan stays with the code", "Your own words, no command to learn"],
        "session": cc("Claude Code session: a plan for a dark mode", "An example exchange in Claude Code: you ask, Claude writes the plan and waits for you.", [
            ("user", "Add a dark mode that follows the system setting"),
            ("say", "I will write a plan first; nothing is built before you approve it."),
            ("tool", "Write", "_compagnon/changes/dark-mode/proposal.md", ["Wrote 41 lines to _compagnon/changes/dark-mode/proposal.md"]),
            ("tool", "Write", "_compagnon/changes/dark-mode/tasks.md", ["Wrote 7 tasks, each with its test"]),
            ("say", "The plan is ready in <code>_compagnon/changes/dark-mode/</code>. Read <code>proposal.md</code> first, then tell me to implement it."),
        ], "The plan is good, implement it"),
        "stats_label": "In figures",
        "stats": [("5", "reading steps"), ("4", "short files per plan"), ("0", "lines of code before your approval")],
        "steps_eyebrow": "The documentation in five steps",
        "steps_title": "Read it in order, or start at your step.",
        "start_here": "Start here",
        "tunnel": [
            ("#ff5331", "Discover", "introduction.html", "1 page", "What Compagnon does, and who decides."),
            ("#00c39b", "Get started", "installation.html", "2 pages", "Install it, then a first change in ten minutes."),
            ("#ffab03", "Every day", "skills.html", "5 pages", "Ask, review, approve, build: the everyday loop."),
            ("#004c66", "Understand", "workflow.html", "5 pages", "What happens behind each step."),
            ("#8aa3ac", "For your team", "guides/existing-project.html", "6 pages", "Set Compagnon up for a team and its conventions."),
        ],
        "step_word": "Step",
        "origins": 'You want to change Compagnon itself? <a href="en/origins.html">Start with its origins <span aria-hidden="true">→</span></a>',
        "jira_eyebrow": "Jira",
        "jira_title": "Start from a Jira ticket",
        "jira_say": "“Prepare a plan for PROJ-123”",
        "jira_steps": [
            "You name the ticket in your request, in your own words.",
            "Claude reads it once — title, description, status — and changes nothing in Jira.",
            "The plan cites the ticket at the top, so everyone sees where it comes from.",
        ],
        "jira_link": '<a href="en/guides/mcp.html">Start from a Jira ticket <span aria-hidden="true">→</span></a>',
        "jira_figure": figure("docs/en/src/guides/mcp.md"),
        "how_eyebrow": "How it works",
        "how_title": "You decide at every step",
        "how_lead": "Claude proposes, you decide. Nothing is built before you approve the plan. Open a step to learn more.",
        "workflow_figure": figure("docs/en/src/workflow.md", 0),
        "steps_label": "The five steps",
        "steps": [
            ("you", "1", "You ask", "Type <code>/compagnon-propose</code> or simply ask in your own words: “add a dark mode that follows the system setting”.", "en/skills.html", "Asking Claude"),
            ("claude", "2", "Claude plans", "Claude studies the code, then writes four short files: why, what changes, how, and the tasks. Independent reviewers re-read it before you do.", "en/guides/example-plan.html", "An annotated example plan"),
            ("you", "3", "You approve", "Read the plan, ask for corrections, approve it — in the conversation or on GitHub. No terminal needed.", "en/guides/review-a-plan.html", "Reviewing a plan"),
            ("claude", "4", "Claude builds", "Claude implements the tasks in order, runs each test, and stops at the first doubt instead of guessing.", "en/guides/build-and-file.html", "Building and filing away"),
            ("kept", "5", "Filed away", "The plan joins the project's specifications: the next person knows why the code is the way it is.", "en/concepts.html", "Concepts"),
        ],
        "go_further": "Go further",
        "plan_eyebrow": "Inside a plan",
        "plan_title": "Four short files, from the why to the tasks",
        "plan_lead": "Read only the first to approve. Open the others when you want the detail.",
        "plan_figure": figure("docs/en/src/guides/review-a-plan.md"),
        "install_eyebrow": "Getting started",
        "install_title": "Install Compagnon",
        "install_lead": "No Rust toolchain needed. Both scripts verify the SHA-256 of the binary they download.",
        "unix": "macOS and Linux",
        "copy": "Copy",
        "then": 'Then, in your project: <code>compagnon init</code>',
        "install_links": '<a href="en/quickstart.html">Your first change <span aria-hidden="true">→</span></a><a href="en/installation.html">Other ways to install <span aria-hidden="true">→</span></a>',
        "why_eyebrow": "What you get",
        "why_title": "The plan before the code, decisions that hold",
        "features": [
            ("Plans challenged before your review", "Independent reviewers — a skeptic, and experts for the domains the change touches — re-read the plan and hand you at most five points."),
            ("Changes are deltas", "A change adds, modifies or removes requirements of the current specs, so Compagnon works on existing code, not only on new projects."),
            ("Decisions are sealed", "An architecture decision takes effect when you accept it; Compagnon seals its text, and changing it means replacing it."),
            ("Docs and tests in every plan", "Every scenario is covered by a test in a task, and the documentation the change affects is updated in a task."),
            ("Your team’s language", "Plans are written in the language your team sets; their structure stays machine-readable."),
            ("Measured, not assumed", "Every change to the skills is evaluated for quality and cost before it ships."),
        ],
        "why_links": '<a href="en/workflow.html">How it works <span aria-hidden="true">→</span></a><a href="en/roles.html">Roles <span aria-hidden="true">→</span></a><a href="en/reliability.html">Reliability of the roles <span aria-hidden="true">→</span></a><a href="https://github.com/mairistem/compagnon/tree/main/_compagnon">Compagnon is developed with Compagnon <span aria-hidden="true">→</span></a>',
        "footer_logo": "Mairistem by JVS",
        "footer_text": 'Compagnon — an open-source project by <a href="https://www.jvs-mairistem.fr/">Mairistem</a> · <a href="https://github.com/mairistem/compagnon/blob/main/LICENSE">MIT License</a>',
        "footer_links": '<li><a href="en/">Documentation</a></li><li><a href="fr.html" hreflang="fr" lang="fr">Accueil en français</a></li><li><a href="https://github.com/mairistem/compagnon/releases/latest">Releases</a></li><li><a href="https://github.com/mairistem/compagnon">GitHub</a></li>',
    },
    "fr": {
        "lang": "fr",
        "skip": "Aller au contenu",
        "brand_href": "fr.html",
        "brand_label": "Accueil du site Compagnon, par Mairistem",
        "nav_label": "Principal",
        "docs": "fr/",
        "other": '<a href="./" hreflang="en" lang="en">English</a>',
        "topband": '<b>Un projet libre de Mairistem</b> · Groupe JVS',
        "install_btn": "Installer",
        "eyebrow": "Libre · pour Claude Code",
        "h1": "Planifiez avec Claude avant qu’il ne code.",
        "lead": "Avant d’écrire la moindre ligne de code, Claude met par écrit ce qu’il s’apprête à construire : un plan court que vous lisez, corrigez et validez. Ensuite, Claude construit exactement cela, et le plan reste avec votre code.",
        "read_docs": "Lire la documentation",
        "github": "Voir sur GitHub",
        "proofs": ["Rien n’est construit sans votre accord", "Le plan reste avec le code", "Vos mots, aucune commande à apprendre"],
        "session": cc("Session Claude Code : un plan pour un mode sombre", "Un exemple d’échange dans Claude Code : vous demandez, Claude rédige le plan et attend votre accord.", [
            ("user", "Ajoute un mode sombre qui suit le réglage du système"),
            ("say", "J’écris d’abord un plan ; rien n’est construit avant votre accord."),
            ("tool", "Write", "_compagnon/changes/mode-sombre/proposal.md", ["Wrote 41 lines to _compagnon/changes/mode-sombre/proposal.md"]),
            ("tool", "Write", "_compagnon/changes/mode-sombre/tasks.md", ["Wrote 7 tasks, each with its test"]),
            ("say", "Le plan est prêt dans <code>_compagnon/changes/mode-sombre/</code>. Lisez d’abord <code>proposal.md</code>, puis dites-moi de le réaliser."),
        ], "Le plan me va, réalise-le"),
        "stats_label": "En chiffres",
        "stats": [("5", "étapes de lecture"), ("4", "fichiers courts par plan"), ("0", "ligne de code avant votre accord")],
        "steps_eyebrow": "La documentation en cinq étapes",
        "steps_title": "Lisez-la dans l’ordre, ou entrez à votre étape.",
        "start_here": "Commencer ici",
        "tunnel": [
            ("#ff5331", "Découvrir", "introduction.html", "1 page", "Ce que fait Compagnon, et qui décide."),
            ("#00c39b", "Démarrer", "installation.html", "2 pages", "L’installer, puis une première modification en dix minutes."),
            ("#ffab03", "Au quotidien", "skills.html", "5 pages", "Demander, relire, approuver, réaliser : la boucle de tous les jours."),
            ("#004c66", "Comprendre", "workflow.html", "5 pages", "Ce qui se passe derrière chaque étape."),
            ("#8aa3ac", "Pour l'équipe", "guides/existing-project.html", "6 pages", "Installer Compagnon pour une équipe et ses conventions."),
        ],
        "step_word": "Étape",
        "origins": 'Vous voulez modifier Compagnon lui-même ? <a href="fr/origins.html">Commencez par ses origines <span aria-hidden="true">→</span></a>',
        "jira_eyebrow": "Jira",
        "jira_title": "Partir d’un ticket Jira",
        "jira_say": "« Prépare un plan pour PROJ-123 »",
        "jira_steps": [
            "Vous nommez le ticket dans votre demande, avec vos mots.",
            "Claude le lit une fois — titre, description, statut — et ne change rien dans Jira.",
            "Le plan cite le ticket en tête : chacun voit d’où il vient.",
        ],
        "jira_link": '<a href="fr/guides/mcp.html">Partir d’un ticket Jira <span aria-hidden="true">→</span></a>',
        "jira_figure": figure("docs/fr/src/guides/mcp.md"),
        "how_eyebrow": "Comment ça marche",
        "how_title": "Vous décidez à chaque étape",
        "how_lead": "Claude propose, vous décidez. Rien n’est construit avant que vous ayez validé le plan. Ouvrez une étape pour en savoir plus.",
        "workflow_figure": figure("docs/fr/src/workflow.md", 0),
        "steps_label": "Les cinq étapes",
        "steps": [
            ("you", "1", "Vous demandez", "Tapez <code>/compagnon-propose</code> ou demandez simplement avec vos mots : « ajoute un mode sombre qui suit le réglage du système ».", "fr/skills.html", "Parler à Claude"),
            ("claude", "2", "Claude planifie", "Claude étudie le code, puis écrit quatre fichiers courts : pourquoi, ce qui change, comment, et les tâches. Des relecteurs indépendants le relisent avant vous.", "fr/guides/example-plan.html", "Un exemple de plan commenté"),
            ("you", "3", "Vous validez", "Lisez le plan, demandez des corrections, validez-le — dans la conversation ou sur GitHub. Sans terminal.", "fr/guides/review-a-plan.html", "Relire un plan"),
            ("claude", "4", "Claude construit", "Claude réalise les tâches dans l’ordre, lance chaque test et s’arrête au premier doute au lieu de deviner.", "fr/guides/build-and-file.html", "Réaliser et ranger"),
            ("kept", "5", "C’est rangé", "Le plan rejoint les spécifications du projet : la personne suivante sait pourquoi le code est ainsi.", "fr/concepts.html", "Concepts"),
        ],
        "go_further": "Pour aller plus loin",
        "plan_eyebrow": "Dans un plan",
        "plan_title": "Quatre fichiers courts, du pourquoi aux tâches",
        "plan_lead": "Lisez seulement le premier pour valider. Ouvrez les autres quand vous voulez le détail.",
        "plan_figure": figure("docs/fr/src/guides/review-a-plan.md"),
        "install_eyebrow": "Démarrer",
        "install_title": "Installer Compagnon",
        "install_lead": "Aucun outil de développement à installer. Les deux scripts vérifient l’empreinte SHA-256 du programme qu’ils téléchargent.",
        "unix": "macOS et Linux",
        "copy": "Copier",
        "then": 'Puis, dans votre projet : <code>compagnon init</code>',
        "install_links": '<a href="fr/quickstart.html">Votre première modification <span aria-hidden="true">→</span></a><a href="fr/installation.html">Autres façons d’installer <span aria-hidden="true">→</span></a>',
        "why_eyebrow": "Ce que vous y gagnez",
        "why_title": "Le plan avant le code, des décisions qui tiennent",
        "features": [
            ("Des plans contestés avant votre relecture", "Des relecteurs indépendants — un sceptique, et des experts des domaines touchés — relisent le plan et vous soumettent au plus cinq points."),
            ("Les modifications sont des deltas", "Une modification ajoute, modifie ou retire des exigences des spécifications actuelles : Compagnon fonctionne sur du code existant, pas seulement sur un projet neuf."),
            ("Les décisions sont scellées", "Une décision d’architecture prend effet quand vous l’acceptez ; Compagnon scelle son texte, et la changer revient à la remplacer."),
            ("Documentation et tests dans chaque plan", "Chaque scénario est couvert par un test dans une tâche, et la documentation concernée est mise à jour dans une tâche."),
            ("La langue de votre équipe", "Les plans sont écrits dans la langue choisie par votre équipe ; leur structure reste lisible par les outils."),
            ("Mesuré, pas supposé", "Chaque modification des compétences est évaluée en qualité et en coût avant d’être publiée."),
        ],
        "why_links": '<a href="fr/workflow.html">Comment ça marche <span aria-hidden="true">→</span></a><a href="fr/roles.html">Rôles <span aria-hidden="true">→</span></a><a href="fr/reliability.html">Fiabilité des rôles <span aria-hidden="true">→</span></a><a href="https://github.com/mairistem/compagnon/tree/main/_compagnon">Compagnon est développé avec Compagnon <span aria-hidden="true">→</span></a>',
        "footer_logo": "Mairistem par JVS",
        "footer_text": 'Compagnon — un projet libre de <a href="https://www.jvs-mairistem.fr/">Mairistem</a> · <a href="https://github.com/mairistem/compagnon/blob/main/LICENSE">Licence MIT</a>',
        "footer_links": '<li><a href="fr/">Documentation</a></li><li><a href="./" hreflang="en" lang="en">Home in English</a></li><li><a href="https://github.com/mairistem/compagnon/releases/latest">Versions publiées</a></li><li><a href="https://github.com/mairistem/compagnon">GitHub</a></li>',
    },
}


def page(t, old):
    lang = t["lang"]
    proofs = "".join(f"<li>{p}</li>" for p in t["proofs"])
    stats = "".join(f"<li><b>{n}</b><span>{label}</span></li>" for n, label in t["stats"])
    nav = "\n      ".join(
        f'<a href="{lang}/{href}">{title}</a>' for _, title, href, _, _ in t["tunnel"]
    )
    tunnel = "\n".join(
        f'        <li class="step" style="--c:{c}"><span class="rule" aria-hidden="true"></span><div>'
        f'<p class="eyebrow">{t["step_word"]} {i + 1} · {pages}</p>'
        f'<h3><a href="{lang}/{href}">{title}</a></h3><p>{lead}</p>'
        f'<span class="go" aria-hidden="true">{t["start_here"]} →</span></div></li>'
        for i, (c, title, href, pages, lead) in enumerate(t["tunnel"])
    )
    jira_steps = "\n".join(f"          <li>{s}</li>" for s in t["jira_steps"])
    steps = "\n".join(
        f'        <details class="acc"><summary><b class="n {kind}">{n}</b> {title}</summary><p>{text}</p><a href="{href}">{t["go_further"]}{":" if lang == "en" else " :"} {link}</a></details>'
        for kind, n, title, text, href, link in t["steps"]
    )
    features = "\n".join(
        f'        <li class="feature"><h3>{h}</h3><p>{p}</p></li>' for h, p in t["features"]
    )
    head = head_of(old).replace('<meta name="theme-color" content="#00263a">', '<meta name="theme-color" content="#004c66">')
    sprites = sprites_of(old)
    script = script_of(old)
    return f"""{head}{CSS}<body>
{sprites}
<a class="skip" href="#main">{t["skip"]}</a>
<header>
  <div class="topband">
    <div class="container">
      <span>{t["topband"]}</span>
      <nav aria-label="{t["nav_label"]} — {"language" if lang == "en" else "langue"}">{t["other"]} <a class="pill" href="https://github.com/mairistem/compagnon">GitHub</a></nav>
    </div>
  </div>
  <div class="mainband">
    <div class="container">
      <a class="brand" href="{t["brand_href"]}" aria-label="{t["brand_label"]}"><svg class="cp-logo" viewBox="0 0 1188 250" aria-hidden="true" focusable="false"><use href="#cp-logo"></use></svg></a>
      <nav class="site-nav" aria-label="{t["nav_label"]}">
      {nav}
      </nav>
      <a class="btn primary sm" href="#install-title">{t["install_btn"]}</a>
    </div>
  </div>
</header>
<main id="main">
  <section class="hero" aria-labelledby="hero-title">
    {symbol("plein-03.svg")}
    <div class="container">
      <div class="hero-text">
        <p class="eyebrow corail"><span class="sq" aria-hidden="true"></span>{t["eyebrow"]}</p>
        <h1 id="hero-title">{t["h1"]}</h1>
        <p class="lead">{t["lead"]}</p>
        <p class="btns">
          <a class="btn primary" href="{lang}/introduction.html">{t["read_docs"]}</a>
          <a class="btn secondary" href="https://github.com/mairistem/compagnon">{t["github"]}</a>
        </p>
        <ul class="proofs">{proofs}</ul>
      </div>
      {t["session"]}
    </div>
  </section>

  <section class="statbar" aria-label="{t["stats_label"]}">
    <ul class="container">{stats}</ul>
  </section>

  <section class="section" aria-labelledby="steps-title">
    <div class="container">
      <div class="section-head">
        <p class="eyebrow">{t["steps_eyebrow"]}</p>
        <h2 id="steps-title">{t["steps_title"]}</h2>
      </div>
      <ol class="steps">
{tunnel}
      </ol>
      <p class="more">{t["origins"]}</p>
    </div>
  </section>

  <section class="section section-alt" aria-labelledby="jira-title">
    <div class="container jira">
      <div class="say">
        <p class="eyebrow">{t["jira_eyebrow"]}</p>
        <h2 id="jira-title">{t["jira_title"]}</h2>
        <p class="say-quote">{t["jira_say"]}</p>
        <ul class="checks">
{jira_steps}
        </ul>
        <p class="more">{t["jira_link"]}</p>
      </div>
{t["jira_figure"]}
    </div>
  </section>

  <section class="section" aria-labelledby="how-title">
    <div class="container">
      <div class="section-head">
        <p class="eyebrow">{t["how_eyebrow"]}</p>
        <h2 id="how-title">{t["how_title"]}</h2>
        <p>{t["how_lead"]}</p>
      </div>
{t["workflow_figure"]}
      <div class="accs" role="group" aria-label="{t["steps_label"]}">
{steps}
      </div>
    </div>
  </section>

  <section class="section section-alt" aria-labelledby="plan-title">
    <div class="container">
      <div class="section-head">
        <p class="eyebrow">{t["plan_eyebrow"]}</p>
        <h2 id="plan-title">{t["plan_title"]}</h2>
        <p>{t["plan_lead"]}</p>
      </div>
{t["plan_figure"]}
    </div>
  </section>

  <section class="section" aria-labelledby="install-title">
    <div class="container">
      <div class="section-head">
        <p class="eyebrow">{t["install_eyebrow"]}</p>
        <h2 id="install-title">{t["install_title"]}</h2>
        <p>{t["install_lead"]}</p>
      </div>
      <div class="install-grid">
        <div class="codeblock"><header><h3 id="install-unix">{t["unix"]}</h3><button class="copy" type="button" data-copy aria-describedby="install-unix">{t["copy"]}</button></header><pre aria-labelledby="install-unix"><code>curl -sSL https://raw.githubusercontent.com/mairistem/compagnon/main/install.sh | sh</code></pre></div>
        <div class="codeblock"><header><h3 id="install-win">Windows (PowerShell)</h3><button class="copy" type="button" data-copy aria-describedby="install-win">{t["copy"]}</button></header><pre aria-labelledby="install-win"><code>iwr -useb https://raw.githubusercontent.com/mairistem/compagnon/main/install.ps1 | iex</code></pre></div>
      </div>
      <p class="more">{t["then"]}</p>
      <p class="more">{t["install_links"]}</p>
      <p class="visually-hidden" role="status" aria-live="polite" id="copy-status"></p>
    </div>
  </section>

  <section class="section section-alt" aria-labelledby="why-title">
    <div class="container">
      <div class="section-head">
        <p class="eyebrow">{t["why_eyebrow"]}</p>
        <h2 id="why-title">{t["why_title"]}</h2>
      </div>
      <ul class="features">
{features}
      </ul>
      <p class="more">{t["why_links"]}</p>
    </div>
  </section>
</main>
<footer class="site-footer">
  <div class="container">
    <div class="footer-brand">
      <a href="https://www.jvs-mairistem.fr/" aria-label="Mairistem"><svg class="logo" viewBox="0 0 1113 249" role="img" aria-label="{t["footer_logo"]}"><use href="#mx-logo"/></svg></a>
      <p>{t["footer_text"]}</p>
    </div>
    <ul class="footer-links">{t["footer_links"]}</ul>
  </div>
</footer>
{script}"""


for lang, name in [("en", "index.html"), ("fr", "fr.html")]:
    path = SITE / name
    old = path.read_text()
    path.write_text(page(T[lang], old))
    print("wrote", path)
