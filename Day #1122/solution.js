// Day 1122 - Ghost: winning starting letters for player 1 under optimal play
// Trie + minimax over prefixes. A mover loses if every letter completes a word
// or leads to a winning position for the opponent. Time: O(total letters).

class TrieNode {
  constructor() {
    this.children = new Map();
    this.isWord = false;
  }
}

function insert(root, w) {
  let node = root;
  for (const ch of w) {
    if (!node.children.has(ch)) node.children.set(ch, new TrieNode());
    node = node.children.get(ch);
  }
  node.isWord = true;
}

function canWin(node) {
  for (const child of node.children.values()) {
    if (child.isWord) continue;
    if (!canWin(child)) return true;
  }
  return false;
}

const words = ["cat", "calf", "dog", "bear"];
const root = new TrieNode();
for (const w of words) insert(root, w);

const res = [];
for (const [ch, child] of root.children)
  if (!child.isWord && !canWin(child)) res.push(ch);
res.sort();

console.log("Winning starting letters:", res.join(" "));
