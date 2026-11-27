const { chromium } = require('playwright');
(async () => {
  const b = await chromium.launch(); const p = await b.newPage(); const fails = [];
  for (const lang of ['en', 'fr']) {
    let url = `file:///home/user/codev/docs/${lang}/book/introduction.html`; let n = 0;
    while (!url.endsWith('reference/cli.html') && n < 40) {
      await p.goto(url);
      const next = await p.$eval('a.nav-chapters.next', a => a.href).catch(() => null);
      const card = await p.$eval('.tn-next p a', a => a.href).catch(() => null);
      if (!next || card !== next) fails.push(`${lang} ${url}: next ${next} card ${card}`);
      url = next; n++;
    }
    console.log(lang, 'reached', url, 'after', n, 'pages');
  }
  console.log(fails.length ? fails.join('\n') : 'walk agrees'); await b.close();
})();
