// Day 1471: O(log N) search in a sorted array with no *, /, or bit-shift.
// Build powers of two by doubling (addition); jump-based binary search.
// Time O(log N), Space O(log N).
package main

import "fmt"

func search(a []int, x int) bool {
	n := len(a)
	if n == 0 {
		return false
	}
	powers := []int{}
	for p := 1; p <= n; p = p + p {
		powers = append(powers, p)
	}
	pos := -1
	for i := len(powers) - 1; i >= 0; i-- {
		nxt := pos + powers[i]
		if nxt < n && a[nxt] <= x {
			pos = nxt
		}
	}
	return pos >= 0 && a[pos] == x
}

func main() {
	arr := []int{1, 3, 5, 7, 9, 11}
	fmt.Println(search(arr, 7)) // true
	fmt.Println(search(arr, 8)) // false
}
