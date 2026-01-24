// Day 946: kth row of Pascal's triangle (1-indexed) using O(k) space.
// In-place update of a single row, right-to-left. Time O(k^2), Space O(k).

function pascalRow(k) {
  const row = [1];
  for (let i = 1; i < k; i++) {
    row.push(0);
    for (let j = row.length - 1; j > 0; j--) row[j] += row[j - 1];
  }
  return row;
}

const k = 5; // README example -> 5th row
console.log(pascalRow(k).join(" "));
