// Day 188: Closure-in-a-loop late binding (Go analog of the Python snippet).
// Closures sharing one variable `i` all read 3 after the loop.
// Fix: shadow with a per-iteration copy. Time/Space O(n).
package main

import "fmt"

func makeFunctionsBuggy() []func() int {
	var funcs []func() int
	i := 0
	for _, v := range []int{1, 2, 3} {
		i = v
		funcs = append(funcs, func() int { return i }) // shares i -> 3
	}
	return funcs
}

func makeFunctionsFixed() []func() int {
	var funcs []func() int
	for _, v := range []int{1, 2, 3} {
		v := v // per-iteration copy
		funcs = append(funcs, func() int { return v })
	}
	return funcs
}

func main() {
	fmt.Println("Late binding prints:")
	for _, f := range makeFunctionsBuggy() {
		fmt.Println(f())
	}
	fmt.Println("Fixed prints:")
	for _, f := range makeFunctionsFixed() {
		fmt.Println(f())
	}
}
