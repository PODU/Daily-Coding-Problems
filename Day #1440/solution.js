// Day 1440: Ternary search tree with insert and search.
// Approach: each node stores a char + left/mid/right; mid advances the word.
// Time: insert/search O(L * log A) avg, Space: O(total chars).

class Node {
  constructor(c) {
    this.c = c;
    this.isEnd = false;
    this.left = this.mid = this.right = null;
  }
}

function insert(root, word, i = 0) {
  if (i >= word.length) return root;
  const ch = word[i];
  if (root === null) root = new Node(ch);
  if (ch < root.c) {
    root.left = insert(root.left, word, i);
  } else if (ch > root.c) {
    root.right = insert(root.right, word, i);
  } else {
    if (i + 1 === word.length) root.isEnd = true;
    else root.mid = insert(root.mid, word, i + 1);
  }
  return root;
}

function search(root, word, i = 0) {
  if (root === null || i >= word.length) return false;
  const ch = word[i];
  if (ch < root.c) return search(root.left, word, i);
  if (ch > root.c) return search(root.right, word, i);
  if (i + 1 === word.length) return root.isEnd;
  return search(root.mid, word, i + 1);
}

let root = null;
for (const w of ["code", "cob", "be", "ax", "war", "we"]) root = insert(root, w);
console.log(search(root, "code")); // true
console.log(search(root, "cob"));  // true
console.log(search(root, "we"));   // true
console.log(search(root, "cod"));  // false
console.log(search(root, "cat"));  // false
