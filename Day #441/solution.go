// Day 441: Three-way partition around pivot x into <x, ==x, >x buckets.
// O(n) time, O(n) space (stable bucket order matches the example).
package main

import "fmt"

func partitionThree(lst []int, x int) []int {
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
	fmt.Println(partitionThree([]int{9, 12, 3, 5, 14, 10, 10}, 10))
	// [9 3 5 10 10 12 14]
}
