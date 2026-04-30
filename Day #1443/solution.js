// Day 1443: Word sense disambiguation (simplified Lesk algorithm).
// For each ambiguous word pick the meaning whose words overlap most with the
// rest of the sentence's words. Time O(W * M * L), Space O(vocab).
function tokenize(s) {
  return s.toLowerCase().split(/[^a-z0-9]+/).filter(Boolean);
}

function disambiguate(meanings, sentence) {
  const words = tokenize(sentence);
  const context = new Set(words);
  const result = {};
  for (const w of words) {
    const senses = meanings[w];
    if (!senses || senses.length <= 1) continue;
    let best = -1, bestMeaning = null;
    for (const m of senses) {
      const mt = new Set(tokenize(m));
      let score = 0;
      for (const t of mt) if (t !== w && context.has(t)) score++;
      if (score > best) { best = score; bestMeaning = m; }
    }
    result[w] = bestMeaning;
  }
  return result;
}

const meanings = {
  bank: [
    "financial institution where people deposit money",
    "sloping land beside a river or lake",
  ],
};
const sentence = "I went to the bank to deposit money";
const res = disambiguate(meanings, sentence);
console.log("bank:", res.bank);
