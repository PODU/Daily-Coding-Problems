// Egyptian fraction via greedy: take ceil(b/a) each step. Time O(terms), Space O(1).
function egyptian(a, b) {
  const terms = [];
  while (a !== 0) {
    const x = Math.ceil(b / a);
    terms.push("1 / " + x);
    [a, b] = [a * x - b, b * x];
  }
  return terms.join(" + ");
}

console.log(egyptian(4, 13));
