/*
 * Mairistem theme for the codev documentation: small progressive
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

  document.querySelectorAll('pre > .buttons button.clip-button').forEach(function (button) {
    button.setAttribute('title', 'Copier dans le presse-papiers');
    button.setAttribute('aria-label', 'Copier dans le presse-papiers');
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
