// Look-and-say sequence: start "1"; each term describes digit runs of previous.
// Iteratively build N terms by run-length encoding. Time O(N * L), Space O(L).

function lookAndSay(n) {
  let cur = "1";
  for (let i = 1; i < n; i++) {
    let next = "";
    let j = 0;
    while (j < cur.length) {
      let count = 1;
      while (j + count < cur.length && cur[j + count] === cur[j]) count++;
      next += count.toString() + cur[j];
      j += count;
    }
    cur = next;
  }
  return cur;
}

function main() {
  const N = 5; // terms: 1, 11, 21, 1211, 111221
  console.log(lookAndSay(N));
}

main();
