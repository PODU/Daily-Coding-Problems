// Day 567: Implement car/cdr from closure-based cons.
// cons(a,b) returns a function taking selector f -> f(a,b). Time O(1), Space O(1).
package main

import "fmt"

// A pair is a function that applies a selector to the two stored values.
type Selector func(a, b int) int
type Pair func(Selector) int

func cons(a, b int) Pair {
	return func(f Selector) int { return f(a, b) }
}

func car(pair Pair) int {
	return pair(func(a, b int) int { return a })
}

func cdr(pair Pair) int {
	return pair(func(a, b int) int { return b })
}

func main() {
	fmt.Println(car(cons(3, 4)))
	fmt.Println(cdr(cons(3, 4)))
}
