// Closure late-binding: closures over one shared variable all read its final value (9);
// fix = copy the loop var per iteration. O(n) build/call, O(n) space.
package main

import "fmt"

func main() {
	// Buggy: every closure captures the SAME shared variable, which ends at 9.
	var buggy []func() int
	shared := 0
	for t := 0; t < 10; t++ {
		shared = t
		buggy = append(buggy, func() int { return shared })
	}
	for _, f := range buggy {
		fmt.Println(f())
	}

	fmt.Println("---")

	// Fixed: shadow with a per-iteration copy so each closure keeps its own value.
	var fixed []func() int
	for t := 0; t < 10; t++ {
		t := t
		fixed = append(fixed, func() int { return t })
	}
	for _, f := range fixed {
		fmt.Println(f())
	}
}
