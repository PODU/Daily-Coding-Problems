// Day 798: Sentence equivalence via synonym pairs (NOT transitive).
// Store each unordered pair in a set; words match if equal or directly paired.
// Time O(W) per comparison, Space O(P).

const syn = new Set();
const key = (a, b) => (a <= b ? `${a}\0${b}` : `${b}\0${a}`);
const add = (a, b) => syn.add(key(a, b));
const same = (a, b) => a === b || syn.has(key(a, b));

function equivalent(s1, s2) {
  if (s1.length !== s2.length) return false;
  return s1.every((w, i) => same(w, s2[i]));
}

add("big", "large");
add("eat", "consume");
const a = "He wants to eat food.".split(" ");
const b = "He wants to consume food.".split(" ");
console.log(equivalent(a, b) ? "True (equivalent)" : "False");
