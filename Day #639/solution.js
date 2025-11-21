// Day 639: Letter combinations of a phone number.
// Approach: iterative Cartesian product over digit->letters map.
// Time: O(4^n * n), Space: O(4^n).
function letterCombinations(digits, mapping) {
  if (!digits) return [];
  let res = [""];
  for (const d of digits) {
    const next = [];
    for (const prefix of res)
      for (const ch of mapping[d])
        next.push(prefix + ch);
    res = next;
  }
  return res;
}

const mapping = {
  "2": ["a", "b", "c"], "3": ["d", "e", "f"], "4": ["g", "h", "i"],
  "5": ["j", "k", "l"], "6": ["m", "n", "o"], "7": ["p", "q", "r", "s"],
  "8": ["t", "u", "v"], "9": ["w", "x", "y", "z"],
};
console.log(letterCombinations("23", mapping));
