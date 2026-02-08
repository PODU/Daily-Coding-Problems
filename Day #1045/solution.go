// Demonstrates three kinds of polymorphism: ad-hoc (distinct typed funcs),
// parametric (generics), subtype (interface dispatch). O(1) demo.
package main

import "fmt"

// Ad-hoc: Go has no overloading, so distinctly named typed functions model it.
func addInt(a, b int) int       { return a + b }
func addStr(a, b string) string { return a + b }

// Parametric: generic function works for any type parameter.
func first[T any](s []T) T { return s[0] }

// Subtype: interface with method implemented (overridden) by concrete types.
type Animal interface{ Speak() string }
type Dog struct{}
type Cat struct{}

func (Dog) Speak() string { return "Woof" }
func (Cat) Speak() string { return "Meow" }

func main() {
	fmt.Println("Ad-hoc polymorphism: same name, different argument types (overloading).")
	fmt.Printf("Ad-hoc: add(2,3)=%d, add(\"a\",\"b\")=%s\n", addInt(2, 3), addStr("a", "b"))

	fmt.Println("Parametric polymorphism: one generic definition works for any type.")
	fmt.Printf("Parametric: first([1,2,3])=%d, first([\"x\",\"y\"])=%s\n",
		first([]int{1, 2, 3}), first([]string{"x", "y"}))

	fmt.Println("Subtype polymorphism: a base reference dispatches to the subclass override.")
	var dog Animal = Dog{}
	var cat Animal = Cat{}
	fmt.Printf("Subtype: Dog says %s, Cat says %s\n", dog.Speak(), cat.Speak())
}
