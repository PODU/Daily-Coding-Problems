// Smallest window containing every distinct char: sliding window with counts.
// Time O(n), Space O(k).
function smallestWindow(s) {
  const need = new Set(s).size;
  const cnt = new Map();
  let formed = 0, left = 0, best = s.length;
  for (let right = 0; right < s.length; right++) {
    const c = s[right];
    cnt.set(c, (cnt.get(c) || 0) + 1);
    if (cnt.get(c) === 1) formed++;
    while (formed === need) {
      best = Math.min(best, right - left + 1);
      const lc = s[left];
      cnt.set(lc, cnt.get(lc) - 1);
      if (cnt.get(lc) === 0) formed--;
      left++;
    }
  }
  return best;
}

console.log(smallestWindow("jiujitsu"));
