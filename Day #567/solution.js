// Day 567: Implement car/cdr from closure-based cons.
// cons(a,b) returns a function taking selector f -> f(a,b). Time O(1), Space O(1).
function cons(a, b) {
  return (f) => f(a, b);
}

function car(pair) {
  return pair((a, b) => a);
}

function cdr(pair) {
  return pair((a, b) => b);
}

console.log(car(cons(3, 4)));
console.log(cdr(cons(3, 4)));
