// Trie with pass-through counts; for each word walk down until count==1 to get its shortest unique prefix.
// Time: O(total chars), Space: O(total chars).
class Trie {
  constructor() {
    this.next = new Map();
    this.count = 0;
  }
}

function insert(root, word) {
  let cur = root;
  for (const c of word) {
    if (!cur.next.has(c)) cur.next.set(c, new Trie());
    cur = cur.next.get(c);
    cur.count++;
  }
}

function prefix(root, word) {
  let cur = root;
  let p = "";
  for (const c of word) {
    cur = cur.next.get(c);
    p += c;
    if (cur.count === 1) break;
  }
  return p;
}

function shortestUniquePrefixes(words) {
  const root = new Trie();
  for (const w of words) insert(root, w);
  return words.map((w) => prefix(root, w));
}

const words = ["dog", "cat", "apple", "apricot", "fish"];
const out = shortestUniquePrefixes(words);
console.log("[" + out.join(", ") + "]");
