"""Version 3 of the tunnel mockup, strictly on the Mairistem design system.

Run: python3 build_v3.py — writes index.html (the previous version is kept as
index-v2.html). Values come from the Mairistem Design System (tokens.css,
components: SiteHeader, ProductHero, Eyebrow, Button, LinkArrow, PersonaCard,
StepCard, Accordion, DarkCard, Pastille, SiteFooter); the brand symbols
(plein-*.svg) are the official exports. No gradient, no blur, IBM Plex Sans
only — a monospace face only inside the Claude Code capture and code.
"""
import html
import pathlib
import re

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[4]
e = html.escape


def figure(path):
    md = (ROOT / path).read_text()
    return re.search(r'<figure class="diagram">.*?</figure>', md, re.S).group(0)


def symbol(name, cls):
    svg = (HERE / name).read_text()
    svg = re.sub(r"<\?xml[^>]*\?>", "", svg).replace('id="Calque_2" ', "").replace(' id="Calque_1-2"', "")
    return svg.replace("<svg ", f'<svg class="{cls}" aria-hidden="true" focusable="false" ', 1)


logo = (ROOT / "docs/assets/compagnon-logo.svg").read_text()
inner = re.search(r"<title[^>]*>.*?</title>(.*)</svg>", logo, re.S).group(1)
inner = inner.replace('fill="#000000"', 'style="fill:var(--logo-ink)"').replace('stroke="#004c66"', 'style="stroke:var(--logo-c)"')
SPRITE = f'<svg width="0" height="0" aria-hidden="true" style="position:absolute"><symbol id="cp-logo" viewBox="0 0 1188 250">{inner}</symbol></svg>'

# The five steps; colors in the brand order (corail, vert, jaune, bleu profond) and a neutral.
STEPS = [
    ("Découvrir", "Discover", ["Compagnon en deux minutes"], "#ff5331", "Ce que fait Compagnon, et qui décide."),
    ("Démarrer", "Get started", ["Installation", "Votre première modification"], "#00c39b", "Installer, puis une première modification en dix minutes."),
    ("Au quotidien", "Every day", ["Parler à Claude", "Partir d'un ticket Jira", "Relire un plan", "Approuver un plan sur GitHub", "Réaliser et ranger"], "#ffab03", "Demander, relire, approuver, réaliser : la boucle de tous les jours."),
    ("Comprendre", "Understand", ["Comment ça marche", "Concepts", "Décisions", "Rôles", "Compte rendu de relecture"], "#004c66", "Ce qui se passe derrière chaque étape."),
    ("Pour l'équipe", "For your team", ["Adopter sur un projet existant", "Déployer pour une équipe", "Sources héritées", "Langue des artefacts", "Notes de version", "Schémas personnalisés"], "#8aa3ac", "Installer Compagnon pour une équipe et ses conventions."),
]


def cc(title, lines, hint):
    """A Claude Code session, shown as the terminal shows it — an application
    capture, framed as the design system frames captures."""
    out = []
    for line in lines:
        kind = line[0]
        if kind == "user":
            out.append(f'<div class="cc-user"><span aria-hidden="true">&gt;</span> {e(line[1])}</div>')
        elif kind == "say":
            out.append(f'<div class="cc-row"><span class="cc-dot" aria-hidden="true">⏺</span><span>{line[1]}</span></div>')
        elif kind == "tool":
            res = "".join(f'<div class="cc-res"><span aria-hidden="true">⎿</span> {r}</div>' for r in line[3])
            out.append(f'<div class="cc-row"><span class="cc-dot ok" aria-hidden="true">⏺</span><span><b>{e(line[1])}</b>({e(line[2])})</span></div>{res}')
    return f"""<figure class="capture" aria-label="Session Claude Code : {e(title)}">
<div class="cc"><div class="cc-body">{"".join(out)}</div>
<div class="cc-input" aria-hidden="true"><div class="box"><span>&gt;</span> <i>{e(hint)}</i></div></div>
<div class="cc-foot" aria-hidden="true">? for shortcuts</div></div>
<figcaption>Dans Claude Code, tel qu'il s'affiche.</figcaption></figure>"""


JIRA_SESSION = cc("préparer un plan pour PROJ-123", [
    ("user", "Prépare un plan pour PROJ-123"),
    ("say", "Je pars du ticket Jira, puis j'écris le plan."),
    ("tool", "atlassian - getJiraIssue (MCP)", 'issueIdOrKey: "PROJ-123"', ['Mode sombre du tableau de bord · Story · <span class="cc-y">In Progress</span>']),
    ("tool", "Write", "_compagnon/changes/mode-sombre/proposal.md", ["Wrote 41 lines to _compagnon/changes/mode-sombre/proposal.md"]),
    ("say", "Le plan est prêt dans <code>_compagnon/changes/mode-sombre/</code> : pourquoi, ce qui change, comment, et 7 tâches. Rien n'est construit avant votre accord — lisez d'abord <code>proposal.md</code>."),
], "Réalise-le")

FIRST_SESSION = cc("une première modification, de la demande au rangement", [
    ("user", "Ajoute un mode sombre qui suit le réglage du système"),
    ("tool", "Bash", "compagnon new change mode-sombre", ["Change `mode-sombre` created"]),
    ("say", "Le plan est prêt dans <code>_compagnon/changes/mode-sombre/</code>. Lisez d'abord <code>proposal.md</code>."),
    ("user", "Réalise-le"),
    ("tool", "Update", "src/theme.ts", ["Updated src/theme.ts with 18 additions and 2 removals"]),
    ("tool", "Bash", "npm test", ['<span class="cc-g">✓</span> 42 tests passed']),
    ("say", "7 tâches sur 7 faites · 2 remarques de relecture, rien à trancher."),
    ("user", "Range-le"),
    ("tool", "Bash", "compagnon archive --change mode-sombre", ["✓ Archived · moved to _compagnon/changes/archive/2026-11-27-mode-sombre"]),
], "Où en suis-je ?")

HOME_SESSION = cc("préparer un plan pour PROJ-123", [
    ("user", "Prépare un plan pour PROJ-123"),
    ("tool", "atlassian - getJiraIssue (MCP)", 'issueIdOrKey: "PROJ-123"', ['Mode sombre du tableau de bord · <span class="cc-y">In Progress</span>']),
    ("tool", "Write", "_compagnon/changes/mode-sombre/proposal.md", ["Wrote 41 lines"]),
    ("say", "Le plan est prêt pour votre relecture. Rien n'est construit avant votre accord."),
], "Réalise-le")


CUR = ' aria-current="page"'


def sidebar(cur_step, cur_page):
    out = ['<nav class="steps-nav" aria-label="Les cinq étapes"><ol>']
    for i, (fr, en, pages, color, _) in enumerate(STEPS):
        state = "done" if i < cur_step else "current" if i == cur_step else "todo"
        mark = "✓" if state == "done" else str(i + 1)
        items = ""
        if i == cur_step:
            items = '<ol class="pages">' + "".join(
                f'<li class="{"here" if j == cur_page else ""}"><a href="#"{CUR if j == cur_page else ""}>{e(p)}</a></li>'
                for j, p in enumerate(pages)
            ) + "</ol>"
        out.append(f'<li class="st {state}" style="--c:{color}"><span class="pastille" aria-hidden="true">{mark}</span><span class="st-name">{e(fr)}</span>{items}</li>')
    out.append('</ol><p class="annexes">Annexes</p><ul class="annexes-list"><li>Référence des commandes</li><li>Configuration</li><li>Codes d\'erreur</li><li>Glossaire</li><li>FAQ</li></ul></nav>')
    return "".join(out)


def progress(cur_step, cur_page):
    fr, _, pages, color, _ = STEPS[cur_step]
    segs = "".join(
        f'<span class="seg" style="--c:{STEPS[i][3]}"><span style="width:{100 if i < cur_step else (int(100 * (cur_page + 1) / len(pages)) if i == cur_step else 0)}%"></span></span>'
        for i in range(5)
    )
    return f'<div class="progress" role="img" aria-label="Étape {cur_step + 1} sur 5, {e(fr)}, page {cur_page + 1} sur {len(pages)}">{segs}</div>'


def page_hero(cur_step, cur_page, title, chapo, skip, shape):
    fr, _, pages, color, _ = STEPS[cur_step]
    return f"""<section class="hero hero-page">{symbol(shape, "filigrane")}
<p class="crumb"><a href="#">Documentation</a> <span aria-hidden="true">›</span> <a href="#">{e(fr)}</a> <span aria-hidden="true">›</span> <span>{e(title)}</span></p>
{progress(cur_step, cur_page)}
<p class="eyebrow corail"><span class="sq" aria-hidden="true"></span>Étape {cur_step + 1} sur 5 · {e(fr)} · page {cur_page + 1} sur {len(pages)}</p>
<h1>{e(title)}</h1>
<p class="chapo">{e(chapo)}</p>
<p class="skip"><a class="link-arrow bleu" href="#">{e(skip)}<span aria-hidden="true">→</span></a></p>
</section>"""


def next_card(step_label, title, text):
    return f"""<nav class="dark-card" aria-label="Étape suivante"><p class="eyebrow on-dark">Étape suivante · {e(step_label)}</p>
<h3>{e(title)}</h3><p>{e(text)}</p><a class="link-arrow jaune" href="#">Continuer<span aria-hidden="true">→</span></a></nav>"""


def hood(question, answer, open_=False):
    return f'<details class="acc"{" open" if open_ else ""}><summary><span>{e(question)}</span><span class="sign" aria-hidden="true"></span></summary><p>{answer}</p></details>'


JIRA = f"""<article class="doc" id="jira" data-view>
{page_hero(2, 1, "Partir d'un ticket Jira", "Votre équipe décrit déjà le travail dans Jira. Nommez le ticket dans votre demande : Claude part du ticket, et le plan dit d'où il vient.", "Pas de Jira ? Passez à Relire un plan", "plein-01.svg")}
<div class="doc-body">
<ol class="checks"><li><b>Vous dites</b> « Prépare un plan pour PROJ-123 ».</li><li><b>Claude lit le ticket</b> une fois, sans rien changer dans Jira.</li><li><b>Vous obtenez un plan qui le cite</b> sur sa première ligne.</li></ol>
{figure("docs/fr/src/guides/mcp.md")}
<section class="example" aria-label="Exemple"><p class="eyebrow vert"><span class="sq" aria-hidden="true"></span>Exemple</p>{JIRA_SESSION}</section>
<div class="accs">{hood("Sous le capot : le mettre en place", "Compagnon a besoin du nom de l'outil de lecture de ticket de votre connecteur Jira, dans <code>_compagnon/config.yaml</code> ; <code>compagnon init</code> le remplit s'il trouve un serveur Jira ou Atlassian dans votre configuration de Claude Code.")}{hood("Sous le capot : ce que Claude peut faire avec Jira", "Une lecture du premier ticket nommé. Jamais de recherche, de commentaire ni de changement de statut.")}</div>
{next_card("page 3 sur 5", "Relire un plan", "Ce qu'il faut vérifier dans un plan avant de dire oui, fichier par fichier.")}
<p class="prev"><a class="link-arrow gris" href="#">Précédent : Parler à Claude<span aria-hidden="true">→</span></a></p>
</div></article>"""

FIRST = f"""<article class="doc" id="first" data-view hidden>
{page_hero(1, 1, "Votre première modification", "Une modification complète en dix minutes : vous demandez, vous lisez le plan, Claude le réalise, et la modification est rangée.", "Déjà fait une modification ? Passez à Parler à Claude", "plein-05.svg")}
<div class="doc-body">
{figure("docs/fr/src/quickstart.md")}
<section class="example" aria-label="Exemple"><p class="eyebrow vert"><span class="sq" aria-hidden="true"></span>Exemple</p>{FIRST_SESSION}</section>
<div class="accs">{hood("Sous le capot : les commandes derrière chaque demande", "<code>/compagnon-propose</code>, <code>/compagnon-apply</code>, <code>/compagnon-archive</code> — les mêmes étapes, tapées comme des commandes.")}</div>
{next_card("étape 3 · Au quotidien", "Parler à Claude", "Ce qu'il faut dire à Claude à chaque étape, avec vos mots.")}
<p class="prev"><a class="link-arrow gris" href="#">Précédent : Installation<span aria-hidden="true">→</span></a></p>
</div></article>"""

PERSONAS = "".join(
    f'<li class="persona" style="--c:{c}"><span class="rule"></span><div><p class="eyebrow gris">Étape {i + 1} · {len(pages)} page{"s" if len(pages) > 1 else ""}</p>'
    f'<h3>{e(fr)}</h3><p>{e(lead)}</p><a class="link-arrow corail" href="#" data-go="{"first" if i == 1 else "jira" if i == 2 else "overview"}">Commencer ici<span aria-hidden="true">→</span></a></div></li>'
    for i, (fr, en, pages, c, lead) in enumerate(STEPS)
)

HOME = f"""<div class="home" id="home" data-home hidden>
<section class="hero hero-home">{symbol("plein-03.svg", "filigrane big")}
<div class="hero-grid"><div class="hero-text">
<p class="eyebrow corail"><span class="sq" aria-hidden="true"></span>Libre · pour Claude Code</p>
<h1>Planifiez avec Claude avant qu'il ne code.</h1>
<p class="chapo">Avant d'écrire la moindre ligne, Claude met par écrit ce qu'il va construire : un plan court que vous lisez, corrigez et validez. Ensuite, il construit exactement cela.</p>
<p class="btns"><a class="btn primary" href="#" data-go="first">Lire la documentation</a><a class="btn secondary" href="#">Voir sur GitHub</a></p>
<ul class="proofs"><li>Rien n'est construit sans votre accord</li><li>Le plan reste avec le code</li><li>Partir d'un ticket Jira</li></ul>
</div>{HOME_SESSION}</div></section>
<section class="statbar" aria-label="En chiffres"><div><b>5</b><span>étapes de lecture</span></div><div><b>4</b><span>fichiers courts par plan</span></div><div><b>0</b><span>ligne de code avant votre accord</span></div></section>
<section class="section"><p class="eyebrow gris">La documentation en cinq étapes</p><h2>Lisez-la dans l'ordre, ou entrez à votre étape.</h2><ol class="personas">{PERSONAS}</ol></section>
</div>"""

OVERVIEW = f"""<article class="doc" id="overview" data-view hidden>
<section class="hero hero-page">{symbol("plein-05.svg", "filigrane")}
<p class="eyebrow corail"><span class="sq" aria-hidden="true"></span>Le tunnel de lecture</p><h1>Cinq étapes, lues dans l'ordre.</h1>
<p class="chapo">Les débutants peuvent s'arrêter après l'étape 3 ; les experts continuent. Les annexes se consultent au besoin.</p></section>
<div class="doc-body"><ol class="personas cols">{PERSONAS}</ol></div></article>"""

CSS = """
/* Mairistem Design System — tokens (tokens.css) */
:root {
  --color-corail: #ff5331; --color-bleu-profond: #004c66; --color-vert: #00c39b; --color-jaune: #ffab03;
  --color-neutral-900: #00394d; --color-neutral-800: #40565e; --color-neutral-700: #5f767e; --color-neutral-600: #6e858d;
  --color-neutral-500: #8aa3ac; --color-neutral-400: #9cbac4; --color-neutral-300: #b9cdd3; --color-neutral-200: #dce5e8;
  --color-neutral-150: #e3eaec; --color-neutral-100: #e7edef; --color-neutral-50: #f4f8f9;
  --color-corail-dark: #c63a1c; --color-corail-texte: #c4361b; --color-corail-texte-hover: #a82d15; --color-vert-ink: #00806a;
  --font-sans: "IBM Plex Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  --radius-xs: 2px; --radius-sm: 6px; --radius-md: 12px; --radius-lg: 14px; --radius-xl: 16px; --radius-pastille: 6px 6px 6px 0;
  --motion-base: .2s ease-in-out; --motion-fast: .18s ease-in-out;
  --shadow-cta: 0 6px 18px 0 rgba(255,83,49,.28); --shadow-cta-hover: 0 10px 22px 0 rgba(255,83,49,.34);
  --shadow-card-hover: 0 8px 18px 0 rgba(0,60,80,.13); --shadow-float: 0 8px 18px 0 rgba(0,60,80,.18);
  /* roles, light */
  --page: #ffffff; --soft: var(--color-neutral-50); --card: #ffffff; --strong: var(--color-bleu-profond); --body: var(--color-neutral-800);
  --muted: var(--color-neutral-700); --subtle: var(--color-neutral-600); --eyebrow: var(--color-neutral-500);
  --border-card: var(--color-neutral-150); --hairline: var(--color-neutral-100); --field: var(--color-neutral-200);
  --link: var(--color-corail-texte); --link-hover: var(--color-corail-texte-hover); --focus: var(--color-corail);
  --dark-deep: var(--color-neutral-900); --dark: var(--color-bleu-profond); --on-dark-muted: #9cbac4;
  --logo-ink: #000000; --logo-c: #004c66; --filigrane: .3; --dark-card: #00394d; --dark-card-border: transparent; --eyebrow-corail: #c63a1c;
  --dg-bg: #ffffff; --dg-box: #f4f8f9; --dg-file: #ffffff; --dg-ink: #00263a; --dg-muted: #4f6670; --dg-line: #567079;
  --dg-you: #e0421f; --dg-you-text: #c4361b; --dg-on-you: #1a0a05; --dg-claude: #004c66;
  color-scheme: light;
}
/* Dark theme, from the system's own dark surfaces (bleu profond, neutral-900) and its on-dark rules (white text,
   #9CBAC4 muted, yellow links, 14% white rules). */
@media (prefers-color-scheme: dark) { :root:not([data-theme="light"]) {
  --page: #00394d; --soft: #004c66; --card: #004c66; --dark-card: #004c66; --dark-card-border: rgba(255,255,255,.14); --eyebrow-corail: #ffab03; --strong: #ffffff; --body: #e7edef; --muted: #9cbac4; --subtle: #9cbac4;
  --eyebrow: #9cbac4; --border-card: rgba(255,255,255,.14); --hairline: rgba(255,255,255,.14); --field: rgba(255,255,255,.22);
  --link: #ffab03; --link-hover: #ffc24d; --dark-deep: #002a3a; --logo-ink: #ffffff; --logo-c: #ffffff; --filigrane: .18;
  --dg-bg: #00394d; --dg-box: #0b4a61; --dg-file: #00394d; --dg-ink: #f2f7f8; --dg-muted: #b4c8cf; --dg-line: #a7c6d1;
  --dg-you: #ff8a70; --dg-you-text: #ffa390; --dg-claude: #7cc8e2; color-scheme: dark; } }
:root[data-theme="dark"] {
  --page: #00394d; --soft: #004c66; --card: #004c66; --dark-card: #004c66; --dark-card-border: rgba(255,255,255,.14); --eyebrow-corail: #ffab03; --strong: #ffffff; --body: #e7edef; --muted: #9cbac4; --subtle: #9cbac4;
  --eyebrow: #9cbac4; --border-card: rgba(255,255,255,.14); --hairline: rgba(255,255,255,.14); --field: rgba(255,255,255,.22);
  --link: #ffab03; --link-hover: #ffc24d; --dark-deep: #002a3a; --logo-ink: #ffffff; --logo-c: #ffffff; --filigrane: .18;
  --dg-bg: #00394d; --dg-box: #0b4a61; --dg-file: #00394d; --dg-ink: #f2f7f8; --dg-muted: #b4c8cf; --dg-line: #a7c6d1;
  --dg-you: #ff8a70; --dg-you-text: #ffa390; --dg-claude: #7cc8e2; color-scheme: dark; }
* { box-sizing: border-box; }
[hidden] { display: none !important; }
body { margin: 0; background: var(--page); color: var(--body); font: 400 15.5px/23.25px var(--font-sans); -webkit-font-smoothing: antialiased; }
a { color: var(--link); text-decoration: none; }
a:hover { color: var(--link-hover); text-decoration: underline; }
:focus-visible { outline: 2px solid var(--focus); outline-offset: 2px; border-radius: 2px; }
code { font-family: ui-monospace, "SF Mono", Menlo, Consolas, monospace; font-size: .9em; background: var(--soft); border: 1px solid var(--hairline); border-radius: 4px; padding: 0 .3em; }
.gut { padding-left: max(16px, env(safe-area-inset-left, 0px)); padding-right: max(16px, env(safe-area-inset-right, 0px)); }

/* Mockup controls (not part of the design) */
.mock { display: flex; flex-wrap: wrap; gap: 6px; align-items: center; background: #1a1a1a; color: #fff; font: 500 12px/1 var(--font-sans); padding-block: 8px; }
.mock span { margin-right: auto; opacity: .75; }
.mock button { font: inherit; color: #fff; background: #333; border: 1px solid #555; border-radius: 4px; padding: 6px 9px; cursor: pointer; }
.mock button[aria-pressed="true"] { background: #ffab03; color: #1a1a1a; border-color: #ffab03; }

/* SiteHeader: top band (surface-dark-deep, 44px, 13.5px) + white band 86px; not sticky */
.topband { background: var(--dark-deep); color: var(--color-neutral-300); font-size: 13.5px; }
.topband .in { max-width: 1240px; margin: 0 auto; min-height: 44px; display: flex; align-items: center; justify-content: space-between; gap: 16px; flex-wrap: wrap; }
.topband b { color: #fff; font-weight: 600; }
.topband .pill { padding: 6px 14px; background: var(--color-corail); color: #fff; border-radius: 4px; font-weight: 600; }
.topband .pill:hover { text-decoration: none; color: #fff; }
.mainband { background: var(--page); border-bottom: 1px solid var(--border-card); }
.mainband .in { max-width: 1240px; margin: 0 auto; min-height: 86px; display: flex; align-items: center; justify-content: space-between; gap: 24px; flex-wrap: wrap; padding-block: 10px; }
.brand svg { height: 44px; width: auto; aspect-ratio: 1188/250; display: block; }
.navlinks { display: flex; gap: 6px; flex-wrap: wrap; }
.navlinks a { padding: 12px 11px; font-size: 15.5px; font-weight: 600; color: var(--strong); border-radius: var(--radius-sm); }
.navlinks a:hover, .navlinks a[aria-current] { background: #f1f6f7; text-decoration: none; color: var(--color-bleu-profond); }

/* Buttons */
.btns { display: flex; gap: 13.7px; flex-wrap: wrap; margin: 6px 0 0; }
.btn { font: 600 16px/1 var(--font-sans); border-radius: var(--radius-sm); padding: 15px 26px; display: inline-flex; align-items: center; border: 1.5px solid transparent; transition: all var(--motion-base); }
.btn:hover { text-decoration: none; transform: translateY(-1px); }
.btn.primary { background: var(--color-corail); color: #fff; box-shadow: var(--shadow-cta); }
.btn.primary:hover { background: #e8431f; color: #fff; box-shadow: var(--shadow-cta-hover); }
.btn.secondary { color: var(--strong); border-color: var(--strong); }
.btn.secondary:hover { background: var(--color-bleu-profond); color: #fff; border-color: var(--color-bleu-profond); }
.btn.sm { font-size: 15px; padding: 13px 22px; }

/* LinkArrow */
.link-arrow { font-weight: 600; font-size: 15px; display: inline-flex; align-items: center; gap: 6px; transition: all var(--motion-fast); }
.link-arrow:hover { text-decoration: none; transform: translateX(4px); }
.link-arrow.corail { color: var(--link); } .link-arrow.corail:hover { color: var(--link-hover); }
.link-arrow.bleu { color: var(--strong); } .link-arrow.bleu:hover { color: var(--link); }
.link-arrow.gris { color: var(--subtle); font-size: 14.5px; } .link-arrow.gris:hover { color: var(--link); }
.link-arrow.jaune { color: var(--color-jaune); } .link-arrow.jaune:hover { color: #ffc24d; }

/* Eyebrow */
.eyebrow { margin: 0; display: flex; align-items: center; gap: 10px; font: 600 13px/1 var(--font-sans); letter-spacing: 1.17px; text-transform: uppercase; color: var(--eyebrow); }
.eyebrow.corail { color: var(--eyebrow-corail); letter-spacing: 1.04px; }
.eyebrow.vert { color: var(--color-vert-ink); letter-spacing: .675px; font-size: 13.5px; }
.eyebrow.on-dark { color: var(--on-dark-muted); letter-spacing: 1.04px; }
.eyebrow .sq { width: 10px; height: 10px; border-radius: var(--radius-xs); background: var(--color-corail); flex: none; }
.eyebrow.vert .sq { background: var(--color-vert); }

/* ProductHero: surface-soft, symbol at 30%, h1 52/600/-1.3, chapo 16/26 muted */
.hero { position: relative; overflow: hidden; background: var(--soft); }
.filigrane { position: absolute; right: 40px; top: 20px; width: 200px; height: auto; opacity: var(--filigrane); pointer-events: none; }
.filigrane.big { width: 300px; right: -30px; top: auto; bottom: -60px; }
.hero h1 { margin: 0; font-size: clamp(34px, 5vw, 52px); line-height: 1.06; font-weight: 600; letter-spacing: -1.3px; color: var(--strong); text-wrap: balance; }
.chapo { margin: 0; font-size: 16px; line-height: 26px; color: var(--muted); max-width: 560px; }
.hero-home { padding: 56px 0 64px; }
.hero-grid { position: relative; max-width: 1240px; margin: 0 auto; display: grid; grid-template-columns: minmax(0, 1fr) minmax(0, 1fr); gap: 56px; align-items: center; padding-inline: max(16px, env(safe-area-inset-left, 0px)) max(16px, env(safe-area-inset-right, 0px)); }
.hero-text { display: grid; gap: 18px; }
.proofs { list-style: none; margin: 8px 0 0; padding: 0; display: flex; flex-wrap: wrap; gap: 8px 18px; font-size: 14.5px; color: var(--muted); }
.proofs li::before { content: "✓"; color: var(--color-vert-ink); font-weight: 700; margin-right: 8px; }
.hero-page { border-radius: var(--radius-xl); padding: 28px 32px 34px; display: grid; gap: 16px; }
.hero-page > *:not(.filigrane) { position: relative; }
.crumb { margin: 0; font-size: 13.5px; color: var(--subtle); display: flex; gap: 8px; flex-wrap: wrap; }
.crumb a { color: var(--subtle); }
.progress { display: grid; grid-template-columns: repeat(5, 1fr); gap: 6px; max-width: 520px; }
.seg { height: 5px; border-radius: var(--radius-xs); background: var(--hairline); overflow: hidden; }
.seg span { display: block; height: 100%; background: var(--c); }
.skip { margin: 2px 0 0; }

/* StatBar: bleu profond band */
.statbar { background: var(--color-bleu-profond); color: #fff; display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 0; }
.statbar div { padding: 26px 32px; border-left: 1px solid rgba(255,255,255,.14); display: grid; gap: 4px; }
.statbar div:first-child { border-left: 0; }
.statbar b { font-size: 40px; line-height: 40px; font-weight: 400; letter-spacing: -.8px; }
.statbar span { color: #9cbac4; font-size: 14.5px; }

/* Sections and PersonaCards (5px colored rule, no shadow; hover lift -3px) */
.section { max-width: 1240px; margin: 0 auto; padding: 76px max(16px, env(safe-area-inset-right, 0px)) 84px max(16px, env(safe-area-inset-left, 0px)); display: grid; gap: 14px; }
.section h2 { margin: 0 0 18px; font-size: clamp(28px, 3.6vw, 38px); line-height: 1.15; font-weight: 600; letter-spacing: -.76px; color: var(--strong); }
.personas { list-style: none; margin: 0; padding: 0; display: grid; grid-template-columns: repeat(5, minmax(0, 1fr)); gap: 16px; }
.personas.cols { grid-template-columns: repeat(auto-fill, minmax(220px, 1fr)); }
.persona { border: 1px solid var(--border-card); border-radius: var(--radius-md); overflow: hidden; background: var(--card); display: flex; flex-direction: column; transition: all var(--motion-base); }
.persona .rule { height: 5px; background: var(--c); }
.persona > div { padding: 26px 24px 24px; display: flex; flex-direction: column; gap: 12px; flex: 1; }
.persona h3 { margin: 0; font-size: 16.5px; line-height: 22px; font-weight: 600; color: var(--strong); }
.persona p:not(.eyebrow) { margin: 0; font-size: 14.5px; line-height: 22px; color: var(--muted); flex: 1; }
.persona:hover { transform: translateY(-3px); box-shadow: var(--shadow-card-hover); border-color: var(--c); }

/* Documentation layout */
.shell { max-width: 1240px; margin: 0 auto; display: grid; grid-template-columns: 260px minmax(0, 1fr); gap: 40px; padding: 32px max(16px, env(safe-area-inset-right, 0px)) 84px max(16px, env(safe-area-inset-left, 0px)); }
.steps-nav ol { list-style: none; margin: 0; padding: 0; display: grid; gap: 6px; }
.st { display: grid; grid-template-columns: 26px 1fr; gap: 4px 12px; align-items: center; }
.pastille { width: 26px; height: 26px; border-radius: var(--radius-pastille); display: grid; place-items: center; font-size: 13px; font-weight: 600; color: var(--subtle); background: var(--soft); border: 1px solid var(--field); }
.st.done .pastille { background: var(--c); border-color: var(--c); color: #fff; }
.st.current .pastille { background: var(--c); border-color: var(--c); color: #fff; }
.st-name { font-size: 15.5px; font-weight: 600; color: var(--strong); padding: 8px 0; }
.st.todo .st-name { color: var(--subtle); font-weight: 400; }
.pages { grid-column: 2; display: grid; gap: 2px; margin: 0 0 8px; }
.pages a { display: block; padding: 7px 10px; font-size: 14.5px; color: var(--body); border-left: 2px solid var(--hairline); }
.pages a:hover { color: var(--link); text-decoration: none; }
.pages .here a { color: var(--strong); font-weight: 600; border-left-color: var(--color-corail); background: var(--soft); }
.annexes { margin: 26px 0 8px; font: 600 13px/1 var(--font-sans); letter-spacing: 1.17px; text-transform: uppercase; color: var(--eyebrow); border-top: 1px solid var(--hairline); padding-top: 18px; }
.annexes-list { list-style: none; margin: 0; padding: 0; display: grid; gap: 6px; font-size: 14.5px; color: var(--subtle); }
.doc { display: grid; gap: 28px; min-width: 0; }
.doc-body { display: grid; gap: 28px; max-width: 760px; }
.checks { list-style: none; margin: 0; padding: 0; display: grid; gap: 10px; font-size: 16px; }
.checks li { padding-left: 28px; position: relative; }
.checks li::before { content: "✓"; position: absolute; left: 0; color: var(--color-vert-ink); font-weight: 700; }
.checks b { color: var(--strong); font-weight: 600; }

/* Diagrams (unchanged tokens) */
figure.diagram { margin: 0; display: grid; gap: 10px; min-width: 0; }
figure.diagram > .scroll { overflow-x: auto; border: 1px solid var(--border-card); border-radius: var(--radius-md); background: var(--dg-bg); padding: 14px; }
figure.diagram svg { display: block; width: 100%; height: auto; min-width: 560px; }
figure.diagram figcaption, .capture figcaption { color: var(--muted); font-size: 13.5px; line-height: 20px; }
figure.diagram text { font-family: var(--font-sans); }
figure.diagram .mono { font-family: ui-monospace, "SF Mono", Menlo, Consolas, monospace; }

/* Example: an application capture, framed white, 1px #E3EAEC, radius 12, shown as it is */
.example { display: grid; gap: 14px; }
.capture { margin: 0; display: grid; gap: 10px; }
.capture .cc { background: #fff; border: 1px solid var(--color-neutral-150); border-radius: var(--radius-md); padding: 8px; }
.cc-body, .cc-input, .cc-foot { background: #1e1e1e; color: #e6e6e6; font: 13px/1.6 ui-monospace, "SF Mono", Menlo, Consolas, monospace; }
.cc-body { border-radius: 6px 6px 0 0; padding: 14px 16px 6px; display: grid; gap: 8px; overflow-x: auto; }
.cc-user { color: #9a9a9a; background: #2b2b2b; border-radius: 4px; padding: 4px 8px; }
.cc-row { display: grid; grid-template-columns: 16px 1fr; gap: 6px; }
.cc-dot.ok { color: #4eba65; }
.cc-row b { color: #fff; }
.cc-res { padding-left: 22px; color: #9a9a9a; }
.cc code { background: none; border: 0; padding: 0; color: #b1b9f9; font-size: 1em; }
.cc-y { color: #e5c07b; } .cc-g { color: #4eba65; }
.cc-input { padding: 8px 12px 0; } .cc-input .box { border: 1px solid #5a5a5a; border-radius: 6px; padding: 6px 10px; } .cc-input i { font-style: normal; color: #6b6b6b; }
.cc-foot { border-radius: 0 0 6px 6px; padding: 6px 14px 10px; color: #6b6b6b; font-size: 11.5px; }

/* Accordion (FAQ): hairlines, question 17/600 bleu, + / − */
.accs { border-top: 1px solid var(--hairline); }
.acc { border-bottom: 1px solid var(--hairline); padding: 0; }
.acc summary { list-style: none; cursor: pointer; display: flex; justify-content: space-between; align-items: center; gap: 24px; padding: 20px 0; font-size: 17px; line-height: 23.8px; font-weight: 600; color: var(--strong); }
.acc summary::-webkit-details-marker { display: none; }
.acc .sign::before { content: "+"; font-size: 22px; font-weight: 400; color: var(--color-neutral-300); }
.acc[open] .sign::before { content: "−"; color: var(--color-corail); }
.acc p { margin: 0 0 20px; font-size: 15.5px; line-height: 25.58px; color: var(--muted); max-width: 600px; }

/* DarkCard: surface-dark-deep, radius 12, padding 32 */
.dark-card { background: var(--dark-card); border: 1px solid var(--dark-card-border); border-radius: var(--radius-md); padding: 32px; display: grid; gap: 10px; }
.dark-card h3 { margin: 0; font-size: 20px; line-height: 25px; font-weight: 600; color: #fff; }
.dark-card p:not(.eyebrow) { margin: 0; font-size: 14.5px; line-height: 23.2px; color: #9cbac4; }
.dark-card .link-arrow { margin-top: 6px; }
.prev { margin: -12px 0 0; }

/* SiteFooter */
.footer { background: var(--color-neutral-900); color: #9cbac4; font-size: 14.5px; }
.footer .in { max-width: 1240px; margin: 0 auto; padding-block: 40px; display: flex; justify-content: space-between; gap: 18px; flex-wrap: wrap; }
.footer a { color: #fff; }

@media (prefers-reduced-motion: reduce) { * { transition: none !important; } }
@media (max-width: 1100px) { .personas { grid-template-columns: repeat(2, minmax(0, 1fr)); } .hero-grid { grid-template-columns: minmax(0, 1fr); } }
@media (max-width: 860px) { .shell { grid-template-columns: minmax(0, 1fr); } .steps-nav { display: none; } .navlinks { display: none; } .statbar { grid-template-columns: minmax(0, 1fr); } .statbar div { border-left: 0; border-top: 1px solid rgba(255,255,255,.14); } }
@media (max-width: 560px) { .personas { grid-template-columns: minmax(0, 1fr); } .hero-page { padding: 22px 18px 26px; } .filigrane { width: 120px; right: 10px; } }
"""

SCRIPT = """
(function () {
  var root = document.documentElement;
  function show(v) {
    document.querySelector('[data-home]').hidden = v !== 'home';
    document.querySelector('[data-shell]').hidden = v === 'home';
    document.querySelectorAll('[data-view]').forEach(function (p) { p.hidden = p.id !== v; });
    document.querySelectorAll('[data-show]').forEach(function (b) { b.setAttribute('aria-pressed', String(b.dataset.show === v)); });
    window.scrollTo(0, 0);
  }
  document.querySelectorAll('[data-show]').forEach(function (b) { b.addEventListener('click', function () { show(b.dataset.show); }); });
  document.querySelectorAll('[data-go]').forEach(function (a) { a.addEventListener('click', function (ev) { ev.preventDefault(); show(a.dataset.go); }); });
  document.querySelectorAll('[data-theme-set]').forEach(function (b) { b.addEventListener('click', function () { root.setAttribute('data-theme', b.dataset.themeSet); }); });
  show('home');
})();
"""


def page():
    return f"""<title>Compagnon Reading Tunnel</title>
<meta name="claude:viewport-fit-sides" content="cover">
<link rel="preconnect" href="https://fonts.googleapis.com"><link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=IBM+Plex+Sans:wght@400;450;500;600;700&display=swap">
<style>{CSS}</style>
{SPRITE}
<div class="mock gut" role="group" aria-label="Vues de la maquette"><span>Maquette — pas le site en ligne</span>
<button type="button" id="v-home" data-show="home" aria-pressed="true">Accueil</button>
<button type="button" id="v-jira" data-show="jira" aria-pressed="false">Page : Jira</button>
<button type="button" id="v-first" data-show="first" aria-pressed="false">Page : première modification</button>
<button type="button" id="v-ov" data-show="overview" aria-pressed="false">Le tunnel</button>
<button type="button" id="t-l" data-theme-set="light">Clair</button>
<button type="button" id="t-d" data-theme-set="dark">Sombre</button></div>
<header>
<div class="topband gut"><div class="in"><span><b>Un projet libre de Mairistem</b> · Groupe JVS</span><a class="pill" href="#">GitHub</a></div></div>
<div class="mainband gut"><div class="in">
<a class="brand" href="#" data-go="home" aria-label="Accueil du site Compagnon, par Mairistem"><svg viewBox="0 0 1188 250" aria-hidden="true"><use href="#cp-logo"></use></svg></a>
<nav class="navlinks" aria-label="Principal"><a href="#" data-go="first">Démarrer</a><a href="#" data-go="jira">Au quotidien</a><a href="#" data-go="overview">Comprendre</a><a href="#" data-go="overview">Pour l'équipe</a><a href="#">Référence</a></nav>
<a class="btn primary sm" href="#" data-go="first">Installer</a>
</div></div></header>
<main>
{HOME}
<div class="shell" data-shell hidden>
<aside>{sidebar(2, 1)}</aside>
<div>{JIRA}{FIRST}{OVERVIEW}</div>
</div>
</main>
<footer class="footer gut"><div class="in"><span>Compagnon — un projet libre de <a href="#">Mairistem</a> · Licence MIT</span><span><a href="#">Documentation</a> · <a href="#">English</a> · <a href="#">GitHub</a></span></div></footer>
<script>{SCRIPT}</script>
"""


cur = HERE / "index.html"
if cur.exists() and "Mairistem Design System — tokens" not in cur.read_text():
    cur.rename(HERE / "index-v2.html")
cur.write_text(page())
print("wrote", cur)
