// Day 1099: Rotate array right by k in-place using the reversal algorithm.
// Reverse whole, reverse first k, reverse rest. Time: O(N). Space: O(1).
package main

import "fmt"

func reverse(a []int, i, j int) {
	for i < j {
		a[i], a[j] = a[j], a[i]
		i++
		j--
	}
}

func rotate(a []int, k int) {
	n := len(a)
	if n == 0 {
		return
	}
	k %= n
	reverse(a, 0, n-1)
	reverse(a, 0, k-1)
	reverse(a, k, n-1)
}

func main() {
	a := []int{1, 2, 3, 4, 5, 6, 7}
	rotate(a, 3)
	fmt.Println(a) // [5 6 7 1 2 3 4]
}
