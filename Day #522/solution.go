// KMP: build prefix-function for pattern, scan text. O(n+m) time, O(m) space.
package main

import "fmt"

func kmpSearch(s, p string) []int {
	m := len(p)
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
	res := []int{}
	for i, j := 0, 0; i < len(s); {
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
	fmt.Println(kmpSearch("abracadabra", "abr"))
}
