// Closure-based pair: cons stores (a,b) in a closure; car/cdr apply a selector.
// Time: O(1), Space: O(1) per pair.
package main

import "fmt"

type Pair func(func(int, int) int) int

func cons(a, b int) Pair {
	return func(f func(int, int) int) int { return f(a, b) }
}
func car(p Pair) int { return p(func(a, b int) int { return a }) }
func cdr(p Pair) int { return p(func(a, b int) int { return b }) }

func main() {
	fmt.Println(car(cons(3, 4)))
	fmt.Println(cdr(cons(3, 4)))
}
