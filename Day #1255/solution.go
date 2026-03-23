// Two-sum existence: one-pass hash set, check (k-num) seen before insert.
// Time O(n), Space O(n).
package main

import "fmt"

func hasPair(a []int, k int) bool {
	seen := make(map[int]bool)
	for _, x := range a {
		if seen[k-x] {
			return true
		}
		seen[x] = true
	}
	return false
}

func main() {
	v := []int{10, 15, 3, 7}
	fmt.Println(hasPair(v, 17))
	fmt.Println(hasPair(v, 50))
}
