// Left fold (reduce): single linear pass applying combiner to accumulator. O(n) time, O(1) extra space.

function reduce(arr, combiningFn, initialValue) {
  let acc = initialValue;
  for (const x of arr) acc = combiningFn(acc, x);
  return acc;
}

const add = (a, b) => a + b;
const multiply = (a, b) => a * b;
const sum = (arr) => reduce(arr, add, 0);

function main() {
  console.log(sum([1, 2, 3, 4, 5]));
  console.log(reduce([1, 2, 3, 4], multiply, 1));
}

main();
