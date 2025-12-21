// Day 777: Ternary Search Tree with insert and search.
// Each node has left/mid/right children. O(L * log A) per op (L=word length).
class Node {
  constructor(c) { this.c = c; this.end = false; this.l = this.m = this.r = null; }
}

class TST {
  constructor() { this.root = null; }

  _insert(node, w, i) {
    const c = w[i];
    if (!node) node = new Node(c);
    if (c < node.c) node.l = this._insert(node.l, w, i);
    else if (c > node.c) node.r = this._insert(node.r, w, i);
    else if (i + 1 < w.length) node.m = this._insert(node.m, w, i + 1);
    else node.end = true;
    return node;
  }
  insert(w) { if (w) this.root = this._insert(this.root, w, 0); }

  search(w) {
    let node = this.root, i = 0;
    while (node && w) {
      const c = w[i];
      if (c < node.c) node = node.l;
      else if (c > node.c) node = node.r;
      else if (i + 1 === w.length) return node.end;
      else { node = node.m; i++; }
    }
    return false;
  }
}

const t = new TST();
for (const w of ["code", "cob", "be", "ax", "war", "we"]) t.insert(w);
console.log(t.search("cob"), t.search("code"), t.search("cod"), t.search("we"));
// true true false true
