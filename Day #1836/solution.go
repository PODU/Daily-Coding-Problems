// Day 1836: Demonstrate ad-hoc, parametric, and subtype polymorphism.
// Each demo is O(1); Go uses overloaded-by-interface, generics, and interface dispatch.
package main

import (
	"fmt"
	"strings"
)

// Ad-hoc polymorphism: behavior selected per concrete type at runtime.
func add(a, b interface{}) interface{} {
	switch x := a.(type) {
	case int:
		return x + b.(int)
	case string:
		return x + b.(string)
	}
	return nil
}

// Parametric polymorphism: one generic definition for any type.
func identity[T any](x T) T { return x }

// Subtype polymorphism: an interface satisfied by multiple types, dispatched at runtime.
type Shape interface{ Name() string }
type Circle struct{}
type Square struct{}

func (Circle) Name() string { return "circle" }
func (Square) Name() string { return "square" }

func main() {
	fmt.Printf("Ad-hoc:     add(2,3)=%v, add(\"a\",\"b\")=%v\n", add(2, 3), add("a", "b"))
	fmt.Printf("Parametric: identity(7)=%v, identity(\"hi\")=%v\n", identity(7), identity("hi"))
	shapes := []Shape{Circle{}, Square{}}
	names := make([]string, 0, len(shapes))
	for _, s := range shapes {
		names = append(names, s.Name())
	}
	fmt.Println("Subtype:    " + strings.Join(names, " "))
}
