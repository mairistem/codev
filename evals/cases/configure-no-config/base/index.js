#!/usr/bin/env node
// Reads a CSV file with a header line and prints it as a JSON array.
const fs = require("node:fs");

function parse(text) {
  const [header, ...rows] = text.trim().split("\n").map((l) => l.split(","));
  return rows.map((row) => Object.fromEntries(header.map((h, i) => [h, row[i]])));
}

if (require.main === module) {
  console.log(JSON.stringify(parse(fs.readFileSync(process.argv[2], "utf8")), null, 2));
}

module.exports = { parse };
