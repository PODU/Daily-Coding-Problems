// Custom left fold (reduce). acc=init; for each x: acc=f(acc,x). O(n) time, O(1) space.
package main

import "fmt"

func myReduce(arr []int, f func(int, int) int, init int) int {
	acc := init
	for _, x := range arr {
		acc = f(acc, x)
	}
	return acc
}

func add(a, b int) int { return a + b }

func sum(lst []int) int {
	return myReduce(lst, add, 0)
}

func main() {
	fmt.Println(sum([]int{1, 2, 3, 4, 5}))
}
