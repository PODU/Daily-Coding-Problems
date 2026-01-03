// Day 846: implement car/cdr for closure-based cons.
// cons stores a pair as a function taking a selector; car/cdr pass a selector. O(1).
function cons(a, b) {
  return (f) => f(a, b);
}
function car(pair) {
  return pair((a, b) => a);
}
function cdr(pair) {
  return pair((a, b) => b);
}

console.log(car(cons(3, 4))); // 3
console.log(cdr(cons(3, 4))); // 4
