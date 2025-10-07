// Mark covered indices for every substring occurrence, then wrap maximal runs.
// Time: O(|s| * sum|sub|), Space: O(|s|).
function embolden(s, lst) {
  const n = s.length;
  const bold = new Array(n).fill(false);
  for (const sub of lst) {
    if (!sub) continue;
    let pos = s.indexOf(sub);
    while (pos !== -1) {
      for (let i = pos; i < pos + sub.length; i++) bold[i] = true;
      pos = s.indexOf(sub, pos + 1);
    }
  }
  let out = "";
  for (let i = 0; i < n; i++) {
    if (bold[i] && (i === 0 || !bold[i - 1])) out += "<b>";
    out += s[i];
    if (bold[i] && (i === n - 1 || !bold[i + 1])) out += "</b>";
  }
  return out;
}

console.log(embolden("abcdefg", ["bc", "ef"]));
console.log(embolden("abcdefg", ["bcd", "def"]));
