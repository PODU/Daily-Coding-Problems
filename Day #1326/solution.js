// Day 1326: Implement reduce/fold — fold an array left to right with a combining function and an initial value.
// O(n) calls to the combiner, O(1) extra space.

function reduce(lst, combine, init) {
  let acc = init;
  for (const x of lst) acc = combine(acc, x);
  return acc;
}

const add = (a, b) => a + b;
const sum = (lst) => reduce(lst, add, 0);

console.log(sum([1, 2, 3, 4, 5])); // 15
