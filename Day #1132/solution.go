// Left-rotate slice in place by k using 3 reversals. O(n) time, O(1) space.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func reverseRange(a []int, i, j int) {
	for i < j {
		a[i], a[j] = a[j], a[i]
		i++
		j--
	}
}

func rotateLeft(a []int, k int) {
	n := len(a)
	if n == 0 {
		return
	}
	k %= n
	reverseRange(a, 0, k-1)
	reverseRange(a, k, n-1)
	reverseRange(a, 0, n-1)
}

func main() {
	a := []int{1, 2, 3, 4, 5, 6}
	rotateLeft(a, 2)
	parts := make([]string, len(a))
	for i, v := range a {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
