// Day 1453: Apply a permutation P (P[i] = destination of element i) to an array
// in place by following cycles. Time O(n), Space O(1) extra (P is consumed).
package main

import (
	"fmt"
	"strings"
)

func applyPermutation(arr []string, P []int) {
	for i := 0; i < len(arr); i++ {
		for P[i] != i {
			pi := P[i]
			arr[i], arr[pi] = arr[pi], arr[i]
			P[i], P[pi] = P[pi], P[i]
		}
	}
}

func main() {
	arr := []string{"a", "b", "c"}
	P := []int{2, 1, 0}
	applyPermutation(arr, P)
	quoted := make([]string, len(arr))
	for i, x := range arr {
		quoted[i] = "\"" + x + "\""
	}
	fmt.Println("[" + strings.Join(quoted, ", ") + "]") // ["c", "b", "a"]
}
