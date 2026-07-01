// Left fold (reduce): single linear pass applying combiner to accumulator. O(n) time, O(1) extra space.
package main

import "fmt"

func reduce(arr []int, combiningFn func(int, int) int, initialValue int) int {
	acc := initialValue
	for _, x := range arr {
		acc = combiningFn(acc, x)
	}
	return acc
}

func add(a, b int) int      { return a + b }
func multiply(a, b int) int { return a * b }
func sum(arr []int) int     { return reduce(arr, add, 0) }

func main() {
	fmt.Println(sum([]int{1, 2, 3, 4, 5}))
	fmt.Println(reduce([]int{1, 2, 3, 4}, multiply, 1))
}
