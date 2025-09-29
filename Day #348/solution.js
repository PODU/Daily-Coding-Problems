// Ternary Search Tree with insert/search. Each node: char + left/mid/right + end flag.
// Time: O(L * log A) per op, Space: O(total chars).
class Node {
  constructor(c) {
    this.c = c;
    this.end = false;
    this.left = this.mid = this.right = null;
  }
}

class TST {
  constructor() {
    this.root = null;
  }
  _insert(node, w, i) {
    const ch = w[i];
    if (!node) node = new Node(ch);
    if (ch < node.c) node.left = this._insert(node.left, w, i);
    else if (ch > node.c) node.right = this._insert(node.right, w, i);
    else if (i + 1 < w.length) node.mid = this._insert(node.mid, w, i + 1);
    else node.end = true;
    return node;
  }
  insert(w) {
    if (w) this.root = this._insert(this.root, w, 0);
  }
  search(w) {
    let node = this.root, i = 0;
    while (node) {
      const ch = w[i];
      if (ch < node.c) node = node.left;
      else if (ch > node.c) node = node.right;
      else {
        if (i + 1 === w.length) return node.end;
        i++;
        node = node.mid;
      }
    }
    return false;
  }
}

const tst = new TST();
for (const w of ["code", "cob", "be", "ax", "war", "we"]) tst.insert(w);
for (const q of ["code", "cob", "ax", "c", "war", "cat"])
  console.log(`${q} -> ${tst.search(q)}`);
