// Demonstrates ad-hoc (overload via distinct names/variadic-by-type),
// parametric (type parameters), and subtype (interface dispatch) polymorphism.
package main

import "fmt"

// Ad-hoc polymorphism: Go lacks overloading, so we emulate it with a
// type switch giving one name multiple type-specific behaviors.
func add(a, b interface{}) interface{} {
	switch av := a.(type) {
	case int:
		return av + b.(int)
	case string:
		return av + b.(string)
	}
	panic("unsupported types")
}

// Parametric polymorphism: one definition works for any type T.
func identity[T any](x T) T { return x }

// Subtype polymorphism: interface value dispatches to concrete method.
type Animal interface{ Speak() string }
type Dog struct{}
type Cat struct{}

func (Dog) Speak() string { return "Woof" }
func (Cat) Speak() string { return "Meow" }

func main() {
	fmt.Println("Ad-hoc (overloading): same name, chosen by argument types.")
	fmt.Printf("  add(2, 3) = %v\n", add(2, 3))
	fmt.Printf("  add(\"foo\", \"bar\") = %v\n", add("foo", "bar"))

	fmt.Println("Parametric (generics): one definition, any type.")
	fmt.Printf("  identity(42) = %v\n", identity(42))
	fmt.Printf("  identity(\"hi\") = %v\n", identity("hi"))

	fmt.Println("Subtype (dynamic dispatch): base ref calls overridden method.")
	for _, a := range []Animal{Dog{}, Cat{}} {
		fmt.Printf("  %s\n", a.Speak())
	}
}
