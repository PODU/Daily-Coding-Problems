// Day 91: Closure-in-loop. Capturing the loop variable by reference shares it;
// bind a per-iteration copy so each closure keeps its own value. O(n).
package main

import "fmt"

func main() {
	broken := []func() int{}
	shared := 0
	for i := 0; i < 10; i++ {
		broken = append(broken, func() int { return shared })
	}
	shared = 9 // mirrors Python: all closures see the final value

	fixed := []func() int{}
	for i := 0; i < 10; i++ {
		i := i // shadow with a per-iteration copy
		fixed = append(fixed, func() int { return i })
	}

	fmt.Print("Broken (prints 9 ten times):")
	for _, f := range broken {
		fmt.Printf(" %d", f())
	}
	fmt.Print("\nFixed:")
	for _, f := range fixed {
		fmt.Printf(" %d", f())
	}
	fmt.Println()
}
