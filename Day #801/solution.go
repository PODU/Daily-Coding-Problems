// Day 801: nth sevenish number = sum of unique powers of 7.
// Bits of n select which powers of 7 to add (base-7 analog of binary).
// Time O(log n), Space O(1).
package main

import (
	"fmt"
	"strings"
)

func sevenish(n int) int {
	result, power := 0, 1
	for n != 0 {
		if n&1 == 1 {
			result += power
		}
		power *= 7
		n >>= 1
	}
	return result
}

func main() {
	var parts []string
	for i := 1; i <= 5; i++ {
		parts = append(parts, fmt.Sprintf("%d", sevenish(i)))
	}
	fmt.Println(strings.Join(parts, " ")) // 1 7 8 49 50
}
