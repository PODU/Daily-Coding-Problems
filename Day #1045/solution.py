# Day 1045: Demonstrates three kinds of polymorphism: ad-hoc (type-dispatched add),
# parametric (duck-typed generic first), subtype (overridden method). O(1) demo.


# Ad-hoc: behavior chosen by argument types. Python lacks overloading, so we
# dispatch explicitly; '+' itself is also ad-hoc (int add vs str concat).
def add(a, b):
    return a + b  # int+int -> sum; str+str -> concatenation


# Parametric: duck-typed generic; works for any indexable sequence regardless
# of element type (no type parameter needed in Python).
def first(seq):
    return seq[0]


# Subtype: base class with method overridden by subclasses, called via base ref.
class Animal:
    def speak(self):
        raise NotImplementedError


class Dog(Animal):
    def speak(self):
        return "Woof"


class Cat(Animal):
    def speak(self):
        return "Meow"


def main():
    print("Ad-hoc polymorphism: same name, different argument types (overloading).")
    print(f'Ad-hoc: add(2,3)={add(2, 3)}, add("a","b")={add("a", "b")}')

    print("Parametric polymorphism: one generic definition works for any type.")
    print(f'Parametric: first([1,2,3])={first([1, 2, 3])}, first(["x","y"])={first(["x", "y"])}')

    print("Subtype polymorphism: a base reference dispatches to the subclass override.")
    animals = [Dog(), Cat()]
    print(f"Subtype: Dog says {animals[0].speak()}, Cat says {animals[1].speak()}")


if __name__ == "__main__":
    main()
