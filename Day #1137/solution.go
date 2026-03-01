// cons returns a closure pair(f)=f(a,b); car/cdr apply a selector. O(1).
package main

import "fmt"

type Pair func(func(int, int) int) int

func cons(a, b int) Pair {
	return func(f func(int, int) int) int { return f(a, b) }
}

func car(p Pair) int {
	return p(func(a, b int) int { return a })
}

func cdr(p Pair) int {
	return p(func(a, b int) int { return b })
}

func main() {
	fmt.Println(car(cons(3, 4)))
	fmt.Println(cdr(cons(3, 4)))
}
