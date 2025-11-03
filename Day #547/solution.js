// Maximum XOR of two elements using a binary trie, greedily pick opposite bit.
// Time O(n*32), Space O(n*32).
class Trie {
  constructor() {
    this.child = [null, null];
  }
}

function insert(root, num) {
  let cur = root;
  for (let b = 31; b >= 0; b--) {
    const bit = (num >> b) & 1;
    if (cur.child[bit] === null) cur.child[bit] = new Trie();
    cur = cur.child[bit];
  }
}

function maxXor(nums) {
  const root = new Trie();
  for (const x of nums) insert(root, x);
  let best = 0;
  for (const x of nums) {
    let cur = root;
    let curXor = 0;
    for (let b = 31; b >= 0; b--) {
      const bit = (x >> b) & 1;
      const want = bit ^ 1;
      if (cur.child[want] !== null) {
        curXor |= 1 << b;
        cur = cur.child[want];
      } else {
        cur = cur.child[bit];
      }
    }
    best = Math.max(best, curXor);
  }
  return best;
}

function main() {
  const nums = [3, 10, 5, 25, 2, 8];
  console.log(maxXor(nums));
}

main();
