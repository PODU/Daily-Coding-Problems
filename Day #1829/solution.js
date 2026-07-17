// Ghost: build a trie, solve the game bottom-up. A mover wins if some child is
// not a word and is a losing position for the opponent. O(total chars).
class Trie {
  constructor() { this.ch = new Map(); this.word = false; }
}

function insert(root, w) {
  let cur = root;
  for (const c of w) {
    if (!cur.ch.has(c)) cur.ch.set(c, new Trie());
    cur = cur.ch.get(c);
  }
  cur.word = true;
}

function canWin(node) {
  for (const child of node.ch.values()) {
    if (child.word) continue;
    if (!canWin(child)) return true;
  }
  return false;
}

const dict = ["cat", "calf", "dog", "bear"];
const root = new Trie();
for (const w of dict) insert(root, w);

const wins = [];
for (const [c, child] of root.ch) {
  if (!child.word && !canWin(child)) wins.push(c);
}
wins.sort();
console.log(wins.join(", "));
