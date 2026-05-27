// Look-and-say: build each term by run-length encoding the previous. Time O(total digits), space O(len).
function lookAndSay(n) {
  let cur = "1";
  for (let k = 1; k < n; k++) {
    let next = "";
    let i = 0;
    const len = cur.length;
    while (i < len) {
      let j = i;
      while (j < len && cur[j] === cur[i]) j++;
      next += (j - i) + cur[i];
      i = j;
    }
    cur = next;
  }
  return cur;
}

console.log(lookAndSay(4)); // 1, 11, 21, 1211
