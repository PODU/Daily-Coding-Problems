// Word wrap greedily: pack max words per line <= k, return null if any word > k. O(total length) time.
function wordWrap(s, k) {
  const lines = [];
  let cur = "";
  for (const w of s.split(" ")) {
    if (w.length > k) return null;
    if (cur === "") cur = w;
    else if (cur.length + 1 + w.length <= k) cur += " " + w;
    else { lines.push(cur); cur = w; }
  }
  if (cur !== "") lines.push(cur);
  return lines;
}

const res = wordWrap("the quick brown fox jumps over the lazy dog", 10);
if (res === null) console.log("null");
else console.log("[" + res.map((ln) => `"${ln}"`).join(", ") + "]");
