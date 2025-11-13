// Kaprekar's routine: repeatedly subtract ascending-digit number from
// descending-digit number (4-digit, zero-padded) until reaching 6174.
// Bounded to <=7 steps. Time O(1), Space O(1).

function pad(n) {
  return String(n).padStart(4, "0");
}

function main() {
  let n = 1234;
  let steps = 0;
  while (n !== 6174) {
    const s = pad(n);
    const asc = parseInt(s.split("").sort().join(""), 10);
    const desc = parseInt(s.split("").sort().reverse().join(""), 10);
    n = desc - asc;
    steps++;
    console.log(`${pad(desc)} - ${pad(asc)} = ${pad(n)}`);
  }
  console.log(`Steps: ${steps}`);
}

main();
