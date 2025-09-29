// Soundex: keep first letter, map consonants to digits, collapse same adjacent codes
// (h,w transparent; vowels reset), pad/truncate to 3 digits. Time O(L), Space O(1).
function code(c) {
  c = c.toLowerCase();
  const map = {
    b: 1, f: 1, p: 1, v: 1,
    c: 2, g: 2, j: 2, k: 2, q: 2, s: 2, x: 2, z: 2,
    d: 3, t: 3,
    l: 4,
    m: 5, n: 5,
    r: 6,
  };
  return map[c] || 0; // vowels, y, w, h -> 0
}

function soundex(name) {
  let res = name[0].toUpperCase();
  let prev = code(name[0]);
  for (let i = 1; i < name.length && res.length < 4; i++) {
    const c = name[i].toLowerCase();
    const d = code(c);
    if (d !== 0 && d !== prev) res += String(d);
    if (c === "h" || c === "w") continue; // transparent: keep prev
    prev = d;                              // vowels reset prev to 0
  }
  return (res + "000").slice(0, 4);
}

console.log(soundex("Jackson"));
