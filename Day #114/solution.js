// Day 114: Reverse words but keep delimiters fixed. Collect words, reverse list,
// re-emit walking original structure. O(n) time, O(n) space.
function reverseKeepDelims(s, delim) {
  const words = [];
  let cur = "";
  for (const c of s) {
    if (delim.has(c)) {
      if (cur) { words.push(cur); cur = ""; }
    } else cur += c;
  }
  if (cur) words.push(cur);
  words.reverse();

  let out = "", wi = 0, i = 0;
  const n = s.length;
  while (i < n) {
    if (delim.has(s[i])) { out += s[i]; i++; }
    else {
      out += words[wi++];
      while (i < n && !delim.has(s[i])) i++;
    }
  }
  return out;
}

const d = new Set(["/", ":"]);
console.log(reverseKeepDelims("hello/world:here", d));   // here/world:hello
console.log(reverseKeepDelims("hello/world:here/", d));  // here/world:hello/
console.log(reverseKeepDelims("hello//world:here", d));  // here//world:hello
