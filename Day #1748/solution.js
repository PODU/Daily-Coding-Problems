// Day 1748: Recover digits from an anagram of concatenated number words (zero-nine).
// Approach: unique-letter signatures (z,w,u,x,g first; then derive odd digits). O(n) time, O(1) space.
'use strict';

function recover(s) {
  const c = {};
  for (const ch of s) c[ch] = (c[ch] || 0) + 1;
  const g = ch => c[ch] || 0;
  const cnt = new Array(10).fill(0);
  cnt[0] = g('z');                           // zero
  cnt[2] = g('w');                           // two
  cnt[4] = g('u');                           // four
  cnt[6] = g('x');                           // six
  cnt[8] = g('g');                           // eight
  cnt[3] = g('h') - cnt[8];                  // three
  cnt[5] = g('f') - cnt[4];                  // five
  cnt[7] = g('s') - cnt[6];                  // seven
  cnt[1] = g('o') - cnt[0] - cnt[2] - cnt[4]; // one
  cnt[9] = g('i') - cnt[5] - cnt[6] - cnt[8]; // nine

  let res = '';
  for (let d = 0; d <= 9; d++) res += String(d).repeat(cnt[d]);
  return res;
}

function main() {
  console.log(recover('niesevehrtfeev')); // 357
}

main();
