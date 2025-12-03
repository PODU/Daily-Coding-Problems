// Custom reduce/fold (left to right). O(n) time, O(1) extra space.

function reduce(arr, fn, init) {
  let acc = init;
  for (const x of arr) acc = fn(acc, x);
  return acc;
}

const add = (a, b) => a + b;
const mul = (a, b) => a * b;
console.log(reduce([1, 2, 3, 4], add, 0)); // 10
console.log(reduce([1, 2, 3, 4], mul, 1)); // 24 (bonus)
