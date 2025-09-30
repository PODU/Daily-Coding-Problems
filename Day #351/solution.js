// Simplified Lesk: score each gloss by word overlap with sentence context; pick max (ties->first).
// Time O(words * meanings * glossLen), Space O(vocab).

function disambiguate(meanings, sentence) {
  const tokens = sentence.toLowerCase().split(/\s+/);
  const results = [];
  for (let i = 0; i < tokens.length; i++) {
    const senses = meanings[tokens[i]];
    if (!senses || senses.length < 2) continue; // not ambiguous
    const context = new Set(tokens.filter((_, j) => j !== i));
    let bestIdx = 0, bestScore = -1;
    senses.forEach((gloss, idx) => {
      let score = 0;
      for (const g of new Set(gloss.toLowerCase().split(/\s+/))) {
        if (context.has(g)) score++;
      }
      if (score > bestScore) { bestScore = score; bestIdx = idx; }
    });
    results.push([tokens[i], senses[bestIdx]]);
  }
  return results;
}

function main() {
  const meanings = {
    bank: ["place where people deposit and withdraw money",
           "sloping land beside a river or lake of water"],
    money: ["currency cash that people deposit"],
    river: ["large natural stream of water"],
  };
  const sentence = "I went to get money from the bank";
  for (const [word, sense] of disambiguate(meanings, sentence)) {
    console.log(`${word}: ${sense}`);
  }
}

main();
