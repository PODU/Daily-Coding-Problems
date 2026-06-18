// cons returns a closure taking a selector(a,b)->int; car/cdr pass selectors
// returning first/second arg. All O(1).
package main

import "fmt"

type Selector func(a, b int) int
type Pair func(f Selector) int

func cons(a, b int) Pair {
	return func(f Selector) int { return f(a, b) }
}

func car(p Pair) int { return p(func(a, b int) int { return a }) }
func cdr(p Pair) int { return p(func(a, b int) int { return b }) }

func main() {
	fmt.Println(car(cons(3, 4)))
	fmt.Println(cdr(cons(3, 4)))
}
