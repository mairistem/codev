#!/usr/bin/env node
const quotes = require("./quotes.json");

const at = process.argv.indexOf("--author");
const author = at > -1 ? process.argv[at + 1] : null;
const pool = author ? quotes.filter((q) => q.author === author) : quotes;

if (pool.length === 0) {
  console.error(`No quote by ${author}.`);
  process.exit(1);
}
const quote = pool[Math.floor(Math.random() * pool.length)];
console.log(`“${quote.text}” — ${quote.author}`);
