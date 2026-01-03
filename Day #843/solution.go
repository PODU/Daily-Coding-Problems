// Day 843: find all start indices of pattern in string using KMP.
// Build failure table, single scan. O(n+m) time, O(m) space.
package main

import "fmt"

func kmpSearch(s, p string) []int {
	n, m := len(s), len(p)
	res := []int{}
	if m == 0 || m > n {
		return res
	}
	lps := make([]int, m)
	for i, length := 1, 0; i < m; {
		if p[i] == p[length] {
			length++
			lps[i] = length
			i++
		} else if length != 0 {
			length = lps[length-1]
		} else {
			lps[i] = 0
			i++
		}
	}
	for i, j := 0, 0; i < n; {
		if s[i] == p[j] {
			i++
			j++
			if j == m {
				res = append(res, i-m)
				j = lps[j-1]
			}
		} else if j != 0 {
			j = lps[j-1]
		} else {
			i++
		}
	}
	return res
}

func main() {
	fmt.Println(kmpSearch("abracadabra", "abr")) // [0 7]
}
