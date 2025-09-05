// Day 221: nth "sevenish" number (sum of distinct powers of 7).
// Approach: bits of n select which powers of 7 to include (bijection with binary). Time O(log n), Space O(1).
package main

import (
	"fmt"
	"strings"
)

func sevenish(n int64) int64 {
	var result, power int64 = 0, 1 // 7^0
	for n > 0 {
		if n&1 == 1 {
			result += power
		}
		power *= 7
		n >>= 1
	}
	return result
}

func main() {
	parts := []string{}
	for i := int64(1); i <= 5; i++ {
		parts = append(parts, fmt.Sprintf("%d", sevenish(i)))
	}
	fmt.Println(strings.Join(parts, " ")) // 1 7 8 49 50
	fmt.Println(sevenish(4))              // 49
}
