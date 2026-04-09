// Day 1326: Implement reduce/fold — fold an array left to right with a combining function and an initial value.
// O(n) calls to the combiner, O(1) extra space.
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
	total := reduce(lst, func(a, b int) int { return a + b }, 0)
	fmt.Println(total) // 15
}
