// Day 444: KMP string matching in O(N + k) time, O(k) space (beats O(N*k)).
// Returns the start index of the first match, or -1 (False) if absent.
package main

import "fmt"

func buildLPS(p string) []int {
	lps := make([]int, len(p))
	length := 0
	for i := 1; i < len(p); {
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
	return lps
}

func kmpSearch(text, pat string) int {
	if len(pat) == 0 {
		return 0
	}
	lps := buildLPS(pat)
	i, j := 0, 0
	for i < len(text) {
		if text[i] == pat[j] {
			i++
			j++
			if j == len(pat) {
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
	idx := kmpSearch("abxabcabcaby", "abcaby")
	if idx == -1 {
		fmt.Println("False")
	} else {
		fmt.Println(idx) // 6
	}
}
