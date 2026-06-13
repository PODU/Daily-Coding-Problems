// Closure capture demo: buggy closures capture a shared variable by reference (final value);
// fix shadows a per-iteration copy. O(n) time/space.
package main

import "fmt"

func main() {
	fmt.Println("Late binding (buggy):")
	var buggy []func() int
	v := 0
	for _, x := range []int{1, 2, 3} {
		v = x
		buggy = append(buggy, func() int { return v })
	}
	for _, f := range buggy {
		fmt.Println(f())
	}
	fmt.Println("Fixed (capture value):")
	var fixed []func() int
	for _, x := range []int{1, 2, 3} {
		x := x
		fixed = append(fixed, func() int { return x })
	}
	for _, f := range fixed {
		fmt.Println(f())
	}
}
