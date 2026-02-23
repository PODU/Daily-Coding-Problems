// Day 1118 - Late-binding closure pitfall
// Closing over a shared variable makes every func see its final value (9).
// Fix: shadow the variable per iteration so each closure binds its own copy.
package main

import "fmt"

func main() {
	var buggy, fixed []func() int
	holder := 0
	for k := 0; k < 10; k++ {
		holder = k
		buggy = append(buggy, func() int { return holder }) // shared variable
		k := k                                               // per-iteration copy
		fixed = append(fixed, func() int { return k })
	}

	fmt.Println("Buggy output (all 9):")
	for _, f := range buggy {
		fmt.Println(f())
	}
	fmt.Println("Fixed output (0-9):")
	for _, f := range fixed {
		fmt.Println(f())
	}
}
