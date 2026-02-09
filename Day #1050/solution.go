// Apply permutation where result[P[i]] = array[i] (scatter). In-place cycle-following via swaps:
// O(n) time, O(1) extra space. Also a simple O(n)-space scatter is provided.
package main

import (
	"fmt"
	"strings"
)

// applyInPlace follows each cycle with swaps; each swap settles an element, so <= n swaps total.
func applyInPlace(a []string, P []int) {
	p := make([]int, len(P)) // local copy so caller's permutation is untouched
	copy(p, P)
	for i := 0; i < len(a); i++ {
		for p[i] != i {
			j := p[i]
			a[i], a[j] = a[j], a[i]
			p[i], p[j] = p[j], p[i]
		}
	}
}

func applySimple(a []string, P []int) []string {
	res := make([]string, len(a))
	for i := range a {
		res[P[i]] = a[i]
	}
	return res
}

func main() {
	a := []string{"a", "b", "c"}
	P := []int{2, 1, 0} // result[P[i]] = a[i]
	applyInPlace(a, P)
	fmt.Println("[" + strings.Join(a, ", ") + "]")
}
