// Day 363: Curried add_subtract that alternately adds/subtracts arguments.
// Return a function carrying the running value (.valueOf) that also chains.
// Time O(k) per chain of k args, Space O(1).
'use strict';

function make(value, count) {
  const f = (x) => make(value + (count % 2 === 1 ? x : -x), count + 1);
  f.valueOf = () => value; // arg1 adds, arg2 subtracts, ...
  return f;
}

function addSubtract(first) {
  return make(first, 1);
}

console.log(+addSubtract(7));
console.log('1 + 2 - 3 ->', +addSubtract(1)(2)(3));
console.log('-5 + 10 - 3 + 9 ->', +addSubtract(-5)(10)(3)(9));
