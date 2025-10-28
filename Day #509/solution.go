// Day 509: Fewest-insertion palindrome with lexicographically earliest result.
// Memoized interval DP building the actual string. Time O(n^3), Space O(n^2).
package main

import "fmt"

func solve(s string) string {
	n := len(s)
	if n == 0 {
		return ""
	}
	memo := make([]string, n*n)
	done := make([]bool, n*n)

	var build func(i, j int) string
	build = func(i, j int) string {
		if i > j {
			return ""
		}
		if i == j {
			return string(s[i])
		}
		key := i*n + j
		if done[key] {
			return memo[key]
		}
		done[key] = true
		var res string
		if s[i] == s[j] {
			res = string(s[i]) + build(i+1, j-1) + string(s[j])
		} else {
			a := string(s[i]) + build(i+1, j) + string(s[i])
			b := string(s[j]) + build(i, j-1) + string(s[j])
			if len(a) < len(b) {
				res = a
			} else if len(b) < len(a) {
				res = b
			} else if a <= b {
				res = a
			} else {
				res = b
			}
		}
		memo[key] = res
		return res
	}

	return build(0, n-1)
}

func main() {
	fmt.Println(solve("race"))
	fmt.Println(solve("google"))
}
