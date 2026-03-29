// Day 1277: Curried add_subtract — alternately + then - successive args.
// Closure returns itself each call; valueOf exposes the running result for coercion. O(1)/call.
function add_subtract(first) {
  let value = first;
  let idx = 1;
  function next(x) {
    value = idx % 2 === 1 ? value + x : value - x;
    idx += 1;
    return next;
  }
  next.valueOf = () => value;
  return next;
}

console.log(+add_subtract(7));            // 7
console.log(+add_subtract(1)(2)(3));      // 0
console.log(+add_subtract(-5)(10)(3)(9)); // 11
