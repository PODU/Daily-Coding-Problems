// Egyptian fraction via greedy (Fibonacci/Sylvester): take ceil(b/a) each step.
// Time: O(number of terms), Space: O(1). int64.
package main

import (
	"fmt"
	"strings"
)

func main() {
	var a, b int64 = 4, 13
	var terms []string
	for a != 0 {
		x := (b + a - 1) / a // ceil(b/a)
		terms = append(terms, fmt.Sprintf("1 / %d", x))
		a = a*x - b
		b = b * x
	}
	fmt.Println(strings.Join(terms, " + "))
}
