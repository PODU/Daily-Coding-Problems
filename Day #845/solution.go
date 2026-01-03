// Day 845: rotate a list left by k in place using the 3-reversal trick.
// reverse(0,k-1), reverse(k,n-1), reverse(0,n-1). O(n) time, O(1) extra space.
package main

import "fmt"

func reverse(a []int, i, j int) {
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
	k = ((k % n) + n) % n
	reverse(a, 0, k-1)
	reverse(a, k, n-1)
	reverse(a, 0, n-1)
}

func main() {
	a := []int{1, 2, 3, 4, 5, 6}
	rotateLeft(a, 2)
	fmt.Println(a) // [3 4 5 6 1 2]
}
