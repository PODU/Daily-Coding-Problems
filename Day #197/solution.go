// Day 197: Rotate array right by k in-place.
// Triple-reversal: reverse whole, reverse first k, reverse rest. O(n) time, O(1) space.
package main

import "fmt"

func reverse(a []int, i, j int) {
	for i < j {
		a[i], a[j] = a[j], a[i]
		i++
		j--
	}
}

func rotateRight(a []int, k int) {
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
	a := []int{1, 2, 3, 4, 5}
	rotateRight(a, 2)
	fmt.Println(a) // [4 5 1 2 3]
}
