// Custom reduce/fold (left to right). O(n) time, O(1) extra space.
package main

import "fmt"

func reduce(arr []int, fn func(int, int) int, init int) int {
	acc := init
	for _, x := range arr {
		acc = fn(acc, x)
	}
	return acc
}

func main() {
	arr := []int{1, 2, 3, 4}
	add := func(a, b int) int { return a + b }
	mul := func(a, b int) int { return a * b }
	fmt.Println(reduce(arr, add, 0)) // 10
	fmt.Println(reduce(arr, mul, 1)) // 24 (bonus)
}
