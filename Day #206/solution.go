// Day 206: Apply permutation P to array (result[P[i]] = a[i]).
// In-place via cycle following on the permutation. Time: O(n), Space: O(1).
package main

import "fmt"

func applyPermutation(a []string, perm []int) []string {
	p := append([]int(nil), perm...)
	for i := range a {
		for p[i] != i {
			j := p[i]
			a[i], a[j] = a[j], a[i]
			p[i], p[j] = p[j], j
		}
	}
	return a
}

func main() {
	fmt.Println(applyPermutation([]string{"a", "b", "c"}, []int{2, 1, 0})) // [c b a]
}
