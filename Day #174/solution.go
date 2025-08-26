// Demonstrates the three polymorphism types: ad-hoc (overloaded-style via types), parametric (generics), subtype (interface dispatch).
package main

import (
	"fmt"
	"math"
)

// Ad-hoc polymorphism: distinct typed implementations (Go simulates overloading per type).
func addInt(a, b int) int       { return a + b }
func addStr(a, b string) string { return a + b }

// Parametric polymorphism: Go generics.
func maxOf[T int | float64](a, b T) T {
	if a > b {
		return a
	}
	return b
}

// Subtype polymorphism: interface with concrete implementations.
type Shape interface{ Area() float64 }
type Circle struct{ R float64 }
type Square struct{ S float64 }

func (c Circle) Area() float64 { return math.Pi * c.R * c.R }
func (s Square) Area() float64 { return s.S * s.S }

func main() {
	fmt.Printf("Ad-hoc polymorphism: add(2,3)=%d, add(\"a\",\"b\")=%s\n", addInt(2, 3), addStr("a", "b"))
	fmt.Printf("Parametric polymorphism: max(3,7)=%d, max(2.5,1.5)=%g\n", maxOf(3, 7), maxOf(2.5, 1.5))
	var c Shape = Circle{1.0}
	var sq Shape = Square{2.0}
	fmt.Printf("Subtype polymorphism: Circle area=%.5f, Square area=%.1f\n", c.Area(), sq.Area())
}
