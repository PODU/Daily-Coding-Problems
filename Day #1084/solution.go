// reduce/fold: apply combiner left-to-right starting from init. Time O(n), Space O(1).
package main

import "fmt"

func reduce(lst []int, combine func(int, int) int, init int) int {
	acc := init
	for _, x := range lst {
		acc = combine(acc, x)
	}
	return acc
}

func main() {
	lst := []int{1, 2, 3, 4, 5}
	add := func(a, b int) int { return a + b }
	fmt.Println(reduce(lst, add, 0)) // 15
}
