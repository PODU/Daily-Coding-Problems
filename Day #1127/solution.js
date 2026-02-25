// Smallest window containing every distinct character of the string.
// Sliding window with frequency counts; expand then shrink. O(n) time, O(k) space.
function smallestWindow(s) {
  const need = new Set(s).size;
  const cnt = new Map();
  let have = 0;
  let best = Infinity;
  let left = 0;
  for (let right = 0; right < s.length; right++) {
    const c = s[right];
    cnt.set(c, (cnt.get(c) || 0) + 1);
    if (cnt.get(c) === 1) have++;
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
