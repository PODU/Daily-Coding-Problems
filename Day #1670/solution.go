// Prefix-sum hashmap: store sum->index; if sum-K seen, slice between. O(n) time, O(n) space.
package main

import (
	"fmt"
	"strings"
	"strconv"
)

func subarraySum(a []int, K int) []int {
	seen := map[int]int{0: -1}; s := 0
	for i, v := range a {
		s += v
		if j, ok := seen[s-K]; ok { return a[j+1 : i+1] }
		if _, ok := seen[s]; !ok { seen[s] = i }
	}
	return []int{}
}
func main() {
	r := subarraySum([]int{1, 2, 3, 4, 5}, 9)
	s := make([]string, len(r))
	for i, v := range r { s[i] = strconv.Itoa(v) }
	fmt.Println("[" + strings.Join(s, ", ") + "]")
}
