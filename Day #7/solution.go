// Decode ways: DP where ways[i] depends on 1-digit and valid 2-digit suffixes.
// Time: O(n), Space: O(1).
package main

import "fmt"

func numDecodings(s string) int {
	if len(s) == 0 || s[0] == '0' {
		return 0
	}
	prev2, prev1 := 1, 1
	for i := 1; i < len(s); i++ {
		cur := 0
		if s[i] != '0' {
			cur += prev1
		}
		two := int(s[i-1]-'0')*10 + int(s[i]-'0')
		if two >= 10 && two <= 26 {
			cur += prev2
		}
		prev2, prev1 = prev1, cur
	}
	return prev1
}

func main() {
	fmt.Println(numDecodings("111"))
}
