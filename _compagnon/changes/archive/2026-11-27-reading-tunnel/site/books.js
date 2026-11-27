const { chromium } = require('playwright');
(async () => {
  const b = await chromium.launch();
  const fails = [];
  const base = 'file:///home/user/codev/docs/';
  for (const [lang, want] of [['en', 'Step 3 of 5 · Every day'], ['fr', 'Étape 3 sur 5 · Au quotidien']]) for (const theme of ['light', 'navy']) for (const w of [360, 1280]) {
    const ctx = await b.newContext({ viewport: { width: w, height: 900 } });
    await ctx.addInitScript(t => { try { localStorage.setItem('mdbook-theme', t); } catch (e) {} }, theme);
    const p = await ctx.newPage();
    const errs = []; p.on('pageerror', e => errs.push(String(e)));
    const tag = `${lang} ${theme} ${w}`;
    await p.goto(base + lang + '/book/guides/mcp.html');
    const eb = await p.locator('.tn-eyebrow').textContent().catch(() => null);
    if (eb !== want) fails.push(`${tag}: eyebrow ${eb}`);
    const cur = await p.$$eval('.tn-progress li', l => l.map(x => x.className).join(','));
    if (cur !== 'done,done,current,todo,todo') fails.push(`${tag}: segments ${cur}`);
    if (!(await p.locator('.tn-next a[href$="review-a-plan.html"]').count())) fails.push(`${tag}: next card link`);
    const skip = p.locator('.skip-step a');
    const name = await skip.evaluate(a => a.textContent.trim());
    const before = await skip.evaluate(a => getComputedStyle(a).backgroundColor + getComputedStyle(a).outlineStyle);
    await skip.focus(); await p.keyboard.press('Shift+Tab'); await p.keyboard.press('Tab');
    const after = await skip.evaluate(a => getComputedStyle(a).backgroundColor + getComputedStyle(a).outlineStyle);
    if (before === after) fails.push(`${tag}: focus looks the same`);
    const acc = await p.locator('.skip-step a').getAttribute('aria-label');
    if (acc || !/Skip to|Passez à/.test(name)) fails.push(`${tag}: name ${name}`);
    const sw = await p.evaluate(() => document.documentElement.scrollWidth);
    if (sw > w) fails.push(`${tag}: scroll ${sw}`);
    if (w === 1280 && lang === 'en') await p.screenshot({ path: `${process.env.S}/book-${theme}.png`, fullPage: true });
    await p.goto(base + lang + '/book/reference/cli.html');
    if (await p.locator('.tn-head').count()) fails.push(`${tag}: reference shows a step`);
    if (errs.length) fails.push(`${tag}: ${errs}`);
    await ctx.close();
  }
  console.log(fails.length ? fails.join('\n') : 'no failure');
  await b.close();
})();
