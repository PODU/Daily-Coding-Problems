// KMP substring search: build LPS array, then single scan.
// Time: O(N + k), Space: O(k).
package main

import "fmt"

// returns (index, true) or (-1, false)
func kmpSearch(s, pat string) (int, bool) {
	N, k := len(s), len(pat)
	if k == 0 {
		return 0, true
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
				return i - k, true
			}
		} else if j != 0 {
			j = lps[j-1]
		} else {
			i++
		}
	}
	return -1, false
}

func main() {
	idx, ok := kmpSearch("abracadabra", "cad")
	if ok {
		fmt.Println(idx)
	} else {
		fmt.Println("False")
	}
}
