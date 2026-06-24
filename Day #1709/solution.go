// Non-decreasing with <=1 change: single pass, on violation greedily lower a[i-1] or raise a[i]. O(n).
package main

import "fmt"

func checkPossibility(arr []int) bool {
	a := make([]int, len(arr))
	copy(a, arr)
	cnt := 0
	for i := 1; i < len(a); i++ {
		if a[i-1] > a[i] {
			cnt++
			if cnt > 1 {
				return false
			}
			if i < 2 || a[i-2] <= a[i] {
				a[i-1] = a[i]
			} else {
				a[i] = a[i-1]
			}
		}
	}
	return true
}

func main() {
	fmt.Println(checkPossibility([]int{10, 5, 7}))
	fmt.Println(checkPossibility([]int{10, 5, 1}))
}
