// Day 991: Closure late-binding demonstration.
// Buggy closures capture a pointer to one shared variable (all read 3);
// fixed closures capture a per-iteration value copy (1,2,3). O(n) time/space.
package main

import "fmt"

func makeBuggy() []func() int {
	var v []func() int
	shared := 0
	for _, val := range []int{1, 2, 3} {
		shared = val // single shared variable
		p := &shared
		v = append(v, func() int { return *p })
	}
	return v
}

func makeFixed() []func() int {
	var v []func() int
	for _, val := range []int{1, 2, 3} {
		val := val // shadow: fresh copy per iteration
		v = append(v, func() int { return val })
	}
	return v
}

func main() {
	fmt.Print("Buggy:")
	for _, f := range makeBuggy() {
		fmt.Print(" ", f())
	}
	fmt.Print("\nFixed:")
	for _, f := range makeFixed() {
		fmt.Print(" ", f())
	}
	fmt.Println()
}
