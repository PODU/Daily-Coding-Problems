// Day 211: All occurrences of pattern in string via KMP.
// Approach: Knuth-Morris-Pratt with LPS failure function. Time O(n+m), Space O(m).
package main

import "fmt"

func findOccurrences(s, p string) []int {
	m, n := len(p), len(s)
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
	fmt.Println(findOccurrences("abracadabra", "abr"))
}
