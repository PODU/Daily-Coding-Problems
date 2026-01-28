// Day 973: Count ways to decode a digit string (a=1..z=26).
// Approach: DP, dp[i]=dp[i-1] if 1-digit valid + dp[i-2] if 2-digit valid. Time O(n), Space O(1).
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
	fmt.Println(numDecodings("111")) // 3
}
