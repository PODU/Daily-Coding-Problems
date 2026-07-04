// Soundex phonetic encoding (NARA rules): keep first letter, map rest to digits,
// collapse adjacent same-codes (h/w transparent), drop vowels, pad/truncate to 3 digits.
// Time: O(n) per name, Space: O(n).

function code(c) {
  c = c.toLowerCase();
  if ("bfpv".includes(c)) return 1;
  if ("cgjkqsxz".includes(c)) return 2;
  if ("dt".includes(c)) return 3;
  if (c === "l") return 4;
  if ("mn".includes(c)) return 5;
  if (c === "r") return 6;
  return 0; // vowels a,e,i,o,u,y and h,w
}

function soundex(name) {
  const s = [...name].filter((c) => /[a-zA-Z]/.test(c));
  if (s.length === 0) return "";
  let res = s[0].toUpperCase();
  let prev = code(s[0]);
  for (let i = 1; i < s.length && res.length < 4; i++) {
    const ch = s[i];
    const d = code(ch);
    if (d !== 0 && d !== prev) res += String(d);
    if (ch.toLowerCase() !== "h" && ch.toLowerCase() !== "w") prev = d;
  }
  res = (res + "000").slice(0, 4);
  return res;
}

console.log(soundex("Jackson"));
console.log(soundex("Jaxen"));
