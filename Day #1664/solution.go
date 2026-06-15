// Apply permutation: result[P[i]] = array[i]. O(n) time, O(n) space.
package main

import (
	"fmt"
	"strings"
)

func main() {
	arr := []string{"a", "b", "c"}
	P := []int{2, 1, 0}
	res := make([]string, len(arr))
	for i := range arr {
		res[P[i]] = arr[i]
	}
	fmt.Println("[" + strings.Join(res, ", ") + "]")
}
