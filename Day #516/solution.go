// Sevenish: nth value = sum of 7^i for each set bit i of n. O(log n) per query.
package main

import (
	"fmt"
	"strings"
)

func sevenish(n int64) int64 {
	var sum, pow7 int64 = 0, 1
	for n > 0 {
		if n&1 == 1 {
			sum += pow7
		}
		pow7 *= 7
		n >>= 1
	}
	return sum
}

func main() {
	// First few sevenish numbers: 1, 7, 8, 49, ...
	var parts []string
	for n := int64(1); n <= 4; n++ {
		parts = append(parts, fmt.Sprint(sevenish(n)))
	}
	fmt.Println(strings.Join(parts, ", "))
}
