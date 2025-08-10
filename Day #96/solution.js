// Day 96: All permutations via backtracking on the remaining elements, yielding
// lexicographic order. O(n*n!) time.
function permutations(nums) {
  const res = [];
  function backtrack(path, rem) {
    if (rem.length === 0) { res.push(path.slice()); return; }
    for (let i = 0; i < rem.length; i++) {
      path.push(rem[i]);
      backtrack(path, rem.slice(0, i).concat(rem.slice(i + 1)));
      path.pop();
    }
  }
  backtrack([], nums);
  return res;
}

console.log(JSON.stringify(permutations([1, 2, 3])));
// [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
