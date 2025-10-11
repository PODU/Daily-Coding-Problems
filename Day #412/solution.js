// Day 412: Nth term of the look-and-say sequence via run-length encoding.
// Build each term from the previous by counting consecutive runs. O(N * L) time where L = term length, O(L) space.
function lookAndSay(n) {
  let cur = "1";
  for (let t = 1; t < n; t++) {
    let next = "";
    let i = 0;
    const m = cur.length;
    while (i < m) {
      let j = i;
      while (j < m && cur[j] === cur[i]) j++;
      next += (j - i).toString() + cur[i];
      i = j;
    }
    cur = next;
  }
  return cur;
}

const n = 4;
console.log(lookAndSay(n));
