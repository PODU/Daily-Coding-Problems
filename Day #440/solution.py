# Day 440: Demonstrates the three kinds of polymorphism.
# ad-hoc = operator overloading / singledispatch, parametric = duck typing/generics,
# subtype = method overriding.
from functools import singledispatch
from math import pi


# Ad-hoc polymorphism: behavior dispatched by argument type.
@singledispatch
def add_them(a, b):
    raise NotImplementedError


@add_them.register
def _(a: int, b: int):
    return a + b


@add_them.register
def _(a: str, b: str):
    return a + b


# Parametric polymorphism: one definition works for any orderable type.
def max_of(a, b):
    return a if a >= b else b


# Subtype polymorphism: each subclass overrides area().
class Shape:
    def area(self):
        raise NotImplementedError


class Circle(Shape):
    def __init__(self, r):
        self.r = r

    def area(self):
        return pi * self.r ** 2


class Square(Shape):
    def __init__(self, s):
        self.s = s

    def area(self):
        return self.s ** 2


if __name__ == "__main__":
    print("Ad-hoc polymorphism (overloading): same name, type-specific impls.")
    print(f"  add(2,3) = {add_them(2, 3)}, add(\"a\",\"b\") = {add_them('a', 'b')}")

    print("Parametric polymorphism (generics): one impl, any type.")
    print(f"  max(3,7) = {max_of(3, 7)}, max(\"ab\",\"zz\") = {max_of('ab', 'zz')}")

    print("Subtype polymorphism (overriding): base ref, derived behavior.")
    for s in (Circle(1.0), Square(2.0)):
        print(f"  area = {s.area():.5f}")
