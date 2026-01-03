// Day 846: implement car/cdr for closure-based cons.
// cons stores a pair as a function taking a selector; car/cdr pass a selector. O(1).
package main

import "fmt"

type selector func(a, b int) int
type pair func(f selector) int

func cons(a, b int) pair {
	return func(f selector) int { return f(a, b) }
}
func car(p pair) int { return p(func(a, b int) int { return a }) }
func cdr(p pair) int { return p(func(a, b int) int { return b }) }

func main() {
	fmt.Println(car(cons(3, 4))) // 3
	fmt.Println(cdr(cons(3, 4))) // 4
}
