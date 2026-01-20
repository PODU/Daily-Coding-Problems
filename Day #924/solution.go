// Smallest window to sort: scan left->right tracking max for right bound,
// right->left tracking min for left bound. Time O(n), Space O(1).
package main

import "fmt"

func findUnsortedWindow(a []int) (int, int) {
	n := len(a)
	right, runMax := -1, a[0]
	for i := 1; i < n; i++ {
		if a[i] < runMax {
			right = i
		} else {
			runMax = a[i]
		}
	}
	left, runMin := -1, a[n-1]
	for i := n - 2; i >= 0; i-- {
		if a[i] > runMin {
			left = i
		} else {
			runMin = a[i]
		}
	}
	return left, right
}

func main() {
	arr := []int{3, 7, 5, 6, 9}
	l, r := findUnsortedWindow(arr)
	fmt.Printf("(%d, %d)\n", l, r)
}
