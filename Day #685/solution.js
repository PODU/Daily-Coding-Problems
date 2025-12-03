// Reverse words between delimiters while keeping delimiters fixed in position.
// Tokenize (words = maximal non-delim runs incl. interior empties), reverse word list, reassemble. O(n) time/space.
function reverseWords(s, delims) {
  const tokens = []; // { d: bool, v: string }
  let buf = "";
  for (const c of s) {
    if (delims.has(c)) {
      tokens.push({ d: false, v: buf });
      tokens.push({ d: true, v: c });
      buf = "";
    } else {
      buf += c;
    }
  }
  if (buf.length) tokens.push({ d: false, v: buf });

  const words = tokens.filter(t => !t.d).map(t => t.v).reverse();
  let out = "", wi = 0;
  for (const t of tokens) out += t.d ? t.v : words[wi++];
  return out;
}

const d = new Set(['/', ':']);
for (const t of ["hello/world:here", "hello/world:here/", "hello//world:here"])
  console.log(`${t} -> ${reverseWords(t, d)}`);
