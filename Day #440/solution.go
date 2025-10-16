// Day 440: Demonstrates the three kinds of polymorphism.
// ad-hoc = type switch, parametric = generics, subtype = interface satisfaction.
package main

import (
	"fmt"
	"math"
)

// Ad-hoc polymorphism: dispatch on concrete type.
func addThem(a, b interface{}) interface{} {
	switch x := a.(type) {
	case int:
		return x + b.(int)
	case string:
		return x + b.(string)
	}
	panic("unsupported types")
}

// Parametric polymorphism: generic over any ordered type.
type Ordered interface {
	~int | ~int64 | ~float64 | ~string
}

func maxOf[T Ordered](a, b T) T {
	if a >= b {
		return a
	}
	return b
}

// Subtype polymorphism: any type satisfying Shape is usable as one.
type Shape interface{ area() float64 }
type Circle struct{ r float64 }
type Square struct{ s float64 }

func (c Circle) area() float64 { return math.Pi * c.r * c.r }
func (s Square) area() float64 { return s.s * s.s }

func main() {
	fmt.Println("Ad-hoc polymorphism (overloading): same name, type-specific impls.")
	fmt.Printf("  add(2,3) = %v, add(\"a\",\"b\") = %v\n", addThem(2, 3), addThem("a", "b"))

	fmt.Println("Parametric polymorphism (generics): one impl, any type.")
	fmt.Printf("  max(3,7) = %v, max(\"ab\",\"zz\") = %v\n", maxOf(3, 7), maxOf("ab", "zz"))

	fmt.Println("Subtype polymorphism (overriding): base ref, derived behavior.")
	for _, s := range []Shape{Circle{1.0}, Square{2.0}} {
		fmt.Printf("  area = %.5f\n", s.area())
	}
}
