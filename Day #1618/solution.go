// Sevenish: write n in binary; each set bit k contributes 7^k. Time O(log n), Space O(1).
package main

import (
	"fmt"
	"strings"
)

func sevenish(n int) int {
	total, pow7 := 0, 1
	for n > 0 {
		if n&1 == 1 {
			total += pow7
		}
		pow7 *= 7
		n >>= 1
	}
	return total
}

func main() {
	parts := make([]string, 0, 5)
	for n := 1; n <= 5; n++ {
		parts = append(parts, fmt.Sprintf("%d", sevenish(n)))
	}
	fmt.Println(strings.Join(parts, " "))
}
