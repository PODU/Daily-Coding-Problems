// Egyptian fraction via greedy: repeatedly take ceil(b/a) as next unit denominator.
// Time: O(number of terms) iterations; Space: O(terms). a/b proper (a<b).
package main

import (
	"fmt"
	"strings"
)

func egyptian(a, b int64) []int64 {
	var denoms []int64
	for a != 0 {
		d := (b + a - 1) / a // ceil(b/a)
		denoms = append(denoms, d)
		a = a*d - b
		b = b * d
	}
	return denoms
}

func main() {
	denoms := egyptian(4, 13)
	parts := make([]string, len(denoms))
	for i, d := range denoms {
		parts[i] = fmt.Sprintf("1 / %d", d)
	}
	fmt.Println(strings.Join(parts, " + "))
}
