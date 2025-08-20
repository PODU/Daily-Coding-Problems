// Pancake sort: only primitive is reverse(lst,i,j). Each round reverse the window's max into place. O(n^2) time, O(1) space.
package main

import "fmt"

func reverse(a []int, i, j int) {
	for i < j {
		a[i], a[j] = a[j], a[i]
		i++
		j--
	}
}

func pancakeSort(a []int) {
	n := len(a)
	for size := n; size > 1; size-- {
		maxIdx := 0
		for k := 1; k < size; k++ {
			if a[k] > a[maxIdx] {
				maxIdx = k
			}
		}
		if maxIdx != size-1 {
			reverse(a, maxIdx, size-1)
		}
	}
}

func main() {
	a := []int{3, 6, 1, 5, 2, 4}
	pancakeSort(a)
	fmt.Println(a) // [1 2 3 4 5 6]
}
