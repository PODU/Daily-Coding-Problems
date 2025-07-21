// Closure-based pair: cons stores (a,b) in a closure; car/cdr apply a selector.
// Time: O(1), Space: O(1) per pair.
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
