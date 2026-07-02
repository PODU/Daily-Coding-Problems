// Day 1753: Egyptian fraction (sum of distinct unit fractions).
// Greedy Fibonacci-Sylvester: repeatedly subtract 1/ceil(b/a). Time O(terms), O(1) space.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func gcd(x, y int64) int64 {
	for y != 0 {
		x, y = y, x%y
	}
	return x
}

func egyptian(a, b int64) string {
	var terms []string
	for a != 0 {
		c := (b + a - 1) / a // ceil(b/a)
		terms = append(terms, "1 / "+strconv.FormatInt(c, 10))
		a = a*c - b
		b = b * c
		na := a
		if na < 0 {
			na = -na
		}
		g := gcd(na, b)
		if g > 1 {
			a /= g
			b /= g
		}
	}
	return strings.Join(terms, " + ")
}

func main() {
	// README example: 4 / 13 -> 1 / 4 + 1 / 18 + 1 / 468
	fmt.Println(egyptian(4, 13))
}
