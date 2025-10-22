// Find duplicate in array of n+1 ints from 1..n using a visited map.
// Time O(n), Space O(n). (Floyd's cycle detection is an O(1)-space alternative.)
package main

import "fmt"

func findDuplicate(a []int) int {
	seen := make(map[int]bool)
	for _, x := range a {
		if seen[x] {
			return x
		}
		seen[x] = true
	}
	return -1
}

func main() {
	fmt.Println(findDuplicate([]int{1, 3, 4, 2, 2}))
}
