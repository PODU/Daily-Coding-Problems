// Word sense disambiguation via simplified Lesk.
// Score each candidate gloss by content-word overlap with the sentence
// context (other words + their glosses); pick the highest.
// Time O(words * meanings * glossLen). Space O(vocab).

const STOP = new Set([
  "i", "to", "the", "a", "an", "of", "by", "and", "or", "where", "people",
  "that", "is", "are", "in", "on", "at", "with", "went", "sat", "this"
]);

function tokens(text) {
  const out = new Set();
  for (const w of text.toLowerCase().split(/\s+/)) {
    const lw = w.replace(/[^a-z]/g, "");
    if (lw && !STOP.has(lw)) out.add(lw);
  }
  return out;
}

function main() {
  const meanings = {
    bank: ["a financial institution where people deposit and withdraw money",
           "the land alongside a river or lake"],
    money: ["currency that people deposit and withdraw"],
    river: ["a large natural stream of water"]
  };

  const sentences = [
    "I went to the bank to deposit money",
    "I sat by the bank of the river"
  ];

  for (const sentence of sentences) {
    const words = sentence.split(/\s+/);
    for (const w of words) {
      const lw = w.toLowerCase();
      const cands = meanings[lw];
      if (!cands || cands.length <= 1) continue;

      const context = new Set();
      for (const other of words) {
        const ol = other.toLowerCase();
        if (ol === lw) continue;
        for (const t of tokens(other)) context.add(t);
        for (const g of meanings[ol] || []) {
          for (const t of tokens(g)) context.add(t);
        }
      }

      let best = -1;
      let bestGloss = "";
      for (const gloss of cands) {
        let overlap = 0;
        for (const t of tokens(gloss)) if (context.has(t)) overlap++;
        if (overlap > best) { best = overlap; bestGloss = gloss; }
      }
      console.log(`${lw}: ${bestGloss}`);
    }
  }
}

main();
