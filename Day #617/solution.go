// Roman numeral -> decimal. Single left-to-right pass; subtract when a smaller
// value precedes a larger one. Time O(n), Space O(1).
package main

import "fmt"

func romanToInt(s string) int {
	val := map[byte]int{'M': 1000, 'D': 500, 'C': 100, 'L': 50, 'X': 10, 'V': 5, 'I': 1}
	total := 0
	for i := 0; i < len(s); i++ {
		v := val[s[i]]
		if i+1 < len(s) && val[s[i+1]] > v {
			total -= v
		} else {
			total += v
		}
	}
	return total
}

func main() {
	fmt.Println(romanToInt("XIV")) // 14
}
