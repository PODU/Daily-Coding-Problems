// Partition around pivot into <x, ==x, >x. Stable bucket collect to match expected order. O(n) time, O(n) space.
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
	res := append(less, equal...)
	res = append(res, greater...)
	return res
}

func main() {
	lst := []int{9, 12, 3, 5, 14, 10, 10}
	fmt.Println(partition(lst, 10)) // [9 3 5 10 10 12 14]
}
