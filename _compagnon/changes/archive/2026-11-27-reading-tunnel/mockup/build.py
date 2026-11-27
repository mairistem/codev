"""Build the tunnel mockup: python3 build.py (from this folder or anywhere).

Writes index.html next to this file: the table of contents of the tunnel,
then one card per chapter — what a beginner sees (purpose, diagram,
example), what is folded for experts, the skip button and the next step.
"""
import html
import pathlib

HERE = pathlib.Path(__file__).resolve().parent

STEPS = [
    ("discover", "1", "Discover", "Découvrir", "#ff5331"),
    ("start", "2", "Get started", "Démarrer", "#ffab03"),
    ("daily", "3", "Every day", "Au quotidien", "#00c39b"),
    ("understand", "4", "Understand", "Comprendre", "#5fb8d3"),
    ("team", "5", "For your team", "Pour l'équipe", "#8fa8b1"),
]

# (id, step, title EN, title FR, purpose, diagram, example, folded topics, skip, next)
PAGES = [
    ("introduction", "discover", "Compagnon in two minutes", "Compagnon en deux minutes",
     "What Compagnon does, who decides, and the five steps of this documentation.",
     "The workflow: you ask → Claude plans → you approve → Claude builds → filed away. Then the five steps you are about to read.",
     "You say “Add a dark mode that follows the system setting”. Claude answers with a plan of four short files and waits: “Nothing is built until you approve it.”",
     ["Why the name Compagnon"], None, "installation"),
    ("installation", "start", "Installation", "Installation",
     "Get Claude Code and a project folder if you start from nothing, install Compagnon in one line, set up your project, check it works.",
     "Installing: install Compagnon → set up the project (compagnon init) → your first request.",
     "On a Mac: curl … | sh, then in your project compagnon init answers “✓ 8 skills installed · ✓ CLAUDE.md updated · Next: ask Claude for a change”.",
     ["Other ways to install (manual download, cargo)", "Installing a specific version", "Shell completions", "Updating Compagnon"], None, "quickstart"),
    ("quickstart", "start", "Your first change", "Votre première modification",
     "One whole change in ten minutes: ask, read the plan, let Claude build it, file it away.",
     "A change's folder: _compagnon/changes/dark-mode/ with its four files → archive → the project's specs updated.",
     "“Add a dark mode that follows the system setting” → plan in _compagnon/changes/dark-mode/ → “implement it” → 7 tasks done → “archive it”.",
     ["The commands behind each request (/compagnon-propose …)", "What each command prints"],
     ("Already made a change? Skip to Asking Claude", "skills"), "skills"),
    ("skills", "daily", "Asking Claude", "Parler à Claude",
     "What to say to Claude at each step, in your own words.",
     "Asking Claude: your words → the skill Claude picks → what it does.",
     "“Prepare a plan for a login page” · “Implement it” · “Where am I?” · “Archive it” — each with Claude's first line of answer.",
     ["The slash commands, for those who prefer them", "The update notice", "When Claude learns from your corrections"], None, "mcp"),
    ("mcp", "daily", "Start from a Jira ticket", "Partir d'un ticket Jira",
     "Name a Jira ticket in your request: Claude reads it once and cites it at the top of the plan.",
     "From a Jira ticket to a plan: PROJ-123 → Claude reads it once, read-only → proposal.md citing it.",
     "“Prepare a plan for PROJ-123” → the proposal opens with “Source: ticket PROJ-123 — Dark mode for the dashboard (In Progress)”.",
     ["Setting it up (mcp.jira_tool, what compagnon init detects)", "What Claude may and may not do with Jira", "Other connected tools"],
     ("No Jira? Skip to Reviewing a plan", "review-a-plan"), "review-a-plan"),
    ("review-a-plan", "daily", "Reviewing a plan", "Relire un plan",
     "What to check in a plan before you say yes, file by file.",
     "Inside a plan: proposal (why) → specs (what) → design (how) → tasks — most people stop after the first.",
     "A real excerpt: the rule “The chosen theme is remembered” before and after the change, and the question to ask Claude about it.",
     ["The review record and the reviewers' points", "How points to challenge are chosen"], None, "approve-a-plan"),
    ("approve-a-plan", "daily", "Approving a plan on GitHub", "Approuver un plan sur GitHub",
     "Comment, ask for changes and approve a plan on GitHub, without a terminal.",
     "Approving on GitHub: plan pushed → comments → approval → Claude builds → code reviewed.",
     "Pull request “Plan: dark mode”: a comment “Follow the system setting only, no toggle”, Claude's updated plan, then “Approve”.",
     ["Protecting the main branch", "The checks GitHub runs on a plan"],
     ("You work alone, in the conversation? Skip to Building and filing away", "build-and-file"), "build-and-file"),
    ("build-and-file", "daily", "Building and filing away", "Réaliser et ranger",
     "Let Claude build the approved plan task by task, then file the change away.",
     "Building and filing away: tasks done one by one → review → archive → specs updated.",
     "“Implement it” → “✓ 7 of 7 tasks · 2 review notes, nothing to decide” → “Archive it” → “Moved to _compagnon/changes/archive/2026-11-27-dark-mode”.",
     ["Updating the specs without archiving (sync)", "When archiving refuses", "Stopping on a doubt"], None, "workflow"),
    ("workflow", "understand", "How it works", "Comment ça marche",
     "What happens behind each step: the plan's files, who writes them, how a plan is built.",
     "How a plan is built: understand → investigate → write → trace → challenge → you approve.",
     None, ["The roles at each step", "Validation rules"],
     ("You only use Compagnon? Skip to Adopting Compagnon on an existing project", "existing-project"), "concepts"),
    ("concepts", "understand", "Concepts", "Concepts",
     "Specs, decisions, changes and deltas: what the project keeps and why.",
     "The project's records: specs (what) · decisions (why) · changes (the work), and a delta merging into the specs.",
     None, ["Capabilities", "Schemas"], None, "decisions"),
    ("decisions", "understand", "Decisions", "Décisions",
     "Architecture decisions: proposed, accepted, replaced, departed from.",
     "The life of a decision: proposed → accepted (sealed) → superseded, with deviations and promotions.",
     None, ["The commands", "The seal"], None, "roles"),
    ("roles", "understand", "Roles", "Rôles",
     "Who reviews what, at each step, and how to configure it.",
     "Who reviews when: the steps of a change across, the roles down.",
     None, ["Inline or isolated review", "Each role's criteria", "The reviewers' models"],
     ("No review to configure? Skip to Review record", "review-record"), "review-record"),
    ("review-record", "understand", "Review record", "Compte rendu de relecture",
     "The record of every finding, and why it can block archiving.",
     "The life of a finding: open → fixed / disputed / accepted risk / deferred → archive gate.",
     None, ["The record's format", "Changes started with an older Compagnon"], None, "existing-project"),
    ("existing-project", "team", "Adopting Compagnon on an existing project", "Adopter Compagnon sur un projet existant",
     "Start on a project that already has code and no specs.", None, None, [], None, "team-rollout"),
    ("team-rollout", "team", "Deploying Compagnon for a team", "Déployer Compagnon pour une équipe",
     "Set up a shared project: what compagnon init writes, compagnon doctor, CI on GitHub, who reviews.",
     None, None, [], None, "inherited-sources"),
    ("inherited-sources", "team", "Inherited sources", "Sources héritées",
     "Share conventions and decisions across projects.", "Inherited sources: projects reading a shared repository, pinned by a lock.",
     None, [], None, "artifact-language"),
    ("artifact-language", "team", "Artifact language", "Langue des artefacts",
     "Write plans in your team's language.", None, None, [], None, "release-notes"),
    ("release-notes", "team", "Release notes and CHANGELOG", "Notes de version et CHANGELOG",
     "Release notes written from the plans.", None, None, [], None, "custom-schemas"),
    ("custom-schemas", "team", "Custom schemas", "Schémas personnalisés",
     "Your own planning steps.", None, None, [], None, "cli"),
]

APPENDICES = [
    ("Appendices", "Annexes", ["Command-line interface", "Configuration", "File formats", "JSON output", "Error codes and fixes", "Glossary", "FAQ", "An annotated example plan"]),
    ("Contributing", "Contribuer", ["Origins", "Contributing to Compagnon", "Architecture", "Reliability of the roles"]),
]

TITLES = {p[0]: p[2] for p in PAGES}
TITLES["cli"] = "Command-line interface"
STEP = {s[0]: s for s in STEPS}

e = html.escape


def toc():
    out = []
    for sid, n, en, fr, color in STEPS:
        items = "".join(
            f'<li><a href="#{p[0]}">{e(p[2])}</a></li>' for p in PAGES if p[1] == sid
        )
        out.append(
            f'<li class="step" style="--c:{color}"><span class="step-n">{n}</span>'
            f'<span class="step-t">{e(en)} <i lang="fr">· {e(fr)}</i></span><ol>{items}</ol></li>'
        )
    for en, fr, names in APPENDICES:
        items = "".join(f"<li>{e(x)}</li>" for x in names)
        out.append(f'<li class="appendix"><span class="step-t">{e(en)} <i lang="fr">· {e(fr)}</i></span><ul>{items}</ul></li>')
    return "\n".join(out)


def card(p):
    pid, sid, en, fr, purpose, diagram, example, folded, skip, nxt = p
    s = STEP[sid]
    parts = [
        f'<article class="card" id="{pid}" tabindex="-1" style="--c:{s[4]}" aria-labelledby="{pid}-t">',
        f'<p class="eyebrow">Step {s[1]} · {e(s[2])}</p>',
        f'<h2 id="{pid}-t">{e(en)}</h2><p class="fr" lang="fr">{e(fr)}</p>',
    ]
    if skip:
        parts.append(f'<p class="skip-step"><a href="#{skip[1]}">{e(skip[0])}</a></p>')
    parts.append(f'<p class="purpose">{e(purpose)}</p>')
    shown = []
    if diagram:
        shown.append(f'<li><b>Diagram</b> {e(diagram)}</li>')
    if example:
        shown.append(f'<li><b>Example</b> {e(example)}</li>')
    if shown:
        parts.append('<div class="layer"><h3>Everyone sees</h3><ul>' + "".join(shown) + "</ul></div>")
    if folded:
        parts.append(
            '<div class="layer folded"><h3>Folded, for experts — “Under the hood”</h3><ul>'
            + "".join(f"<li>{e(x)}</li>" for x in folded)
            + "</ul></div>"
        )
    target = TITLES[nxt]
    href = f"#{nxt}" if nxt != "cli" else "#appendices"
    parts.append(f'<p class="next"><span>Next step</span> <a href="{href}">{e(target)} <span aria-hidden="true">→</span></a></p>')
    parts.append("</article>")
    return "\n".join(parts)


CSS = """
:root {
  --font: "IBM Plex Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Arial, sans-serif;
  --mono: ui-monospace, "SF Mono", Menlo, Consolas, monospace;
  --bg: #ffffff; --fg: #2b3f47; --heading: #00263a; --muted: #4f6670; --surface: #f4f8f9;
  --line: #dce5e8; --link: #c4361b; --focus: #c4361b; --skip-bg: #fff4f1;
  color-scheme: light;
}
@media (prefers-color-scheme: dark) { :root:not([data-theme="light"]) {
  --bg: #0b1c24; --fg: #d5e2e7; --heading: #ffffff; --muted: #a3b8c0; --surface: #10262f;
  --line: #22404c; --link: #ff8a70; --focus: #ffab03; --skip-bg: #2a1a16; color-scheme: dark; } }
:root[data-theme="dark"] {
  --bg: #0b1c24; --fg: #d5e2e7; --heading: #ffffff; --muted: #a3b8c0; --surface: #10262f;
  --line: #22404c; --link: #ff8a70; --focus: #ffab03; --skip-bg: #2a1a16; color-scheme: dark; }
* { box-sizing: border-box; }
body { margin: 0; background: var(--bg); color: var(--fg); font: 400 15px/1.55 var(--font); }
a { color: var(--link); }
:focus-visible { outline: 3px solid var(--focus); outline-offset: 2px; border-radius: 6px; }
.bar { position: sticky; top: env(safe-area-inset-top, 0px); z-index: 5; background: #1a1a1a; color: #fff; font: 500 13px/1 var(--mono);
  padding: 8px max(16px, env(safe-area-inset-right, 0px)) 8px max(16px, env(safe-area-inset-left, 0px)); display: flex; flex-wrap: wrap; gap: 8px; align-items: center; }
body::before { content: ""; position: fixed; inset: 0 0 auto; height: env(safe-area-inset-top, 0px); z-index: 6; background: #1a1a1a; }
.bar span { margin-right: auto; opacity: .8; }
.bar button { font: inherit; color: #fff; background: #333; border: 1px solid #555; border-radius: 6px; padding: 6px 10px; cursor: pointer; }
.bar button[aria-pressed="true"] { background: #ffab03; color: #1a1a1a; border-color: #ffab03; }
.layout { display: grid; grid-template-columns: 300px minmax(0, 1fr); gap: 32px; max-width: 1200px; margin: 0 auto;
  padding: 24px max(16px, env(safe-area-inset-right, 0px)) 80px max(16px, env(safe-area-inset-left, 0px)); }
nav h1 { font-size: 20px; color: var(--heading); margin: 0 0 4px; }
nav > p { margin: 0 0 16px; color: var(--muted); font-size: 13px; }
.toc { list-style: none; margin: 0; padding: 0; display: grid; gap: 14px; position: sticky; top: 56px; }
.toc ol, .toc ul { margin: 6px 0 0 30px; padding: 0; display: grid; gap: 2px; font-size: 14px; }
.step-n { display: inline-grid; place-items: center; width: 22px; height: 22px; border-radius: 50%; background: var(--c); color: #1a0a05; font: 700 12px/1 var(--mono); margin-right: 8px; }
.step-t { font: 600 12px/1.4 var(--mono); letter-spacing: .06em; text-transform: uppercase; color: var(--heading); }
.step-t i { font-style: normal; color: var(--muted); text-transform: none; letter-spacing: 0; }
.appendix { opacity: .6; }
.appendix ul { list-style: none; margin-left: 0; color: var(--muted); }
main { display: grid; gap: 18px; min-width: 0; }
.intro { border: 1px dashed var(--line); border-radius: 14px; padding: 16px 18px; color: var(--muted); }
.intro strong { color: var(--heading); }
.card { overflow-wrap: anywhere; position: relative; border: 1px solid var(--line); border-radius: 14px; padding: 20px 22px; background: var(--bg); display: grid; gap: 10px; scroll-margin-top: 60px; }
.card::before { content: ""; position: absolute; left: 18px; right: 18px; top: -1px; height: 3px; border-radius: 0 0 3px 3px; background: var(--c); }
.card:focus { outline: 3px solid var(--focus); outline-offset: 2px; }
.eyebrow { margin: 0; font: 500 12px/1 var(--mono); letter-spacing: .06em; text-transform: uppercase; color: var(--muted); }
.card h2 { margin: 0; font-size: 22px; color: var(--heading); }
.fr { margin: -6px 0 0; color: var(--muted); font-size: 14px; }
.purpose { margin: 0; font-size: 16px; }
.skip-step { margin: 0; }
.skip-step a { display: inline-flex; align-items: center; gap: 10px; padding: 8px 14px; border: 1.5px solid var(--link); border-radius: 999px; text-decoration: none; font-weight: 600; font-size: 14px; }
.skip-step a::after { content: ""; width: 7px; height: 7px; border-top: 2px solid currentColor; border-right: 2px solid currentColor; transform: rotate(45deg); }
.skip-step a:hover, .skip-step a:focus-visible { background: var(--skip-bg); }
.layer { border-radius: 10px; padding: 10px 14px; background: var(--surface); }
.layer h3 { margin: 0 0 6px; font: 600 12px/1.3 var(--mono); letter-spacing: .05em; text-transform: uppercase; color: var(--heading); }
.layer ul { margin: 0; padding-left: 18px; display: grid; gap: 6px; }
.layer b { display: inline-block; min-width: 70px; color: var(--heading); }
.folded { background: transparent; border: 1px dashed var(--line); }
.folded ul { color: var(--muted); }
.next { margin: 4px 0 0; display: flex; flex-wrap: wrap; gap: 10px; align-items: baseline; border-top: 1px solid var(--line); padding-top: 10px; }
.next span:first-child { font: 600 12px/1 var(--mono); text-transform: uppercase; letter-spacing: .06em; color: var(--muted); }
.next a { font-weight: 600; }
.end { border-style: dashed; }
@media (max-width: 860px) { .layout { grid-template-columns: minmax(0, 1fr); } .toc { position: static; } }
"""

SCRIPT = """
(function () {
  function focusTarget() {
    var id = decodeURIComponent(location.hash.slice(1));
    var el = id && document.getElementById(id);
    if (el) { el.focus({ preventScroll: false }); }
  }
  window.addEventListener('hashchange', focusTarget);
  var root = document.documentElement;
  document.querySelectorAll('[data-theme-set]').forEach(function (b) {
    b.addEventListener('click', function () {
      root.setAttribute('data-theme', b.dataset.themeSet);
      document.querySelectorAll('[data-theme-set]').forEach(function (o) { o.setAttribute('aria-pressed', String(o === b)); });
    });
  });
})();
"""


def page():
    cards = "\n".join(card(p) for p in PAGES)
    return f"""<title>Compagnon Reading Tunnel</title>
<meta name="claude:viewport-fit-sides" content="cover">
<style>{CSS}</style>
<div class="bar" role="region" aria-label="Mockup controls"><span>Mockup of the reading tunnel — not the live documentation</span>
<button type="button" id="t-light" data-theme-set="light" aria-pressed="false">Light</button>
<button type="button" id="t-dark" data-theme-set="dark" aria-pressed="false">Dark</button></div>
<div class="layout">
<nav aria-label="Table of contents">
<h1>Compagnon documentation</h1>
<p>Five steps, read in order. Click a page, or follow “Next step”.</p>
<ol class="toc">
{toc()}
</ol>
</nav>
<main>
<p class="intro"><strong>How to read this mockup.</strong> Each card is one page of the tunnel. “Everyone sees” is the beginner layer: short text, a diagram and a concrete example. “Folded, for experts” lists what sits in closed “Under the hood” blocks. A round button lets readers skip a page that is not for them. “Next step” is where every page leads.</p>
{cards}
<article class="card end" id="appendices" tabindex="-1" style="--c:#dce5e8" aria-labelledby="appendices-t">
<p class="eyebrow">After the tunnel</p>
<h2 id="appendices-t">Appendices</h2>
<p class="purpose">Looked up when needed, not read in order: the command-line interface, the configuration, file formats, JSON output, error codes and fixes, the glossary, the FAQ, an annotated example plan. Then Contributing: origins, contributing, architecture, reliability of the roles.</p>
<p class="next"><span>Back to</span> <a href="#introduction">Compagnon in two minutes <span aria-hidden="true">↑</span></a></p>
</article>
</main>
</div>
<script>{SCRIPT}</script>
"""


(HERE / "index.html").write_text(page())
print("wrote", HERE / "index.html")
