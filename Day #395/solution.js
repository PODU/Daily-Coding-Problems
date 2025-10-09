// Group anagrams: Map keyed by sorted chars -> list, preserving first-seen group order.
// Time O(N*K log K), Space O(N*K).
function groupAnagrams(words) {
  const idx = new Map();
  const groups = [];
  for (const w of words) {
    const key = w.split("").sort().join("");
    if (!idx.has(key)) {
      idx.set(key, groups.length);
      groups.push([w]);
    } else {
      groups[idx.get(key)].push(w);
    }
  }
  return groups;
}

const input = ["eat", "ate", "apt", "pat", "tea", "now"];
const groups = groupAnagrams(input);
const out =
  "[" +
  groups
    .map((g) => "[" + g.map((w) => "'" + w + "'").join(", ") + "]")
    .join(", ") +
  "]";
console.log(out);
