// Roman to decimal: add each value, subtract when a smaller numeral precedes a larger. Time O(n), Space O(1).
package main

import "fmt"

func romanToInt(s string) int {
	v := map[byte]int{'M': 1000, 'D': 500, 'C': 100, 'L': 50, 'X': 10, 'V': 5, 'I': 1}
	total := 0
	for i := 0; i < len(s); i++ {
		if i+1 < len(s) && v[s[i]] < v[s[i+1]] {
			total -= v[s[i]]
		} else {
			total += v[s[i]]
		}
	}
	return total
}

func main() {
	fmt.Println(romanToInt("XIV"))
}
