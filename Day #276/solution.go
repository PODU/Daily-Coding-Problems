// Day 276: KMP pattern search. Time O(N + k), Space O(k) -- beats O(N*k).
// Returns start index of first match, or -1 (False).
package main

import "fmt"

func kmp(text, pat string) int {
	n, k := len(text), len(pat)
	if k == 0 {
		return 0
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
	for i, j := 0, 0; i < n; {
		if text[i] == pat[j] {
			i++
			j++
			if j == k {
				return i - j
			}
		} else if j != 0 {
			j = lps[j-1]
		} else {
			i++
		}
	}
	return -1
}

func main() {
	fmt.Println(kmp("abxabcabcaby", "abcaby")) // 6
	fmt.Println(kmp("abxabcabcaby", "zzz"))    // -1
}
