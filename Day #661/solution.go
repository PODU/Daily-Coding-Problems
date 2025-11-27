// Day 661: Search sorted array without mul/div/bit-shift.
// Fibonacci search uses only +/- to pick probe points. Time O(log N), Space O(1).
package main

import "fmt"

func fibSearch(a []int, x int) int {
	n := len(a)
	f2, f1 := 0, 1
	f := f1 + f2
	for f < n {
		f2, f1 = f1, f
		f = f1 + f2
	}
	offset := -1
	for f > 1 {
		i := offset + f2
		if i > n-1 {
			i = n - 1
		}
		if a[i] < x {
			f, f1 = f1, f2
			f2 = f - f1
			offset = i
		} else if a[i] > x {
			f = f2
			f1 = f1 - f2
			f2 = f - f1
		} else {
			return i
		}
	}
	if f1 != 0 && offset+1 < n && a[offset+1] == x {
		return offset + 1
	}
	return -1
}

func main() {
	a := []int{-1, 0, 3, 5, 9, 12}
	fmt.Println("Index of 5:", fibSearch(a, 5)) // 3
	fmt.Println("Index of 7:", fibSearch(a, 7)) // -1 (absent)
}
