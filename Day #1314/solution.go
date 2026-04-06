// Egyptian fraction via Fibonacci/Sylvester greedy: repeatedly subtract the
// largest unit fraction 1/ceil(b/a). Time O(#terms), Space O(1).
package main

import (
	"fmt"
	"strings"
)

func egyptian(a, b int64) string {
	var terms []string
	for a != 0 {
		x := (b + a - 1) / a // ceil(b/a)
		terms = append(terms, fmt.Sprintf("1 / %d", x))
		a, b = a*x-b, b*x
	}
	return strings.Join(terms, " + ")
}

func main() {
	fmt.Println(egyptian(4, 13)) // 1 / 4 + 1 / 18 + 1 / 468
}
