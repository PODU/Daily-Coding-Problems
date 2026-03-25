// Day 1262: Closure-in-a-loop (late binding) demonstrated in Go.
// Capturing a shared variable makes every closure print its final value.
// Fix: give each iteration its own copy of the value (i := i).
package main

import "fmt"

func main() {
	// Buggy analogue: all closures read the same shared variable.
	shared := 0
	var buggy []func()
	for i := 1; i <= 3; i++ {
		shared = i
		buggy = append(buggy, func() { fmt.Println(shared) })
	}

	// Fixed: each closure captures its own per-iteration copy.
	var fixed []func()
	for i := 1; i <= 3; i++ {
		i := i
		fixed = append(fixed, func() { fmt.Println(i) })
	}

	fmt.Println("Buggy output:")
	for _, f := range buggy {
		f()
	}
	fmt.Println("Fixed output:")
	for _, f := range fixed {
		f()
	}
}
