// cons returns a closure pair(f)=f(a,b); car/cdr apply a selector. O(1).
const cons = (a, b) => (f) => f(a, b);
const car = (pair) => pair((a, b) => a);
const cdr = (pair) => pair((a, b) => b);

console.log(car(cons(3, 4)));
console.log(cdr(cons(3, 4)));
