// Day 275: Nth term of look-and-say (term 1 = "1").
// Build each term by run-length encoding the previous. Time O(N * len), Space O(len).
function lookAndSay(n) {
  let cur = "1";
  for (let t = 1; t < n; t++) {
    let nxt = "";
    let i = 0;
    const m = cur.length;
    while (i < m) {
      let j = i;
      while (j < m && cur[j] === cur[i]) j++;
      nxt += (j - i).toString() + cur[i];
      i = j;
    }
    cur = nxt;
  }
  return cur;
}

console.log(lookAndSay(5)); // 111221
