// Reverse words but keep delimiters fixed in place: extract words, reverse the list,
// rebuild keeping delimiter chars at original positions. O(n) time, O(n) space.

const DELIMS = new Set(['/', ':']);

function reverseWords(s) {
  const words = [];
  let cur = "";
  for (const c of s) {
    if (DELIMS.has(c)) {
      if (cur) { words.push(cur); cur = ""; }
    } else {
      cur += c;
    }
  }
  if (cur) words.push(cur);
  words.reverse();

  let out = "";
  let wi = 0, i = 0;
  const n = s.length;
  while (i < n) {
    if (DELIMS.has(s[i])) {
      out += s[i];
      i++;
    } else {
      out += words[wi++];
      while (i < n && !DELIMS.has(s[i])) i++;
    }
  }
  return out;
}

function main() {
  const tests = ["hello/world:here", "hello/world:here/", "hello//world:here"];
  for (const t of tests) console.log(`${t} -> ${reverseWords(t)}`);
}

main();
