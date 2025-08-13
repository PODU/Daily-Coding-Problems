// Day 113: Reverse word order in-place: reverse whole, then each word. O(n) time, O(1) extra.
package main

import "fmt"

func rev(s []byte, i, j int) {
	for i < j {
		s[i], s[j] = s[j], s[i]
		i++
		j--
	}
}

func reverseWords(str string) string {
	s := []byte(str)
	n := len(s)
	rev(s, 0, n-1)
	start := 0
	for i := 0; i <= n; i++ {
		if i == n || s[i] == ' ' {
			rev(s, start, i-1)
			start = i + 1
		}
	}
	return string(s)
}

func main() {
	fmt.Printf("%q\n", reverseWords("hello world here")) // "here world hello"
}
