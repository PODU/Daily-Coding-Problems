// Demonstrates ad-hoc (type-dispatched overloading), parametric (generic),
// and subtype (prototype/override dispatch) polymorphism. Runs on load.

// Ad-hoc polymorphism: behavior chosen by argument types.
function add(a, b) {
  if (typeof a === "number" && typeof b === "number") return a + b;
  if (typeof a === "string" && typeof b === "string") return a + b;
  throw new TypeError("unsupported types");
}

// Parametric polymorphism: one definition works for any type.
function identity(x) {
  return x;
}

// Subtype polymorphism: base reference dispatches to override.
class Animal {
  speak() {
    return "...";
  }
}
class Dog extends Animal {
  speak() {
    return "Woof";
  }
}
class Cat extends Animal {
  speak() {
    return "Meow";
  }
}

console.log("Ad-hoc (overloading): same name, chosen by argument types.");
console.log(`  add(2, 3) = ${add(2, 3)}`);
console.log(`  add("foo", "bar") = ${add("foo", "bar")}`);

console.log("Parametric (generics): one definition, any type.");
console.log(`  identity(42) = ${identity(42)}`);
console.log(`  identity("hi") = ${identity("hi")}`);

console.log("Subtype (dynamic dispatch): base ref calls overridden method.");
for (const a of [new Dog(), new Cat()]) {
  console.log(`  ${a.speak()}`);
}
