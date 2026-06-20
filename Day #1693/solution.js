// Ternary Search Tree: node has char + left/mid/right + isEnd. Compare char: <left, >right, ==mid & advance.
// Insert/search: O(L * log A) average where L=key length, A=alphabet size.

class Node {
  constructor(c) {
    this.c = c;
    this.isEnd = false;
    this.left = null;
    this.mid = null;
    this.right = null;
  }
}

function insert(node, s, i) {
  const ch = s[i];
  if (node === null) node = new Node(ch);
  if (ch < node.c) node.left = insert(node.left, s, i);
  else if (ch > node.c) node.right = insert(node.right, s, i);
  else if (i + 1 < s.length) node.mid = insert(node.mid, s, i + 1);
  else node.isEnd = true;
  return node;
}

function search(node, s, i) {
  if (node === null) return false;
  const ch = s[i];
  if (ch < node.c) return search(node.left, s, i);
  if (ch > node.c) return search(node.right, s, i);
  if (i + 1 === s.length) return node.isEnd;
  return search(node.mid, s, i + 1);
}

let root = null;
for (const w of ["code", "cob", "be", "ax", "war", "we"]) root = insert(root, w, 0);

for (const q of ["code", "cob", "cod", "war", "wa"]) console.log(search(root, q, 0));
