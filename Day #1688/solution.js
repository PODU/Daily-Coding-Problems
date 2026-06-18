// cons(a,b) returns a closure pair(f)=f(a,b); car/cdr pass selectors. All O(1).

const cons = (a, b) => (f) => f(a, b);
const car = (p) => p((a, b) => a);
const cdr = (p) => p((a, b) => b);

console.log(car(cons(3, 4)));
console.log(cdr(cons(3, 4)));
