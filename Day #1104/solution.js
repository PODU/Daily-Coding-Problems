// Day 1104: Phone digit -> letters combinations via backtracking.
// Time: O(prod of choices * len). Space: O(len) recursion.
function letterCombos(mapping, digits) {
  if (!digits) return [];
  const out = [];
  const dfs = (i, cur) => {
    if (i === digits.length) { out.push(cur.join("")); return; }
    for (const c of mapping[digits[i]]) {
      cur.push(c);
      dfs(i + 1, cur);
      cur.pop();
    }
  };
  dfs(0, []);
  return out;
}

const mapping = { "2": ["a", "b", "c"], "3": ["d", "e", "f"] };
console.log(letterCombos(mapping, "23"));
// [ 'ad','ae','af','bd','be','bf','cd','ce','cf' ]
