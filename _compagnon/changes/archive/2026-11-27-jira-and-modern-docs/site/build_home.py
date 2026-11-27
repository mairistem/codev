"""Build docs/site/index.html and docs/site/fr.html from the approved mockup.

Run from the repository root:
    python3 _compagnon/changes/jira-and-modern-docs/site/build_home.py

It keeps from the current pages what is theirs (the head up to the styles,
the sprites, the copy script) and takes the diagrams from the chapters that
own them, so the home pages draw exactly what the books draw.
"""
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[4]
SITE = ROOT / "docs/site"


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
  /* Layout: a developer tool's site — dark hero with a static terminal,
     reading surfaces that follow the reader's color scheme, mono accents;
     coral marks what you decide, blue what Claude does. */
  --font: "IBM Plex Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Arial, sans-serif;
  --mono: ui-monospace, "SF Mono", SFMono-Regular, Menlo, Consolas, "Liberation Mono", monospace;
  --corail: #ff5331;
  --bg: #ffffff;
  --fg: #2b3f47;
  --heading: #00263a;
  --muted: #4f6670;
  --surface: #f4f8f9;
  --line: #dce5e8;
  --link: #c4361b;
  --focus: #ffab03;
  --code-bg: #f4f8f9;
  --code-fg: #00394d;
  --chip-bg: #e7f0f3;
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
    --bg: #0b1c24;
    --fg: #d5e2e7;
    --heading: #ffffff;
    --muted: #a3b8c0;
    --surface: #10262f;
    --line: #22404c;
    --link: #ff8a70;
    --code-bg: #071820;
    --code-fg: #d8e6ea;
    --chip-bg: #16343f;
    --cp-logo-ink: #ffffff;
    --cp-logo-c: #ffffff;
    color-scheme: dark;
  }
}

@media (prefers-color-scheme: dark) {
  :root {
    --dg-bg: #0b1c24;
    --dg-box: #12303d;
    --dg-file: #0b1c24;
    --dg-ink: #eef4f6;
    --dg-muted: #a9bcc3;
    --dg-line: #8fa8b1;
    --dg-you: #ff7a5c;
    --dg-you-text: #ff7a5c;
    --dg-on-you: #1a0a05;
    --dg-claude: #5fb8d3;
  }
}

* { box-sizing: border-box; }
html { scroll-behavior: smooth; }
body { margin: 0; background: var(--bg); color: var(--fg); font: 400 16px/1.6 var(--font); -webkit-font-smoothing: antialiased; }
h1, h2, h3 { color: var(--heading); line-height: 1.2; text-wrap: balance; margin: 0; }
p { margin: 0; }
a { color: var(--link); text-underline-offset: 3px; }
code { font-family: var(--mono); font-size: .88em; background: var(--chip-bg); color: var(--code-fg); padding: .1em .35em; border-radius: 4px; }
:focus-visible { outline: 3px solid var(--focus); outline-offset: 2px; border-radius: 4px; }
.visually-hidden { position: absolute; width: 1px; height: 1px; overflow: hidden; clip: rect(0 0 0 0); white-space: nowrap; }
.skip { position: absolute; left: 16px; top: -48px; z-index: 50; background: var(--heading); color: var(--bg); padding: 10px 14px; border-radius: 8px; font-weight: 600; }
.skip:focus { top: 12px; }
.container { max-width: 1120px; margin: 0 auto; padding-left: max(16px, env(safe-area-inset-left, 0px)); padding-right: max(16px, env(safe-area-inset-right, 0px)); }
.eyebrow { font: 500 .78rem/1 var(--mono); letter-spacing: .08em; text-transform: uppercase; color: var(--muted); }

.site-header { border-bottom: 1px solid var(--line); background: var(--bg); }
.site-header .container { display: flex; align-items: center; justify-content: space-between; gap: 16px; padding-block: 14px; flex-wrap: wrap; }
.brand { display: inline-flex; line-height: 0; border-radius: 6px; }
.cp-logo { display: block; height: 40px; width: auto; aspect-ratio: 1188 / 250; max-width: 100%; }
.site-nav { display: flex; gap: 20px; flex-wrap: wrap; font-weight: 500; }
.site-nav a { color: var(--heading); text-decoration: none; }
.site-nav a:hover { color: var(--link); }

.hero { background: radial-gradient(1200px 500px at 85% -10%, #0b5d7a 0%, transparent 60%), linear-gradient(160deg, #00263a 0%, #003a4f 55%, #004c66 100%); color: #d9e7ec; position: relative; overflow: hidden; }
.hero::before { content: ""; position: absolute; inset: 0; background-image: linear-gradient(rgba(255,255,255,.05) 1px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,.05) 1px, transparent 1px); background-size: 32px 32px; mask-image: linear-gradient(180deg, #000 30%, transparent 100%); }
.hero .container { position: relative; display: grid; grid-template-columns: minmax(0, 1fr) minmax(0, 1.05fr); gap: 48px; align-items: center; padding-block: 72px 80px; }
.hero h1 { color: #ffffff; font-size: clamp(2.1rem, 4.4vw, 3.4rem); font-weight: 700; letter-spacing: -.02em; }
.hero .eyebrow { color: #9fd9e9; margin-bottom: 18px; display: flex; gap: 10px; align-items: center; }
.hero .eyebrow i { width: 8px; height: 8px; border-radius: 50%; background: #00c39b; display: inline-block; }
.hero .lead { margin-top: 20px; font-size: 1.15rem; max-width: 34em; color: #d9e7ec; }
.actions { display: flex; flex-wrap: wrap; gap: 12px; margin-top: 28px; }
.button { display: inline-flex; align-items: center; gap: 8px; padding: 12px 18px; border-radius: 10px; font-weight: 600; text-decoration: none; }
.button-primary { background: var(--corail); color: #1a0a05; }
.button-primary:hover { background: #ff6d50; }
.button-ghost { color: #ffffff; border: 1px solid rgba(255,255,255,.4); }
.button-ghost:hover { border-color: #ffffff; }

.term { margin: 0; background: #071820; border: 1px solid rgba(255,255,255,.12); border-radius: 14px; box-shadow: 0 30px 60px -20px rgba(0,0,0,.55); min-width: 0; }
.term-bar { display: flex; align-items: center; gap: 8px; padding: 10px 14px; border-bottom: 1px solid rgba(255,255,255,.08); font: 500 12px/1 var(--mono); color: #8fa8b1; }
.term-bar i { width: 11px; height: 11px; border-radius: 50%; display: inline-block; }
.term-bar b { font-weight: 500; margin-left: 8px; }
.term pre { margin: 0; padding: 18px 18px 20px; font: 13.5px/1.75 var(--mono); color: #d8e6ea; overflow-x: auto; }
.term .p { color: #ff7a5c; }
.term .u { color: #ffffff; }
.term .c { color: #5fd3b3; }
.term .y { color: #ffc75a; }
.term .m { color: #8fa8b1; }
.term figcaption { padding: 0 18px 14px; font-size: 12.5px; color: #a3b8c0; }

.section { padding-block: 72px; }
.section-alt { background: var(--surface); border-block: 1px solid var(--line); }
.section-head { max-width: 46em; display: grid; gap: 12px; margin-bottom: 32px; }
.section-head h2 { font-size: clamp(1.6rem, 3vw, 2.2rem); letter-spacing: -.01em; }
.section-head p { color: var(--muted); font-size: 1.05rem; }
.more { margin-top: 22px; display: flex; flex-wrap: wrap; gap: 8px 20px; }

.paths { list-style: none; margin: 0; padding: 0; display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 16px; }
.path { position: relative; border: 1px solid var(--line); border-radius: 14px; padding: 22px 22px 20px; background: var(--bg); display: grid; gap: 8px; }
.path::before { content: ""; position: absolute; left: 18px; right: 18px; top: -1px; height: 3px; border-radius: 0 0 3px 3px; background: var(--c); }
.path h3 { font-size: 1.12rem; }
.path h3 a { color: var(--heading); text-decoration: none; }
.path h3 a::after { content: ""; position: absolute; inset: 0; border-radius: 14px; }
.path:hover { border-color: var(--c); }
.path:focus-within { outline: 3px solid var(--focus); outline-offset: 2px; }
.path h3 a:focus-visible { outline: none; }
.path p { color: var(--muted); }
.path .meta { font: 500 .78rem/1 var(--mono); display: flex; gap: 8px; align-items: center; }
.path .meta i { width: 8px; height: 8px; border-radius: 50%; background: var(--c); }

.jira { display: grid; grid-template-columns: minmax(0, .8fr) minmax(0, 1.2fr); gap: 40px; align-items: center; }
.jira .say { display: grid; gap: 14px; align-content: start; }
.say-quote { font: 500 1.05rem/1.5 var(--mono); color: var(--heading); background: var(--bg); border: 1px solid var(--line); border-left: 4px solid var(--dg-you); border-radius: 10px; padding: 14px 16px; }
.steps3 { list-style: none; margin: 0; padding: 0; display: grid; gap: 10px; }
.steps3 li { display: grid; grid-template-columns: 28px 1fr; gap: 10px; align-items: start; }
.steps3 b { font: 600 .8rem/28px var(--mono); text-align: center; border-radius: 50%; background: var(--chip-bg); color: var(--heading); }

figure.diagram { margin: 0; display: grid; gap: 10px; min-width: 0; }
figure.diagram > .scroll { overflow-x: auto; border: 1px solid var(--line); border-radius: 14px; background: var(--dg-bg); padding: 12px; }
figure.diagram svg { display: block; width: 100%; height: auto; min-width: 560px; }
figure.diagram figcaption { color: var(--muted); font-size: .92rem; }
figure.diagram text { font-family: var(--font); }
figure.diagram .mono { font-family: var(--mono); }

.diagram-steps { display: grid; grid-template-columns: repeat(5, minmax(0, 1fr)); gap: 10px; margin-top: 14px; }
.diagram-steps details { border: 1px solid var(--line); border-radius: 12px; background: var(--bg); padding: 10px 12px; min-width: 0; }
.diagram-steps details[open] { grid-column: 1 / -1; border-color: var(--heading); }
.diagram-steps summary { cursor: pointer; font-weight: 600; color: var(--heading); display: flex; gap: 8px; align-items: center; list-style: none; }
.diagram-steps summary::-webkit-details-marker { display: none; }
.diagram-steps summary::after { content: "+"; margin-left: auto; font: 600 16px/1 var(--mono); color: var(--muted); }
.diagram-steps details[open] summary::after { content: "–"; }
.diagram-steps p { margin: 10px 0 8px; max-width: 60em; }
.n { font: 700 12px/22px var(--mono); width: 22px; height: 22px; border-radius: 50%; text-align: center; flex: none; }
.n.you { background: var(--dg-you); color: var(--dg-on-you); }
.n.claude { background: var(--dg-claude); color: var(--dg-box); }
.n.kept { background: var(--dg-line); color: var(--dg-box); }

.install-grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 16px; }
.codeblock { border: 1px solid var(--line); border-radius: 12px; overflow: hidden; background: var(--code-bg); min-width: 0; }
.codeblock header { display: flex; justify-content: space-between; align-items: center; gap: 8px; padding: 8px 12px; border-bottom: 1px solid var(--line); background: var(--surface); }
.codeblock h3 { font: 500 12px/1 var(--mono); color: var(--muted); }
.codeblock pre { margin: 0; padding: 14px 16px; overflow-x: auto; font: 13.5px/1.6 var(--mono); color: var(--code-fg); }
.codeblock pre code { background: none; padding: 0; font-size: inherit; }
.copy { font: 500 12px/1 var(--mono); color: var(--heading); background: var(--bg); border: 1px solid var(--line); border-radius: 6px; padding: 6px 9px; cursor: pointer; }
.copy[data-state="done"] { border-color: #00c39b; }

.features { list-style: none; margin: 0; padding: 0; display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 16px; }
.feature { border: 1px solid var(--line); border-radius: 14px; padding: 18px 20px; display: grid; gap: 8px; align-content: start; }
.feature h3 { font-size: 1.02rem; }
.feature p { color: var(--muted); font-size: .95rem; }

.site-footer { background: #00263a; color: #c3d6dd; padding-block: 40px; }
.site-footer .container { display: flex; flex-wrap: wrap; gap: 24px; justify-content: space-between; align-items: center; }
.site-footer a { color: #ffffff; }
.footer-brand { display: flex; flex-wrap: wrap; gap: 16px; align-items: center; }
.site-footer .logo { height: 30px; width: auto; aspect-ratio: 1113 / 249; --mx-logo-ink: #fff; --mx-logo-corail: #fff; --mx-logo-jaune: #fff; --mx-logo-bleu: #fff; --mx-logo-vert: #fff; }
.footer-links { list-style: none; margin: 0; padding: 0; display: flex; flex-wrap: wrap; gap: 8px 20px; }

@media (prefers-reduced-motion: no-preference) {
  .term pre > span.l { animation: line-in .45s ease-out both; }
  @keyframes line-in { from { opacity: .25; transform: translateY(4px); } to { opacity: 1; transform: none; } }
  .path, .button, .diagram-steps details, .codeblock { transition: transform .2s ease, border-color .2s ease, box-shadow .2s ease; }
  .path:hover { transform: translateY(-3px); box-shadow: 0 14px 30px -18px rgba(0, 38, 58, .45); }
  .button:hover { transform: translateY(-1px); }
  .cursor { display: inline-block; width: .6em; height: 1.1em; vertical-align: -2px; background: #5fd3b3; animation: blink 1.1s steps(2, start) 4; }
  @keyframes blink { to { visibility: hidden; } }
}
@media (max-width: 900px) {
  .hero .container, .jira { grid-template-columns: minmax(0, 1fr); }
  .hero .container { padding-block: 48px 56px; gap: 32px; }
  .paths, .install-grid, .features { grid-template-columns: minmax(0, 1fr); }
  .diagram-steps { grid-template-columns: minmax(0, 1fr); }
}
@media (max-width: 480px) {
  .cp-logo { height: 32px; }
  .section { padding-block: 52px; }
  .term pre { font-size: 12px; }
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
        "eyebrow": "Open source · for Claude Code",
        "h1": "Plan with Claude before it codes.",
        "lead": "Before any code is written, Claude writes down what it is about to build: a short plan you read, correct and approve. Then Claude builds exactly that, and the plan stays with your code.",
        "read_docs": "Read the documentation",
        "github": "View on GitHub",
        "term_title": "claude — ~/shop-app",
        "term": [
            '<span class="p">&gt;</span> <span class="u">Prepare a plan for PROJ-123</span>',
            "",
            '<span class="c">●</span> Reading ticket <span class="y">PROJ-123</span> — “Dark mode for the dashboard”',
            '<span class="c">●</span> Writing the plan in <span class="m">_compagnon/changes/</span>dark-mode/',
            '    <span class="c">✓</span> proposal.md   <span class="m">why, and what changes</span>',
            '    <span class="c">✓</span> specs/        <span class="m">the expected behavior</span>',
            '    <span class="c">✓</span> design.md     <span class="m">how it will be built</span>',
            '    <span class="c">✓</span> tasks.md      <span class="m">7 tasks, each with its test</span>',
            '<span class="c">●</span> Re-read by 3 reviewers — 2 points fixed',
            "",
            '<span class="u">The plan is ready for your review.</span>',
            '<span class="m">Nothing is built until you approve it.</span>',
        ],
        "term_caption": "An example exchange: you name the ticket, Claude writes the plan and waits for you.",
        "paths_eyebrow": "Choose your path",
        "paths_title": "What will you do with Compagnon?",
        "paths_lead": "Pick the answer closest to what you will do. Each one opens a short path of at most three pages.",
        "paths": [
            ("#ff5331", "no terminal · 3 pages", "review", "I review and approve plans — I do not code", "Claude writes a plan; you check that it says the right thing, ask for changes, then approve it."),
            ("#ffab03", "plain words · 3 pages", "build", "I do not code, but I want Claude to build something for me", "You describe your idea in plain words, approve Claude’s plan, and Claude builds it."),
            ("#00c39b", "10 minutes · 3 pages", "develop", "I am a developer and I have Claude write code", "A first change in ten minutes, then how to ask for each step."),
            ("#004c66", "setup · 3 pages", "team", "I set Compagnon up for my team", "Installation, checks on the shared project, who reviews plans and code."),
        ],
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
            ("you", "3", "You approve", "Read the plan, ask for corrections, approve it — in the conversation or on GitHub and GitLab. No terminal needed.", "en/guides/review-a-plan.html", "Reviewing a plan"),
            ("claude", "4", "Claude builds", "Claude implements the tasks in order, runs each test, and stops at the first doubt instead of guessing.", "en/workflow.html", "How it works"),
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
        "eyebrow": "Libre · pour Claude Code",
        "h1": "Planifiez avec Claude avant qu’il ne code.",
        "lead": "Avant d’écrire la moindre ligne de code, Claude met par écrit ce qu’il s’apprête à construire : un plan court que vous lisez, corrigez et validez. Ensuite, Claude construit exactement cela, et le plan reste avec votre code.",
        "read_docs": "Lire la documentation",
        "github": "Voir sur GitHub",
        "term_title": "claude — ~/boutique",
        "term": [
            '<span class="p">&gt;</span> <span class="u">Prépare un plan pour PROJ-123</span>',
            "",
            '<span class="c">●</span> Lecture du ticket <span class="y">PROJ-123</span> — « Mode sombre du tableau de bord »',
            '<span class="c">●</span> Rédaction du plan dans <span class="m">_compagnon/changes/</span>mode-sombre/',
            '    <span class="c">✓</span> proposal.md   <span class="m">pourquoi, et ce qui change</span>',
            '    <span class="c">✓</span> specs/        <span class="m">le comportement attendu</span>',
            '    <span class="c">✓</span> design.md     <span class="m">comment ce sera construit</span>',
            '    <span class="c">✓</span> tasks.md      <span class="m">7 tâches, chacune avec son test</span>',
            '<span class="c">●</span> Relu par 3 relecteurs — 2 points corrigés',
            "",
            '<span class="u">Le plan est prêt pour votre relecture.</span>',
            '<span class="m">Rien n’est construit avant votre accord.</span>',
        ],
        "term_caption": "Un exemple d’échange : vous nommez le ticket, Claude rédige le plan et attend votre accord.",
        "paths_eyebrow": "Choisissez votre parcours",
        "paths_title": "Qu’allez-vous faire avec Compagnon ?",
        "paths_lead": "Choisissez la réponse la plus proche de ce que vous allez faire. Chacune ouvre un parcours court, de trois pages au plus.",
        "paths": [
            ("#ff5331", "sans terminal · 3 pages", "review", "Je relis et je valide des plans — je ne code pas", "Claude rédige un plan ; vous vérifiez qu’il dit la bonne chose, demandez des corrections, puis le validez."),
            ("#ffab03", "avec vos mots · 3 pages", "build", "Je ne code pas, mais je veux que Claude construise quelque chose pour moi", "Vous décrivez votre idée avec vos mots, validez le plan de Claude, et Claude la réalise."),
            ("#00c39b", "10 minutes · 3 pages", "develop", "Je suis développeur et je fais coder Claude", "Une première modification en dix minutes, puis comment demander chaque étape."),
            ("#004c66", "installation · 3 pages", "team", "J'installe Compagnon pour mon équipe", "Installation, vérifications sur le projet partagé, qui relit les plans et le code."),
        ],
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
            ("you", "3", "Vous validez", "Lisez le plan, demandez des corrections, validez-le — dans la conversation ou sur GitHub et GitLab. Sans terminal.", "fr/guides/review-a-plan.html", "Relire un plan"),
            ("claude", "4", "Claude construit", "Claude réalise les tâches dans l’ordre, lance chaque test et s’arrête au premier doute au lieu de deviner.", "fr/workflow.html", "Comment ça marche"),
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
    term = "\n".join(
        f'<span class="l" style="animation-delay:{i * 0.12:.2f}s">{line}</span>' if line else ""
        for i, line in enumerate(t["term"])
    )
    term = term.replace(
        t["term"][-1] + "</span>",
        t["term"][-1] + ' <span class="cursor" aria-hidden="true"></span></span>',
    )
    paths = "\n".join(
        f'        <li class="path" style="--c:{c}"><p class="meta"><i></i>{meta}</p><h3><a href="{t["lang"]}/paths/{slug}.html">{label}</a></h3><p>{desc}</p></li>'
        for c, meta, slug, label, desc in t["paths"]
    )
    jira_steps = "\n".join(
        f"          <li><b>{i + 1}</b><span>{s}</span></li>" for i, s in enumerate(t["jira_steps"])
    )
    steps = "\n".join(
        f'        <details><summary><b class="n {kind}">{n}</b> {title}</summary><p>{text}</p><a href="{href}">{t["go_further"]} : {link}</a></details>'.replace(
            " : " if t["lang"] == "en" else "\x00", ": " if t["lang"] == "en" else "\x00"
        )
        for kind, n, title, text, href, link in t["steps"]
    )
    features = "\n".join(
        f"        <li class=\"feature\"><h3>{h}</h3><p>{p}</p></li>" for h, p in t["features"]
    )
    head = head_of(old).replace('<meta name="theme-color" content="#ffffff">', '<meta name="theme-color" content="#00263a">')
    sprites = sprites_of(old)
    script = script_of(old)
    return f"""{head}{CSS}<body>
{sprites}
<a class="skip" href="#main">{t["skip"]}</a>
<header class="site-header">
  <div class="container">
    <a class="brand" href="{t["brand_href"]}" aria-label="{t["brand_label"]}"><svg class="cp-logo" viewBox="0 0 1188 250" aria-hidden="true" focusable="false"><use href="#cp-logo"></use></svg></a>
    <nav class="site-nav" aria-label="{t["nav_label"]}">
      <a class="nav-docs" href="{t["docs"]}">Documentation</a>
      {t["other"]}
      <a href="https://github.com/mairistem/compagnon">GitHub</a>
    </nav>
  </div>
</header>
<main id="main">
  <section class="hero" aria-labelledby="hero-title">
    <div class="container">
      <div>
        <p class="eyebrow"><i></i>{t["eyebrow"]}</p>
        <h1 id="hero-title">{t["h1"]}</h1>
        <p class="lead">{t["lead"]}</p>
        <div class="actions">
          <a class="button button-primary" href="{t["docs"]}">{t["read_docs"]}</a>
          <a class="button button-ghost" href="https://github.com/mairistem/compagnon">{t["github"]}</a>
        </div>
      </div>
      <figure class="term">
        <div class="term-bar" aria-hidden="true"><i style="background:#ff5331"></i><i style="background:#ffab03"></i><i style="background:#00c39b"></i><b>{t["term_title"]}</b></div>
<pre>{term}</pre>
        <figcaption>{t["term_caption"]}</figcaption>
      </figure>
    </div>
  </section>

  <section class="section" aria-labelledby="paths-title">
    <div class="container">
      <div class="section-head">
        <p class="eyebrow">{t["paths_eyebrow"]}</p>
        <h2 id="paths-title">{t["paths_title"]}</h2>
        <p>{t["paths_lead"]}</p>
      </div>
      <ul class="paths">
{paths}
      </ul>
      <p class="more">{t["origins"]}</p>
    </div>
  </section>

  <section class="section section-alt" aria-labelledby="jira-title">
    <div class="container jira">
      <div class="say">
        <p class="eyebrow">{t["jira_eyebrow"]}</p>
        <h2 id="jira-title">{t["jira_title"]}</h2>
        <p class="say-quote">{t["jira_say"]}</p>
        <ol class="steps3">
{jira_steps}
        </ol>
        <p>{t["jira_link"]}</p>
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
      <div class="diagram-steps" role="group" aria-label="{t["steps_label"]}">
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
