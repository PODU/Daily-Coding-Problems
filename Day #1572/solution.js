// Phone keypad letter combinations via iterative Cartesian product. O(prod*len) time, O(output) space.

function letterCombinations(digits, mapping) {
  if (!digits) return [];
  let res = [""];
  for (const d of digits) {
    const next = [];
    for (const pre of res) for (const c of mapping[d]) next.push(pre + c);
    res = next;
  }
  return res;
}

const mapping = { "2": "abc", "3": "def" };
const res = letterCombinations("23", mapping);
console.log("[" + res.map((w) => `"${w}"`).join(", ") + "]");
