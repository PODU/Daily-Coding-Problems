// KMP string matching: build LPS failure array O(k), scan text O(N). Time O(N+k), Space O(k).
package main

import "fmt"

func kmpSearch(text, pat string) int {
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
	text := "abxabcabcaby"
	r1 := kmpSearch(text, "abcaby")
	if r1 == -1 {
		fmt.Println("Not found")
	} else {
		fmt.Printf("Found at index %d\n", r1)
	}
	r2 := kmpSearch(text, "xyz")
	if r2 == -1 {
		fmt.Println("Not found")
	} else {
		fmt.Printf("Found at index %d\n", r2)
	}
}
