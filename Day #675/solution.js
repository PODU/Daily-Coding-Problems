// Day 675: Sentence equivalence under (non-transitive) synonym pairs. For each position,
// words must be equal or a known synonym pair. Time O(W) with a hashed pair set.
function tokens(s) {
  return (s.toLowerCase().match(/[a-z0-9]+/g)) || [];
}

function equivalent(synonyms, s1, s2) {
  const pairs = new Set();
  for (const [a, b] of synonyms) { pairs.add(a + "\0" + b); pairs.add(b + "\0" + a); }
  const w1 = tokens(s1), w2 = tokens(s2);
  if (w1.length !== w2.length) return false;
  return w1.every((a, i) => a === w2[i] || pairs.has(a + "\0" + w2[i]));
}

const syn = [["big", "large"], ["eat", "consume"]];
console.log(equivalent(syn, "He wants to eat food.", "He wants to consume food.") ? "True" : "False"); // True
