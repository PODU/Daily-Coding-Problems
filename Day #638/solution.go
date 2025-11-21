// Day 638: Reverse the words in a string (in-place on a byte slice).
// Approach: reverse whole slice, then reverse each word back.
// Time: O(n), Space: O(1) extra.
package main

import "fmt"

func rev(a []byte, i, j int) {
	for i < j {
		a[i], a[j] = a[j], a[i]
		i++
		j--
	}
}

func reverseWords(s string) string {
	a := []byte(s)
	n := len(a)
	rev(a, 0, n-1)
	i := 0
	for i < n {
		j := i
		for j < n && a[j] != ' ' {
			j++
		}
		rev(a, i, j-1)
		i = j + 1
	}
	return string(a)
}

func main() {
	fmt.Println(reverseWords("hello world here")) // here world hello
}
