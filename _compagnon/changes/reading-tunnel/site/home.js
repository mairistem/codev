const { chromium } = require('playwright');
(async () => {
  const b = await chromium.launch();
  const fails = [];
  for (const file of ['index.html', 'fr.html']) for (const scheme of ['light', 'dark']) for (const w of [360, 1280]) {
    const p = await b.newPage({ viewport: { width: w, height: 900 }, colorScheme: scheme });
    const errs = []; p.on('pageerror', e => errs.push(String(e)));
    await p.goto('file:///home/user/codev/docs/site/' + file);
    const sw = await p.evaluate(() => document.documentElement.scrollWidth);
    if (sw > w) fails.push(`${file} ${scheme} ${w}: scroll ${sw}`);
    if (errs.length) fails.push(`${file} ${scheme} ${w}: ${errs}`);
    if (file === 'index.html' && (w === 1280 || scheme === 'dark')) await p.screenshot({ path: `${process.env.S}/home-${scheme}-${w}.png`, fullPage: w === 1280 && scheme === 'light' });
    await p.close();
  }
  console.log(fails.length ? fails.join('\n') : 'no failure');
  await b.close();
})();
