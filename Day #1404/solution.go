// KMP: build LPS, scan once, record every full match start. Time O(N+k), Space O(k).
package main

import "fmt"

func findAll(s, pat string) []int {
	res := []int{}
	N, k := len(s), len(pat)
	if k == 0 || k > N {
		return res
	}
	lps := make([]int, k)
	for i, length := 1, 0; i < k; {
		if pat[i] == pat[length] {
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
	for i, j := 0, 0; i < N; {
		if s[i] == pat[j] {
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
	fmt.Println(findAll("abracadabra", "abr")) // [0 7]
}
