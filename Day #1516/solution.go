// Reverse words in a space-delimited string.
// Approach: reverse whole byte slice, then reverse each word in place (in-place).
// Time: O(n), Space: O(1) extra.
package main

import "fmt"

func reverseRange(s []byte, i, j int) {
	for i < j {
		s[i], s[j] = s[j], s[i]
		i++
		j--
	}
}

func reverseWords(str string) string {
	s := []byte(str)
	n := len(s)
	reverseRange(s, 0, n-1)
	start := 0
	for i := 0; i <= n; i++ {
		if i == n || s[i] == ' ' {
			reverseRange(s, start, i-1)
			start = i + 1
		}
	}
	return string(s)
}

func main() {
	fmt.Printf("\"%s\"\n", reverseWords("hello world here")) // "here world hello"
}
