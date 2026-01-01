// Day 837: Sentence checker over a character stream.
// Accumulate chars until a terminal mark, then validate the buffered sentence by regex; print if valid.
// O(N) over the stream; O(L) per sentence buffer.

const TERMINALS = ".?!‽"; // . ? ! ‽
const PATTERN = /^[A-Z][a-z]*[,;:]?( [a-z]+[,;:]?)*[.?!‽]$/;

function checkStream(stream) {
  const results = [];
  let buf = "";
  for (const ch of stream) {
    buf += ch;
    if (TERMINALS.includes(ch)) {
      const sentence = buf.replace(/^ +/, ""); // trim leading space(s) between sentences
      if (PATTERN.test(sentence)) results.push(sentence);
      buf = "";
    }
  }
  return results;
}

const stream = "Hello, world. this is wrong. The cat sat.";
for (const s of checkStream(stream)) console.log(s);
