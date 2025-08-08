// Day 81: Phone-number letter combinations via iterative cartesian product.
// Time O(prod of letters * len), Space O(output).
function letterCombinations(mapping, digits) {
  if (!digits) return [];
  let res = [""];
  for (const d of digits) {
    const next = [];
    for (const prefix of res)
      for (const letter of mapping[d]) next.push(prefix + letter);
    res = next;
  }
  return res;
}

const mapping = {
  "2": ["a", "b", "c"], "3": ["d", "e", "f"], "4": ["g", "h", "i"],
  "5": ["j", "k", "l"], "6": ["m", "n", "o"], "7": ["p", "q", "r", "s"],
  "8": ["t", "u", "v"], "9": ["w", "x", "y", "z"],
};
console.log(letterCombinations(mapping, "23"));
// [ 'ad', 'ae', 'af', 'bd', 'be', 'bf', 'cd', 'ce', 'cf' ]
