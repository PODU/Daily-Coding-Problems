// Day 259: Ghost word game. Build a trie of the dictionary. A starting letter is
// guaranteed safe for player 1 iff every word in its subtree has even length, so
// the opponent is always the one forced to complete a word. Trie DFS, O(total chars).

function insert(root, w) {
  let n = root;
  for (const c of w) {
    if (!n.kids[c]) n.kids[c] = { kids: {}, word: false };
    n = n.kids[c];
  }
  n.word = true;
}

function allEven(n, depth) {
  if (n.word && depth % 2 !== 0) return false;
  for (const c of Object.keys(n.kids)) if (!allEven(n.kids[c], depth + 1)) return false;
  return true;
}

const words = ["cat", "calf", "dog", "bear"];
const root = { kids: {}, word: false };
for (const w of words) insert(root, w);

let res = "";
for (const c of Object.keys(root.kids).sort()) if (allEven(root.kids[c], 1)) res += c;
console.log(res);
