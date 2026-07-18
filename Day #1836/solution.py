# Day 1836: Demonstrate ad-hoc, parametric, and subtype polymorphism.
# Each demo is O(1); Python uses duck typing for parametric/subtype, dispatch for ad-hoc.
from functools import singledispatch


# Ad-hoc polymorphism: behavior chosen per argument type (here via singledispatch).
@singledispatch
def add(a, b):
    raise NotImplementedError


@add.register(int)
def _(a, b):
    return a + b


@add.register(str)
def _(a, b):
    return a + b


# Parametric polymorphism: one definition usable with any type (duck typing / generics).
def identity(x):
    return x


# Subtype polymorphism: base class with overriding subclasses, dispatched at runtime.
class Shape:
    def name(self):
        return "shape"


class Circle(Shape):
    def name(self):
        return "circle"


class Square(Shape):
    def name(self):
        return "square"


def main():
    print(f'Ad-hoc:     add(2,3)={add(2, 3)}, add("a","b")={add("a", "b")}')
    print(f'Parametric: identity(7)={identity(7)}, identity("hi")={identity("hi")}')
    shapes = [Circle(), Square()]
    print("Subtype:    " + " ".join(s.name() for s in shapes))


if __name__ == "__main__":
    main()
