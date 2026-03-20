// Fewest insertions for palindrome; lexicographically smallest among minima.
// Interval DP with memoized reconstruction. Time/Space O(n^2).
package main

import "fmt"

var s string
var memo map[int]string

func build(i, j int) string {
	if i > j {
		return ""
	}
	if i == j {
		return string(s[i])
	}
	key := i*len(s) + j
	if v, ok := memo[key]; ok {
		return v
	}
	var res string
	if s[i] == s[j] {
		res = string(s[i]) + build(i+1, j-1) + string(s[i])
	} else {
		left := string(s[i]) + build(i+1, j) + string(s[i])
		right := string(s[j]) + build(i, j-1) + string(s[j])
		if len(left) != len(right) {
			if len(left) < len(right) {
				res = left
			} else {
				res = right
			}
		} else if left <= right {
			res = left
		} else {
			res = right
		}
	}
	memo[key] = res
	return res
}

func makePalindrome(str string) string {
	s = str
	memo = make(map[int]string)
	return build(0, len(s)-1)
}

func main() {
	fmt.Println(makePalindrome("race"))
	fmt.Println(makePalindrome("google"))
}
