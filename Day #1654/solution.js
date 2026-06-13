// Simplified Lesk WSD: score each candidate meaning by overlap with the union of other
// in-dict context words and their meaning texts. O(W*M*L) time, O(V) space.
const meanings = {
  bank: ["place where people deposit and withdraw money", "land beside a river or lake"],
  money: ["currency coins and cash used to buy things"],
  river: ["a large natural stream of flowing water"],
};
const sentence = "I went to get money from the bank";
const words = s => s.toLowerCase().split(/\s+/);
const toks = words(sentence);
for (const w of toks) {
  if (meanings[w] && meanings[w].length > 1) {
    const ctx = new Set();
    for (const o of toks) {
      if (o !== w && meanings[o]) {
        ctx.add(o);
        for (const m of meanings[o]) words(m).forEach(x => ctx.add(x));
      }
    }
    let best = meanings[w][0], bestScore = -1;
    for (const cand of meanings[w]) {
      let score = 0;
      for (const t of words(cand)) if (ctx.has(t)) score++;
      if (score > bestScore) { bestScore = score; best = cand; }
    }
    console.log(`${w}: ${best}`);
  }
}
