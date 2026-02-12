// Word sense disambiguation: for each ambiguous word pick the meaning whose words
// overlap most with the sentence context (other words). Tie-break -> first meaning.
// Time: O(W * M * L), Space: O(L) for the context set.
"use strict";

function tokenize(s) {
  return s.toLowerCase().split(/[^a-z0-9]+/).filter(Boolean);
}

function disambiguate(dict, sentence) {
  const words = tokenize(sentence);
  const results = {};
  for (let i = 0; i < words.length; i++) {
    const w = words[i];
    const meanings = dict[w];
    if (!meanings || meanings.length <= 1) continue;

    const context = new Set(words.filter((_, j) => j !== i));
    let best = 0, bestScore = -1;
    for (let m = 0; m < meanings.length; m++) {
      let score = 0;
      for (const t of tokenize(meanings[m])) if (context.has(t)) score++;
      if (score > bestScore) { bestScore = score; best = m; }
    }
    results[w] = meanings[best];
  }
  return results;
}

const dict = {
  bank: [
    "financial institution where people deposit money",
    "land beside a river or lake",
  ],
};
const sentence = "I went to get money from the bank";
for (const [word, meaning] of Object.entries(disambiguate(dict, sentence))) {
  console.log(`${word}: ${meaning}`);
}
