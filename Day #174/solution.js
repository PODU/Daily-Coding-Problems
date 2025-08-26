// Demonstrates the three polymorphism types: ad-hoc (overloading via +), parametric (generic fn), subtype (override).

// Ad-hoc polymorphism: + is overloaded for numbers and strings.
function add(a, b) { return a + b; }

// Parametric polymorphism: generic comparison, works for any orderable type.
function maxOf(a, b) { return a > b ? a : b; }

// Subtype polymorphism: base class with overridden area().
class Shape { area() { throw new Error("abstract"); } }
class Circle extends Shape { constructor(r) { super(); this.r = r; } area() { return Math.PI * this.r * this.r; } }
class Square extends Shape { constructor(s) { super(); this.s = s; } area() { return this.s * this.s; } }

console.log(`Ad-hoc polymorphism: add(2,3)=${add(2, 3)}, add("a","b")=${add("a", "b")}`);
console.log(`Parametric polymorphism: max(3,7)=${maxOf(3, 7)}, max(2.5,1.5)=${maxOf(2.5, 1.5)}`);
const c = new Circle(1.0), sq = new Square(2.0);
console.log(`Subtype polymorphism: Circle area=${c.area().toFixed(5)}, Square area=${sq.area().toFixed(1)}`);
