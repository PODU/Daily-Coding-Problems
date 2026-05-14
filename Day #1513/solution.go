// KMP pattern matching: build failure (LPS) array, then scan once collecting all match starts.
// Time: O(N+k), Space: O(k).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func findAll(s, p string) []int {
	n, k := len(s), len(p)
	res := []int{}
	if k == 0 || k > n {
		return res
	}
	lps := make([]int, k)
	for i, length := 1, 0; i < k; {
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
			if j == k {
				res = append(res, i-k)
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
	r := findAll("abracadabra", "abr")
	parts := make([]string, len(r))
	for i, v := range r {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
