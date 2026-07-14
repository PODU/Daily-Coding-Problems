// Curried add_subtract: each call alternates +/- on the running total.
// Returned function carries a valueOf so it coerces to the running number.
// Time: O(1) per call. Space: O(1).
function makeAddSub(total, sign) {
  const f = (y) => makeAddSub(total + sign * y, -sign);
  f.valueOf = () => total;
  return f;
}

function add_subtract(x) {
  return makeAddSub(x, 1);
}

console.log(+add_subtract(7));            // 7
console.log(+add_subtract(1)(2)(3));      // 0
console.log(+add_subtract(-5)(10)(3)(9)); // 11
