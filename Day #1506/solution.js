// Maximum XOR of two elements using a binary (bitwise prefix) trie over 32 bits.
// Insert each number's bits, query best complement. Time O(n*32), Space O(n*32).

class Trie {
  constructor() {
    this.child = [null, null];
  }
}

function insertNum(root, num) {
  let node = root;
  for (let i = 31; i >= 0; i--) {
    const b = (num >> i) & 1;
    if (!node.child[b]) node.child[b] = new Trie();
    node = node.child[b];
  }
}

function queryBest(root, num) {
  let node = root;
  let best = 0;
  for (let i = 31; i >= 0; i--) {
    const b = (num >> i) & 1;
    const want = b ^ 1;
    if (node.child[want]) {
      best |= (1 << i);
      node = node.child[want];
    } else {
      node = node.child[b];
    }
  }
  return best;
}

function findMaximumXOR(nums) {
  const root = new Trie();
  let best = 0;
  for (const x of nums) {
    insertNum(root, x);
    best = Math.max(best, queryBest(root, x));
  }
  return best;
}

const nums = [3, 10, 5, 25, 2, 8];
console.log(findMaximumXOR(nums));
