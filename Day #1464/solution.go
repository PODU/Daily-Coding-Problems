// Two single elements (rest in pairs): XOR all -> a^b, split by a set bit, XOR groups.
// Time: O(n), Space: O(1).
package main

import "fmt"

func twoUnique(arr []int) (int, int) {
	x := 0
	for _, v := range arr {
		x ^= v
	}
	bit := x & (-x) // lowest set bit
	a, b := 0, 0
	for _, v := range arr {
		if v&bit != 0 {
			a ^= v
		} else {
			b ^= v
		}
	}
	if a > b {
		a, b = b, a
	}
	return a, b
}

func main() {
	arr := []int{2, 4, 6, 8, 10, 2, 6, 10}
	a, b := twoUnique(arr)
	fmt.Printf("%d and %d\n", a, b)
}
