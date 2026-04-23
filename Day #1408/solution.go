// Pancake sort: for each shrinking prefix find its max, flip it to the front,
// then flip it into its final position. Only uses reverse(lst, i, j).
// Time O(n^2) comparisons, O(n) flips, Space O(1).
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
	for n := len(a); n > 1; n-- {
		mi := 0
		for i := 1; i < n; i++ {
			if a[i] > a[mi] {
				mi = i
			}
		}
		if mi != n-1 {
			reverse(a, 0, mi)
			reverse(a, 0, n-1)
		}
	}
}

func main() {
	a := []int{3, 1, 4, 1, 5, 9, 2, 6}
	pancakeSort(a)
	fmt.Println(a)
}
