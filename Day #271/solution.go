// Day 271: Search sorted list without *, /, or bit-shift -> Fibonacci search.
// Uses only addition/subtraction/comparison. Time O(log N), Space O(1).
package main

import "fmt"

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func fibSearch(arr []int, x int) int {
	n := len(arr)
	fibMm2, fibMm1 := 0, 1
	fibM := fibMm2 + fibMm1
	for fibM < n {
		fibMm2 = fibMm1
		fibMm1 = fibM
		fibM = fibMm2 + fibMm1
	}
	offset := -1
	for fibM > 1 {
		i := min(offset+fibMm2, n-1)
		if arr[i] < x {
			fibM = fibMm1
			fibMm1 = fibMm2
			fibMm2 = fibM - fibMm1
			offset = i
		} else if arr[i] > x {
			fibM = fibMm2
			fibMm1 = fibMm1 - fibMm2
			fibMm2 = fibM - fibMm1
		} else {
			return i
		}
	}
	if fibMm1 != 0 && offset+1 < n && arr[offset+1] == x {
		return offset + 1
	}
	return -1
}

func main() {
	arr := []int{1, 3, 4, 7, 9, 11, 15}
	fmt.Println("7 -> index", fibSearch(arr, 7)) // 3
	fmt.Println("8 -> index", fibSearch(arr, 8)) // -1
}
