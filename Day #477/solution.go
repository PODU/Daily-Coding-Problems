// Closure capture: closures over one shared variable all read its final value (9).
// Fix by shadowing with a per-iteration copy (j := j) so each closure captures its own -> 0..9.
// (Python analogue: lambda: i prints 9 ten times; fix is lambda i=i: i.)
package main

import "fmt"

func main() {
	var buggy []func() int
	shared := 0
	for i := 0; i < 10; i++ {
		shared = i
		buggy = append(buggy, func() int { return shared }) // all close over `shared`
	}
	fmt.Println("Buggy:")
	for _, f := range buggy {
		fmt.Println(f())
	}

	var fixed []func() int
	for j := 0; j < 10; j++ {
		j := j // per-iteration copy
		fixed = append(fixed, func() int { return j })
	}
	fmt.Println("Fixed:")
	for _, f := range fixed {
		fmt.Println(f())
	}
}
