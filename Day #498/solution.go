// Smallest window to sort so the whole array is sorted.
// Two passes: left->right track max for right bound; right->left track min for left bound.
// Time: O(n), Space: O(1).
package main

import "fmt"

func windowToSort(a []int) (int, int) {
	n := len(a)
	left, right := -1, -1
	maxSoFar := a[0]
	for i := 1; i < n; i++ {
		if a[i] < maxSoFar {
			right = i
		} else {
			maxSoFar = a[i]
		}
	}
	minSoFar := a[n-1]
	for i := n - 2; i >= 0; i-- {
		if a[i] > minSoFar {
			left = i
		} else {
			minSoFar = a[i]
		}
	}
	return left, right
}

func main() {
	a := []int{3, 7, 5, 6, 9}
	left, right := windowToSort(a)
	fmt.Printf("(%d, %d)\n", left, right)
}
