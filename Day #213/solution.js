// Day 213: Generate all valid IP addresses from a digit string.
// Approach: backtracking over the 3 dot positions; each segment 1-3 digits, 0-255, no leading zeros.
// Time O(1) effectively (length <= 12), Space O(1).
function valid(seg) {
  if (!seg || seg.length > 3) return false;
  if (seg.length > 1 && seg[0] === "0") return false;
  return parseInt(seg, 10) <= 255;
}

function restore(s) {
  const res = [];
  function solve(start, parts) {
    if (parts.length === 4) {
      if (start === s.length) res.push(parts.join("."));
      return;
    }
    for (let len = 1; len <= 3 && start + len <= s.length; len++) {
      const seg = s.slice(start, start + len);
      if (valid(seg)) solve(start + len, [...parts, seg]);
    }
  }
  solve(0, []);
  return res;
}

const r = restore("2542540123");
console.log("[" + r.map((x) => `'${x}'`).join(", ") + "]");
