// Day 952: decode an anagram of spelled-out digit words (zero..nine) -> sorted digits.
// Use unique marker letters to count each digit. Time O(n), Space O(1).

function decode(s) {
  const c = {};
  for (const ch of s) c[ch] = (c[ch] || 0) + 1;
  const g = (ch) => c[ch] || 0;
  const cnt = new Array(10).fill(0);
  cnt[0] = g("z");
  cnt[2] = g("w");
  cnt[4] = g("u");
  cnt[6] = g("x");
  cnt[8] = g("g");
  cnt[3] = g("h") - cnt[8];
  cnt[5] = g("f") - cnt[4];
  cnt[7] = g("s") - cnt[6];
  cnt[1] = g("o") - cnt[0] - cnt[2] - cnt[4];
  cnt[9] = g("i") - cnt[5] - cnt[6] - cnt[8];
  let res = "";
  for (let d = 0; d <= 9; d++) res += String(d).repeat(cnt[d]);
  return res;
}

console.log(decode("niesevehrtfeev")); // 357
