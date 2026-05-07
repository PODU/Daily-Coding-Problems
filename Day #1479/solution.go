// Day 1479: Partition a list into <x, ==x, >x around pivot x.
// Stable bucketing into three lists preserves relative order. Time O(N), Space O(N).
package main

import "fmt"

func partition(lst []int, x int) []int {
	var less, equal, greater []int
	for _, v := range lst {
		switch {
		case v < x:
			less = append(less, v)
		case v == x:
			equal = append(equal, v)
		default:
			greater = append(greater, v)
		}
	}
	res := append([]int{}, less...)
	res = append(res, equal...)
	res = append(res, greater...)
	return res
}

func main() {
	fmt.Println(partition([]int{9, 12, 3, 5, 14, 10, 10}, 10))
	// [9 3 5 10 10 12 14]
}
