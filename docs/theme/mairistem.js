/*
 * Mairistem theme for the compagnon documentation: small progressive
 * enhancements. The pages read correctly without it.
 *
 * - Callouts: a blockquote opening with **Note** / **Remarque** (or
 *   **Warning** / **Attention**) gets a callout class and its title a label
 *   class, for mairistem.css.
 * - Language switch: the EN / FR link points to the same page in the other
 *   book when that page exists, else to the other book's home (the default
 *   href in index.hbs).
 * - Interface strings added by mdBook's scripts (copy button, search
 *   results header) in French in the French book.
 */
(function () {
  'use strict';

  var lang = document.documentElement.lang === 'fr' ? 'fr' : 'en';

  // Callouts ---------------------------------------------------------------
  var KINDS = {
    note: 'note', remarque: 'note', tip: 'note', astuce: 'note', info: 'note',
    warning: 'warning', attention: 'warning', caution: 'warning',
    important: 'warning', avertissement: 'warning'
  };
  document.querySelectorAll('.content main blockquote').forEach(function (quote) {
    var first = quote.firstElementChild;
    if (!first || first.tagName !== 'P') { return; }
    var strong = first.firstChild;
    while (strong && strong.nodeType === 3 && !strong.textContent.trim()) {
      strong = strong.nextSibling;
    }
    if (!strong || strong.nodeName !== 'STRONG') { return; }
    var word = strong.textContent.trim().replace(/[\s:.]+$/, '').toLowerCase();
    var kind = KINDS[word];
    if (!kind) { return; }
    quote.classList.add('mx-callout', 'mx-callout-' + kind);
    strong.classList.add('mx-callout-title');
    quote.setAttribute('role', 'note');
  });

  // Language switch --------------------------------------------------------
  var nav = document.querySelector('.mx-lang');
  var other = nav && nav.querySelector('a[data-lang]');
  var path = nav && nav.getAttribute('data-path');
  if (other && path && /\.md$/.test(path) && path !== 'print.md' && path !== '404.md') {
    var page = path === 'index.md' ? '' : path.replace(/\.md$/, '.html');
    var target = other.getAttribute('href') + page;
    if (!page) {
      // Already the home page of the other book.
    } else if (/^https?:$/.test(location.protocol)) {
      var home = other.getAttribute('href');
      other.setAttribute('href', target);
      fetch(new URL(target, location.href), { method: 'HEAD' }).then(function (res) {
        if (!res.ok) { other.setAttribute('href', home); }
      }).catch(function () { other.setAttribute('href', home); });
    } else {
      // file:// preview: the two books share their page list.
      other.setAttribute('href', target);
    }
  }

  // French interface strings -----------------------------------------------
  if (lang !== 'fr') { return; }

  // mdBook writes its clipboard tooltip ("Copied!") into the button's first
  // child after each copy; translate it as soon as it appears.
  var clipboardMessages = { 'Copied!': 'Copié\u00a0!', 'Clipboard error!': 'Échec de la copie' };
  document.querySelectorAll('pre > .buttons button.clip-button').forEach(function (button) {
    button.setAttribute('title', 'Copier dans le presse-papiers');
    button.setAttribute('aria-label', 'Copier dans le presse-papiers');
    var tooltip = button.firstChild;
    if (!tooltip || !window.MutationObserver) { return; }
    new MutationObserver(function () {
      var translated = clipboardMessages[tooltip.textContent];
      if (translated) { tooltip.textContent = translated; }
    }).observe(tooltip, { childList: true, characterData: true, subtree: true });
  });

  var header = document.getElementById('mdbook-searchresults-header');
  if (header && window.MutationObserver) {
    var translating = false;
    new MutationObserver(function () {
      if (translating) { return; }
      var text = header.textContent;
      var m = /^(\d+) search results? for '(.*)':$/.exec(text);
      var none = /^No search results for '(.*)'\.$/.exec(text);
      var next = null;
      if (m) {
        next = m[1] + (m[1] === '1' ? ' résultat pour « ' : ' résultats pour « ') + m[2] + ' »';
      } else if (none) {
        next = 'Aucun résultat pour « ' + none[1] + ' »';
      }
      if (next !== null && next !== text) {
        translating = true;
        header.textContent = next;
        translating = false;
      }
    }).observe(header, { childList: true, characterData: true, subtree: true });
  }
})();

/* The reading tunnel: above a tunnel chapter's title, its step ("Step 3
 * of 5 · Every day") and five progress segments; its "Next step" section
 * drawn as a card; the lines of its examples marked as Claude Code marks
 * them. Without script, the chapter reads the same, without these. The
 * chapters and step titles follow docs_checks::TUNNEL. */
(function () {
  'use strict';
  var STEPS = [
    { en: 'Discover', fr: 'Découvrir', color: '#ff5331', chapters: ['introduction'] },
    { en: 'Get started', fr: 'Démarrer', color: '#00c39b', chapters: ['installation', 'quickstart'] },
    { en: 'Every day', fr: 'Au quotidien', color: '#ffab03', chapters: ['skills', 'guides/mcp', 'guides/review-a-plan', 'guides/approve-a-plan', 'guides/build-and-file'] },
    { en: 'Understand', fr: 'Comprendre', color: '#004c66', chapters: ['workflow', 'concepts', 'guides/decisions', 'roles', 'guides/review-record'] },
    { en: 'For your team', fr: "Pour l'équipe", color: '#8aa3ac', chapters: ['guides/existing-project', 'guides/team-rollout', 'guides/inherited-sources', 'guides/artifact-language', 'guides/release-notes', 'guides/custom-schemas'] }
  ];
  var fr = (document.documentElement.lang || '').indexOf('fr') === 0;
  var path = window.location.pathname.replace(/\/index\.html$/, '/introduction.html');
  var here = -1;
  STEPS.forEach(function (step, i) {
    step.chapters.forEach(function (chapter) {
      if (path.slice(-(chapter.length + 6)) === '/' + chapter + '.html') { here = i; }
    });
  });
  var main = document.querySelector('#mdbook-content main, .content main, main');

  if (main && here >= 0) {
    var title = main.querySelector('h1');
    if (title) {
      var head = document.createElement('div');
      head.className = 'tn-head';
      var eyebrow = document.createElement('p');
      eyebrow.className = 'tn-eyebrow';
      eyebrow.textContent = (fr ? 'Étape ' + (here + 1) + ' sur 5 · ' : 'Step ' + (here + 1) + ' of 5 · ') + STEPS[here][fr ? 'fr' : 'en'];
      var bar = document.createElement('ol');
      bar.className = 'tn-progress';
      bar.setAttribute('aria-hidden', 'true');
      STEPS.forEach(function (step, i) {
        var seg = document.createElement('li');
        seg.className = i < here ? 'done' : i === here ? 'current' : 'todo';
        seg.style.setProperty('--seg', step.color);
        bar.appendChild(seg);
      });
      head.appendChild(eyebrow);
      head.appendChild(bar);
      title.parentNode.insertBefore(head, title);
    }

    var next = main.querySelector('h2#next-step, h2#étape-suivante');
    if (next) {
      var card = document.createElement('section');
      card.className = 'tn-next';
      card.setAttribute('aria-labelledby', next.id);
      next.parentNode.insertBefore(card, next);
      while (card.nextSibling) { card.appendChild(card.nextSibling); }
    }
  }

  Array.prototype.forEach.call(document.querySelectorAll('.example pre code.language-text'), function (code) {
    var escape = function (text) {
      return text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
    };
    code.innerHTML = code.textContent.split('\n').map(function (line) {
      if (/^> /.test(line)) { return '<span class="cc-user">' + escape(line) + '</span>'; }
      if (/^⏺/.test(line)) { return '<span class="cc-dot">⏺</span>' + escape(line.slice(1)); }
      if (/^\s+⎿/.test(line)) { return '<span class="cc-res">' + escape(line) + '</span>'; }
      return escape(line);
    }).join('\n');
  });
})();
