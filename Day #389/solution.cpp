// Open-ended: contrast composition vs inheritance and when to use each.
#include <iostream>
int main(){
    std::cout << R"(Composition vs Inheritance:

Inheritance is an "is-a" relationship: a subclass extends a base class and
reuses or overrides its behavior. Example: a Dog is-an Animal, so Dog
inherits eat() and adds bark(). It is powerful but tightly couples the
subclass to the parent; changes in the base ripple into all subclasses,
and deep hierarchies become rigid and hard to change.

Composition is a "has-a" relationship: an object is built by holding other
objects and delegating work to them. Example: a Car has-an Engine; the Car
calls engine.start() instead of being an Engine. Behavior is assembled from
parts that can even be swapped at runtime.

When to use each:
- Use inheritance when there is a genuine, stable is-a relationship and you
  want polymorphism over a common base type (e.g., Shape -> Circle, Square).
- Prefer composition for code reuse and flexibility ("favor composition over
  inheritance"). It avoids fragile base-class problems, lets you mix
  behaviors, and supports swapping collaborators at runtime (strategy /
  dependency injection).

Rule of thumb: model true type hierarchies with inheritance; reuse behavior
and build flexible, changeable systems with composition.)" << std::endl;
    return 0;
}
