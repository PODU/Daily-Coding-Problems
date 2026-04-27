// Day 1433: Sentence checker over a character stream.
// Approach: finite-state machine validating one sentence at a time. Time: O(n), Space: O(n) buffer.

const TERMINALS = new Set([".", "?", "!", "‽"]); // . ? ! ‽
const SEPARATORS = new Set([",", ";", ":"]);

function isLowerLetter(c) {
  return c >= "a" && c <= "z";
}
function isUpperLetter(c) {
  return c >= "A" && c <= "Z";
}

function isValidSentence(s) {
  const n = s.length;
  if (n < 2) return false;
  // Rule 1: capital then lowercase or space.
  if (!isUpperLetter(s[0])) return false;
  if (!(isLowerLetter(s[1]) || s[1] === " ")) return false;

  let prevWasLetter = isUpperLetter(s[0]) || isLowerLetter(s[0]);
  for (let i = 1; i < n; i++) {
    const c = s[i];
    if (TERMINALS.has(c)) {
      if (!prevWasLetter) return false; // Rule 4
      return i === n - 1;
    }
    if (c === " ") {
      if (s[i - 1] === " ") return false; // Rule 3: single space
      prevWasLetter = false;
    } else if (isLowerLetter(c)) {
      prevWasLetter = true;
    } else if (SEPARATORS.has(c)) {
      prevWasLetter = false;
    } else {
      return false;
    }
  }
  return false; // no terminal mark
}

const tests = [
  "The quick brown fox.",
  "Hello world!",
  "lowercase start.",
  "No terminal mark",
  "Two  spaces here.",
];
for (const t of tests) {
  if (isValidSentence(t)) console.log(t);
}
