// Shortest palindrome by insertions, lexicographically earliest: memoized DP on
// (i,j) building best palindrome for s[i..j]. Time O(n^2) states, Space O(n^2).
package main

import "fmt"

func shortestPalindrome(s string) string {
	n := len(s)
	memo := make([][]string, n)
	done := make([][]bool, n)
	for i := range memo {
		memo[i] = make([]string, n)
		done[i] = make([]bool, n)
	}
	var solve func(i, j int) string
	solve = func(i, j int) string {
		if i > j {
			return ""
		}
		if i == j {
			return string(s[i])
		}
		if done[i][j] {
			return memo[i][j]
		}
		var res string
		if s[i] == s[j] {
			res = string(s[i]) + solve(i+1, j-1) + string(s[i])
		} else {
			opt1 := string(s[i]) + solve(i+1, j) + string(s[i])
			opt2 := string(s[j]) + solve(i, j-1) + string(s[j])
			if len(opt1) < len(opt2) {
				res = opt1
			} else if len(opt2) < len(opt1) {
				res = opt2
			} else if opt1 <= opt2 {
				res = opt1
			} else {
				res = opt2
			}
		}
		done[i][j] = true
		memo[i][j] = res
		return res
	}
	return solve(0, n-1)
}

func main() {
	fmt.Println(shortestPalindrome("race"))
	fmt.Println(shortestPalindrome("google"))
}
