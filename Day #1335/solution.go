// Day 1335: Count decodings of a digit string with a=1..z=26.
// DP: ways[i] += ways[i-1] if digit valid, += ways[i-2] if two-digit 10..26 valid. O(n) time, O(1) space.
package main

import "fmt"

func numDecodings(s string) int64 {
	n := len(s)
	if n == 0 {
		return 0
	}
	var prev2 int64 = 1
	var prev1 int64
	if s[0] != '0' {
		prev1 = 1
	}
	for i := 1; i < n; i++ {
		var cur int64
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
