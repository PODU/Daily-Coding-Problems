// Closure capture demo: "buggy" closures share a pointer to one variable (prints 9 x10);
// "fixed" closures capture the per-iteration value (prints 0..9). Time O(n), Space O(n).
package main

import "fmt"

func main() {
	// Buggy: every closure captures the SAME variable via pointer; final value is 9.
	var buggy []func() int
	shared := 0
	for i := 0; i < 10; i++ {
		shared = i
		p := &shared
		buggy = append(buggy, func() int { return *p })
	}
	for _, f := range buggy {
		fmt.Println(f())
	}

	fmt.Println()

	// Fixed: capture the per-iteration value by copying it into a local.
	var fixed []func() int
	for i := 0; i < 10; i++ {
		v := i
		fixed = append(fixed, func() int { return v })
	}
	for _, f := range fixed {
		fmt.Println(f())
	}
}
