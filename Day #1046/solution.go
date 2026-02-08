// Shortest palindrome by inserting chars; lexicographically earliest on ties.
// DP dp[i][j]=min insertions; memoized build(i,j) reconstructs string. Time O(n^2), Space O(n^2).
package main

import "fmt"

func solve(s string) string {
	n := len(s)
	if n == 0 {
		return ""
	}
	dp := make([][]int, n)
	for i := range dp {
		dp[i] = make([]int, n)
	}
	for length := 2; length <= n; length++ {
		for i := 0; i+length-1 < n; i++ {
			j := i + length - 1
			if s[i] == s[j] {
				if i+1 <= j-1 {
					dp[i][j] = dp[i+1][j-1]
				} else {
					dp[i][j] = 0
				}
			} else {
				a, b := dp[i+1][j], dp[i][j-1]
				if a < b {
					dp[i][j] = 1 + a
				} else {
					dp[i][j] = 1 + b
				}
			}
		}
	}
	memo := make([][]string, n)
	done := make([][]bool, n)
	for i := range memo {
		memo[i] = make([]string, n)
		done[i] = make([]bool, n)
	}
	var build func(i, j int) string
	build = func(i, j int) string {
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
			res = string(s[i]) + build(i+1, j-1) + string(s[i])
		} else {
			a := string(s[i]) + build(i+1, j) + string(s[i])
			b := string(s[j]) + build(i, j-1) + string(s[j])
			if dp[i+1][j] < dp[i][j-1] {
				res = a
			} else if dp[i][j-1] < dp[i+1][j] {
				res = b
			} else if a <= b {
				res = a
			} else {
				res = b
			}
		}
		done[i][j] = true
		memo[i][j] = res
		return res
	}
	return build(0, n-1)
}

func main() {
	fmt.Println("race ->", solve("race"))
	fmt.Println("google ->", solve("google"))
}
