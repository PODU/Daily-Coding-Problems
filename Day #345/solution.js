// Sentence Similarity. Direct (non-transitive) synonym pairs via Set.
// Time O(P + N) for P pairs and N words. Space O(P).
// Secondary union-find approach (transitive follow-up) included below.
function tokenize(s) {
  return s
    .trim()
    .split(/\s+/)
    .map((w) => w.replace(/[.,!?;:]+$/, ""));
}

function areSimilar(synonyms, s1, s2) {
  const pairs = new Set();
  for (const [a, b] of synonyms) {
    pairs.add(a + "\0" + b);
    pairs.add(b + "\0" + a);
  }
  const w1 = tokenize(s1), w2 = tokenize(s2);
  if (w1.length !== w2.length) return false;
  for (let i = 0; i < w1.length; i++) {
    if (w1[i] === w2[i]) continue;
    if (pairs.has(w1[i] + "\0" + w2[i])) continue;
    return false;
  }
  return true;
}

// --- Follow-up (transitive): union-find ---
function areSimilarTransitive(synonyms, s1, s2) {
  const parent = new Map();
  const find = (x) => {
    if (!parent.has(x)) parent.set(x, x);
    while (parent.get(x) !== x) {
      parent.set(x, parent.get(parent.get(x)));
      x = parent.get(x);
    }
    return x;
  };
  const union = (a, b) => parent.set(find(a), find(b));
  for (const [a, b] of synonyms) union(a, b);
  const w1 = tokenize(s1), w2 = tokenize(s2);
  if (w1.length !== w2.length) return false;
  return w1.every((x, i) => x === w2[i] || find(x) === find(w2[i]));
}

function main() {
  const synonyms = [["big", "large"], ["eat", "consume"]];
  const s1 = "He wants to eat food.";
  const s2 = "He wants to consume food.";
  console.log(areSimilar(synonyms, s1, s2) ? "equivalent" : "not equivalent");
}

main();
