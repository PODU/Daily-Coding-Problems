// Day 1292: kth (0-indexed) row of Pascal's triangle.
// Update row in place from right to left. O(k^2) time, O(k) space.
function pascalRow(k) {
  const row = new Array(k + 1).fill(1);
  for (let i = 1; i <= k; i++)
    for (let j = i - 1; j >= 1; j--) row[j] += row[j - 1];
  return row;
}

console.log(pascalRow(4).join(" ")); // 1 4 6 4 1
