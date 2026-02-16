// Generate valid IP addresses via backtracking over 4 octets (each 0-255, no leading zeros).
// Time O(1) (bounded by string length <= 12), Space O(1) extra.
function valid(seg) {
  if (!seg || seg.length > 3) return false;
  if (seg.length > 1 && seg[0] === '0') return false;
  return parseInt(seg, 10) <= 255;
}

function restoreIpAddresses(s) {
  const res = [];
  function backtrack(start, part, cur) {
    if (part === 4) {
      if (start === s.length) res.push(cur.join('.'));
      return;
    }
    for (let len = 1; len <= 3 && start + len <= s.length; len++) {
      const seg = s.slice(start, start + len);
      if (valid(seg)) backtrack(start + len, part + 1, [...cur, seg]);
    }
  }
  backtrack(0, 0, []);
  return res;
}

const res = restoreIpAddresses("2542540123");
console.log('[' + res.map(x => `'${x}'`).join(', ') + ']');
