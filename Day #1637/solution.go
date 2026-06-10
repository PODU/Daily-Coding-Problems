// Binary search without *, /, or bit-shift; midpoint via two-pointer walk (only ++/--).
// Time: O(log N), Space: O(1).
package main

import "fmt"

func midpoint(lo, hi int) int {
	i, j := lo, hi
	for i < j {
		i++
		j--
	}
	return j // floor((lo+hi)/2) using only ++/--
}

func contains(arr []int, x int) bool {
	lo, hi := 0, len(arr)-1
	for lo <= hi {
		mid := midpoint(lo, hi)
		if arr[mid] == x {
			return true
		} else if arr[mid] < x {
			lo = mid + 1
		} else {
			hi = mid - 1
		}
	}
	return false
}

func main() {
	arr := []int{1, 3, 5, 7, 9, 11, 13}
	if contains(arr, 7) {
		fmt.Println("true")
	} else {
		fmt.Println("false")
	}
	if contains(arr, 8) {
		fmt.Println("true")
	} else {
		fmt.Println("false")
	}
}
