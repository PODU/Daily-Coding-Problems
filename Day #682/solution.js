// add_subtract: true currying. Returned function coerces to its numeric
// value via valueOf and alternates +/- on each call. O(n) time/space.

function add_subtract(first) {
  function make(total, sign) {
    const f = (x) => make(total + sign * x, -sign);
    f.valueOf = () => total;
    return f;
  }
  return make(first, 1); // next arg added, then subtracted, then added, ...
}

console.log(+add_subtract(7));              // 7
console.log(+add_subtract(1)(2)(3));        // 0
console.log(+add_subtract(-5)(10)(3)(9));   // 11
