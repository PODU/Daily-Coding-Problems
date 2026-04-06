// Egyptian fraction via Fibonacci/Sylvester greedy: repeatedly subtract the
// largest unit fraction 1/ceil(b/a). Time O(#terms), Space O(1).
'use strict';

function egyptian(a, b) {
  const terms = [];
  while (a !== 0) {
    const x = Math.ceil(b / a);
    terms.push(`1 / ${x}`);
    [a, b] = [a * x - b, b * x];
  }
  return terms.join(' + ');
}

console.log(egyptian(4, 13)); // 1 / 4 + 1 / 18 + 1 / 468
