// Restore IP addresses via backtracking over 4 octets. Time O(1) (<=3^3 splits),
// Space O(1) recursion depth. Each octet in [0,255], no leading zeros (except "0").
function valid(seg) {
  if (!seg || seg.length > 3) return false;
  if (seg.length > 1 && seg[0] === "0") return false;
  return parseInt(seg, 10) <= 255;
}

function restore(s) {
  const res = [];
  const bt = (start, part, cur) => {
    if (part === 4) {
      if (start === s.length) res.push(cur.join("."));
      return;
    }
    for (let len = 1; len <= 3 && start + len <= s.length; len++) {
      const seg = s.slice(start, start + len);
      if (valid(seg)) bt(start + len, part + 1, [...cur, seg]);
    }
  };
  bt(0, 0, []);
  return res;
}

const res = restore("2542540123");
console.log("[" + res.map((x) => `'${x}'`).join(", ") + "]");
