// Day 440: Demonstrates the three kinds of polymorphism.
// ad-hoc = type-checked dispatch, parametric = generic over any type, subtype = override.

// Ad-hoc polymorphism: behavior depends on argument types.
function addThem(a, b) {
  if (typeof a === "number" && typeof b === "number") return a + b;
  if (typeof a === "string" && typeof b === "string") return a + b;
  throw new TypeError("unsupported types");
}

// Parametric polymorphism: works uniformly for any comparable type.
function maxOf(a, b) {
  return a >= b ? a : b;
}

// Subtype polymorphism: each subclass overrides area().
class Shape { area() { throw new Error("abstract"); } }
class Circle extends Shape { constructor(r) { super(); this.r = r; } area() { return Math.PI * this.r ** 2; } }
class Square extends Shape { constructor(s) { super(); this.s = s; } area() { return this.s ** 2; } }

console.log("Ad-hoc polymorphism (overloading): same name, type-specific impls.");
console.log(`  add(2,3) = ${addThem(2, 3)}, add("a","b") = ${addThem("a", "b")}`);

console.log("Parametric polymorphism (generics): one impl, any type.");
console.log(`  max(3,7) = ${maxOf(3, 7)}, max("ab","zz") = ${maxOf("ab", "zz")}`);

console.log("Subtype polymorphism (overriding): base ref, derived behavior.");
for (const s of [new Circle(1.0), new Square(2.0)]) console.log(`  area = ${s.area().toFixed(5)}`);
