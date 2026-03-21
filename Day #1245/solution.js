// Full text justification: greedy packing + even space distribution (extras
// to the left). Time O(total chars), Space O(output).
function justify(words, k) {
  const lines = [];
  let i = 0;
  const n = words.length;
  while (i < n) {
    let j = i, length = 0;
    while (j < n && length + words[j].length + (j - i) <= k) {
      length += words[j].length;
      j++;
    }
    const gaps = j - i - 1;
    let line;
    if (gaps === 0) {
      line = words[i] + ' '.repeat(k - words[i].length);
    } else {
      const spaces = k - length, base = Math.floor(spaces / gaps), extra = spaces % gaps;
      const parts = [];
      for (let w = i; w < j - 1; w++) {
        parts.push(words[w]);
        parts.push(' '.repeat(base + ((w - i) < extra ? 1 : 0)));
      }
      parts.push(words[j - 1]);
      line = parts.join('');
    }
    lines.push(line);
    i = j;
  }
  return lines;
}

const words = ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"];
for (const line of justify(words, 16)) console.log(`"${line}"`);
