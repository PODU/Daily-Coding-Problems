# Day 174: Demonstrates the three polymorphism types: ad-hoc (overloading via dispatch), parametric (duck-typed generics), subtype (override).
import math
from abc import ABC, abstractmethod


# Ad-hoc polymorphism: a single add that behaves per operand type (+ is overloaded).
def add(a, b):
    return a + b


# Parametric polymorphism: works generically for any comparable type.
def max_of(a, b):
    return a if a > b else b


# Subtype polymorphism: base class with overridden area().
class Shape(ABC):
    @abstractmethod
    def area(self):
        ...


class Circle(Shape):
    def __init__(self, r):
        self.r = r

    def area(self):
        return math.pi * self.r * self.r


class Square(Shape):
    def __init__(self, s):
        self.s = s

    def area(self):
        return self.s * self.s


if __name__ == "__main__":
    print(f'Ad-hoc polymorphism: add(2,3)={add(2, 3)}, add("a","b")={add("a", "b")}')
    print(f"Parametric polymorphism: max(3,7)={max_of(3, 7)}, max(2.5,1.5)={max_of(2.5, 1.5)}")
    c, sq = Circle(1.0), Square(2.0)
    print(f"Subtype polymorphism: Circle area={c.area():.5f}, Square area={sq.area():.1f}")
