// Day 162: Shortest unique prefix via trie. Each node stores pass-through count;
// the shortest prefix with count 1 is unique. Time O(total chars), Space O(same).
'use strict';

function shortestUniquePrefixes(words) {
  const root = { count: 0, child: new Map() };
  for (const w of words) {
    let cur = root;
    for (const c of w) {
      if (!cur.child.has(c)) cur.child.set(c, { count: 0, child: new Map() });
      cur = cur.child.get(c);
      cur.count++;
    }
  }
  const result = [];
  for (const w of words) {
    let cur = root;
    let prefix = '';
    for (const c of w) {
      cur = cur.child.get(c);
      prefix += c;
      if (cur.count === 1) break;
    }
    result.push(prefix);
  }
  return result;
}

const words = ['dog', 'cat', 'apple', 'apricot', 'fish'];
console.log(shortestUniquePrefixes(words).join('\n'));
