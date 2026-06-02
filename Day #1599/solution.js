// Generate all valid IPv4 addresses by backtracking: place 3 dots, each
// segment len 1..3, value 0..255, no leading zeros. Time O(1) (bounded).
"use strict";

function restoreIp(s) {
  const res = [];

  function backtrack(start, part, cur) {
    if (part === 4) {
      if (start === s.length) res.push(cur.join("."));
      return;
    }
    for (let len = 1; len <= 3 && start + len <= s.length; len++) {
      const seg = s.slice(start, start + len);
      if (seg.length > 1 && seg[0] === "0") break;
      if (parseInt(seg, 10) > 255) break;
      cur.push(seg);
      backtrack(start + len, part + 1, cur);
      cur.pop();
    }
  }

  backtrack(0, 0, []);
  return res;
}

const s = "2542540123";
const res = restoreIp(s);
console.log("[" + res.map((ip) => "'" + ip + "'").join(", ") + "]");
