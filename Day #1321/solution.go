// Day 1321: Roman numeral -> decimal.
// Approach: single left-to-right pass; subtract if a smaller value precedes a larger one. O(n) time, O(1) space.
package main

import "fmt"

func romanToInt(s string) int {
	v := map[byte]int{'M': 1000, 'D': 500, 'C': 100, 'L': 50, 'X': 10, 'V': 5, 'I': 1}
	total := 0
	for i := 0; i < len(s); i++ {
		cur := v[s[i]]
		if i+1 < len(s) && cur < v[s[i+1]] {
			total -= cur
		} else {
			total += cur
		}
	}
	return total
}

func main() {
	fmt.Println(romanToInt("XIV")) // 14
}
