// Apply permutation P: result[P[i]] = array[i]. O(n) time, O(n) space.
// (In-place alternative: follow cycles swapping elements.)
package main

import (
	"fmt"
	"strings"
)

func applyPermutation(array []string, P []int) []string {
	result := make([]string, len(array))
	for i, val := range array {
		result[P[i]] = val
	}
	return result
}

func main() {
	array := []string{"a", "b", "c"}
	P := []int{2, 1, 0}
	res := applyPermutation(array, P)
	quoted := make([]string, len(res))
	for i, x := range res {
		quoted[i] = "\"" + x + "\""
	}
	fmt.Println("[" + strings.Join(quoted, ", ") + "]")
}
