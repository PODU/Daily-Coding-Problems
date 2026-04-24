// Day 1419: reverse the order of space-delimited words, in-place.
// Approach: reverse whole byte slice, then reverse each word. O(n) time, O(1) extra space.
package main

import "fmt"

func reverseRange(s []byte, i, j int) {
	for i < j {
		s[i], s[j] = s[j], s[i]
		i++
		j--
	}
}

func reverseWords(s []byte) {
	n := len(s)
	reverseRange(s, 0, n-1)
	start := 0
	for i := 0; i <= n; i++ {
		if i == n || s[i] == ' ' {
			reverseRange(s, start, i-1)
			start = i + 1
		}
	}
}

func main() {
	s := []byte("hello world here")
	reverseWords(s)
	fmt.Println(string(s)) // here world hello
}
