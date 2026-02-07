// Reverse word order in-place: reverse whole char array, then reverse each word.
// Time: O(n), Space: O(1) extra (on the mutable rune/byte buffer).
package main

import "fmt"

func reverseRange(a []byte, i, j int) {
	for i < j {
		a[i], a[j] = a[j], a[i]
		i++
		j--
	}
}

func reverseWords(a []byte) {
	reverseRange(a, 0, len(a)-1)
	start := 0
	for i := 0; i <= len(a); i++ {
		if i == len(a) || a[i] == ' ' {
			reverseRange(a, start, i-1)
			start = i + 1
		}
	}
}

func main() {
	a := []byte("hello world here")
	reverseWords(a)
	fmt.Println("\"" + string(a) + "\"")
}
