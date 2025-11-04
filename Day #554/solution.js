// Egyptian fraction via greedy (Fibonacci/Sylvester): take ceil(b/a) each step.
// Time: O(number of terms), Space: O(1). BigInt-safe via Number here (small).
function egyptian(a, b) {
  const terms = [];
  while (a !== 0) {
    const x = Math.ceil(b / a);
    terms.push(`1 / ${x}`);
    a = a * x - b;
    b = b * x;
  }
  return terms.join(" + ");
}

console.log(egyptian(4, 13));
