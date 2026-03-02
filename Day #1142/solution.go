// Day 1142: nth sevenish number = sum of distinct powers of 7.
// Bits of n in binary select powers of 7 (bijection). Time O(log n), Space O(1).
package main

import "fmt"

func sevenish(n int64) int64 {
	var result, power int64 = 0, 1
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
	for i := int64(1); i <= 5; i++ {
		fmt.Printf("%d ", sevenish(i)) // 1 7 8 49 50
	}
	fmt.Println()
}
