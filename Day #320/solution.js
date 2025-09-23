// Smallest window containing all distinct chars: O(n) sliding window.
// Time O(n), Space O(alphabet).
'use strict';

function smallestWindow(s) {
  const need = new Set(s).size;
  const cnt = new Map();
  let have = 0, left = 0, best = Infinity;
  for (let right = 0; right < s.length; right++) {
    const rc = s[right];
    cnt.set(rc, (cnt.get(rc) || 0) + 1);
    if (cnt.get(rc) === 1) have++;
    while (have === need) {
      best = Math.min(best, right - left + 1);
      const lc = s[left];
      cnt.set(lc, cnt.get(lc) - 1);
      if (cnt.get(lc) === 0) have--;
      left++;
    }
  }
  return best;
}

console.log(smallestWindow("jiujitsu"));
