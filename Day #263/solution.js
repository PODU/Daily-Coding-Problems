// Day 263: Sentence checker over a stream of characters.
// Approach: scan the stream into candidate sentences (cut at terminal marks),
// validate each with a regex enforcing all four rules. Time O(n) per sentence.

// Rules:
// 1. Starts with a capital letter followed by a lowercase letter or a space.
// 2. Other chars: lowercase letters, separators (, ; :) or terminal marks (. ? ! ‽).
// 3. A single space between each word.
// 4. Ends with a terminal mark immediately following a word.
const PATTERN = /^[A-Z](?=[a-z ])[a-z]*[,;:]?( [a-z]+[,;:]?)*[.?!‽]$/;

function isValid(sentence) {
  return PATTERN.test(sentence);
}

function main() {
  const tests = [
    "Hello world.",
    "hello world",
    "Hello  world.",
    "The quick, brown fox jumps.",
    "Is this valid?",
    "No terminal mark",
    "Bad ,spacing.",
  ];
  for (const t of tests) {
    if (isValid(t)) console.log(t);
  }
}

main();
