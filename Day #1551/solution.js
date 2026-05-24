// Sentence validator over a char stream: split on terminal marks, validate each candidate.
// isValid checks capital start, lowercase/separators body, single spaces, terminal end. Time O(n).

const isTerminal = (c) => c === '.' || c === '?' || c === '!' || c === '‽';
const isSeparator = (c) => c === ',' || c === ';' || c === ':';
const isLower = (c) => c >= 'a' && c <= 'z';
const isUpper = (c) => c >= 'A' && c <= 'Z';

function isValid(s) {
  const n = s.length;
  if (n < 2) return false;
  if (!isUpper(s[0])) return false;
  if (!(isLower(s[1]) || s[1] === ' ')) return false;
  if (!isTerminal(s[n - 1])) return false;
  if (!(isLower(s[n - 2]) || isUpper(s[n - 2]))) return false;
  for (let i = 1; i + 1 < n; i++) {
    const c = s[i];
    if (isLower(c) || isSeparator(c)) continue;
    if (c === ' ') {
      if (s[i - 1] === ' ') return false;
      continue;
    }
    return false;
  }
  return true;
}

function main() {
  const stream = "Hello world. this is bad.";
  let buf = "";
  for (const ch of stream) {
    buf += ch;
    if (isTerminal(ch)) {
      const candidate = buf.trim();
      if (isValid(candidate)) console.log(candidate);
      buf = "";
    }
  }
}

main();
