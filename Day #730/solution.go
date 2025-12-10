// Day 730: Closure-in-a-loop late binding (Go analogue).
// Capturing a shared variable -> all closures see its final value (3,3,3).
// Fix: shadow the variable per iteration so each closure keeps its own copy (1,2,3).
package main

import "fmt"

func main() {
	// Buggy: shared variable captured by all closures
	var buggy []func()
	var i int
	for _, v := range []int{1, 2, 3} {
		i = v
		buggy = append(buggy, func() { fmt.Println(i) })
	}
	for _, f := range buggy {
		f() // 3, 3, 3
	}

	// Fixed: per-iteration shadowed variable
	var fixed []func()
	for _, v := range []int{1, 2, 3} {
		v := v
		fixed = append(fixed, func() { fmt.Println(v) })
	}
	for _, f := range fixed {
		f() // 1, 2, 3
	}
}
