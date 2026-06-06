// Day 1623: Sentence equivalence via synonym set.
// Build symmetric synonym set; compare word-by-word. Time O(W), Space O(S).
function equivalent(a, b, syns) {
  const pairs = new Set();
  for (const [x, y] of syns) {
    pairs.add(x + "\0" + y);
    pairs.add(y + "\0" + x);
  }
  const wa = a.split(/\s+/), wb = b.split(/\s+/);
  if (wa.length !== wb.length) return false;
  return wa.every((x, i) => x === wb[i] || pairs.has(x + "\0" + wb[i]));
}

const syns = [["big", "large"], ["eat", "consume"]];
const eq = equivalent("He wants to eat food.", "He wants to consume food.", syns);
console.log(eq ? "The two sentences are equivalent." : "The two sentences are not equivalent.");
