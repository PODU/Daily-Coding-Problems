// Demonstrates three kinds of polymorphism: ad-hoc (type-dispatched add),
// parametric (duck-typed generic first), subtype (overridden method). O(1) demo.

// Ad-hoc: '+' is type-dependent; numbers add, strings concatenate.
function add(a, b) {
  return a + b;
}

// Parametric: duck-typed generic; works for any array regardless of element type
// (no type parameter needed in JS).
function first(arr) {
  return arr[0];
}

// Subtype: base class with method overridden by subclasses, called via base ref.
class Animal {
  speak() {
    throw new Error("abstract");
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

function main() {
  console.log("Ad-hoc polymorphism: same name, different argument types (overloading).");
  console.log(`Ad-hoc: add(2,3)=${add(2, 3)}, add("a","b")=${add("a", "b")}`);

  console.log("Parametric polymorphism: one generic definition works for any type.");
  console.log(`Parametric: first([1,2,3])=${first([1, 2, 3])}, first(["x","y"])=${first(["x", "y"])}`);

  console.log("Subtype polymorphism: a base reference dispatches to the subclass override.");
  const animals = [new Dog(), new Cat()];
  console.log(`Subtype: Dog says ${animals[0].speak()}, Cat says ${animals[1].speak()}`);
}

main();
