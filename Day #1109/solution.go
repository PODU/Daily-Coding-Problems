// Day 1109: Three-way (Dutch national flag) partition around pivot x.
// Bucket into <x, ==x, >x preserving relative order to match the example.
// Time: O(N). Space: O(N).
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
	res := append(less, equal...)
	res = append(res, greater...)
	return res
}

func main() {
	fmt.Println(partitionThree([]int{9, 12, 3, 5, 14, 10, 10}, 10)) // [9 3 5 10 10 12 14]
}
