// Look-and-say: build each term by run-length encoding the previous one.
// Time O(sum of term lengths), Space O(length of Nth term).
'use strict';

function lookAndSay(n) {
  let s = '1';
  for (let i = 1; i < n; i++) {
    let next = '';
    for (let j = 0; j < s.length; ) {
      let k = j;
      while (k < s.length && s[k] === s[j]) k++;
      next += (k - j) + s[j];
      j = k;
    }
    s = next;
  }
  return s;
}

console.log(lookAndSay(5)); // 111221
