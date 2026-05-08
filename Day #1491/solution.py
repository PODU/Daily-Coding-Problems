# Day 1491: Demonstrates ad-hoc (single-dispatch overloading), parametric (duck-typed
# generic), and subtype (inheritance/override) polymorphism. Runs in main.
from functools import singledispatch


# Ad-hoc polymorphism: dispatch on argument type.
@singledispatch
def add(a, b):
    raise NotImplementedError


@add.register
def _(a: int, b: int):
    return a + b


@add.register
def _(a: str, b: str):
    return a + b


# Parametric polymorphism: one definition works for any type.
def identity(x):
    return x


# Subtype polymorphism: base reference dispatches to override.
class Animal:
    def speak(self):
        return "..."


class Dog(Animal):
    def speak(self):
        return "Woof"


class Cat(Animal):
    def speak(self):
        return "Meow"


if __name__ == "__main__":
    print("Ad-hoc (overloading): same name, chosen by argument types.")
    print(f"  add(2, 3) = {add(2, 3)}")
    print(f"  add('foo', 'bar') = {add('foo', 'bar')}")

    print("Parametric (generics): one definition, any type.")
    print(f"  identity(42) = {identity(42)}")
    print(f"  identity('hi') = {identity('hi')}")

    print("Subtype (dynamic dispatch): base ref calls overridden method.")
    for a in (Dog(), Cat()):
        print(f"  {a.speak()}")
