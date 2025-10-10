// Strobogrammatic numbers of N digits: recursive build outside-in, skip leading 0 pair.
// Time O(5^(N/2)) results, Space O(N) recursion depth.
const PAIRS = [["0", "0"], ["1", "1"], ["6", "9"], ["8", "8"], ["9", "6"]];

function build(n, total) {
  if (n === 0) return [""];
  if (n === 1) return ["0", "1", "8"];
  const inner = build(n - 2, total);
  const res = [];
  for (const s of inner) {
    for (const [a, b] of PAIRS) {
      if (n === total && a === "0") continue; // no leading zero
      res.push(a + s + b);
    }
  }
  return res;
}

function main() {
  const res = build(2, 2);
  console.log("[" + res.map((x) => `"${x}"`).join(", ") + "]");
}

main();
