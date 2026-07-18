// Day 1836: Demonstrate ad-hoc, parametric, and subtype polymorphism.
// Each demo is O(1); JS uses dynamic typing for parametric/subtype, runtime checks for ad-hoc.

// Ad-hoc polymorphism: dispatch behavior by argument type.
function add(a, b) {
  if (typeof a === "number") return a + b; // numeric addition
  return a + b; // string concatenation
}

// Parametric polymorphism: one definition usable for any type.
function identity(x) {
  return x;
}

// Subtype polymorphism: base class overridden by subclasses, dispatched at runtime.
class Shape {
  name() {
    return "shape";
  }
}
class Circle extends Shape {
  name() {
    return "circle";
  }
}
class Square extends Shape {
  name() {
    return "square";
  }
}

function main() {
  console.log(`Ad-hoc:     add(2,3)=${add(2, 3)}, add("a","b")=${add("a", "b")}`);
  console.log(`Parametric: identity(7)=${identity(7)}, identity("hi")=${identity("hi")}`);
  const shapes = [new Circle(), new Square()];
  console.log("Subtype:    " + shapes.map((s) => s.name()).join(" "));
}

main();
