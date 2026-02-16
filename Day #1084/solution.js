// reduce/fold: apply combiner left-to-right starting from init. Time O(n), Space O(1).
function reduce(lst, combine, init) {
  let acc = init;
  for (const x of lst) acc = combine(acc, x);
  return acc;
}

const add = (a, b) => a + b;
const sum = (lst) => reduce(lst, add, 0);

console.log(sum([1, 2, 3, 4, 5])); // 15
