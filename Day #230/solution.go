// Egg drop: find min trials t such that maxFloors(t, eggs) >= k, where
// f(t,e) = f(t-1,e-1) + f(t-1,e) + 1 (floors distinguishable). Time: O(eggs * answer), Space: O(eggs).
package main

import "fmt"

func eggDrop(eggs, k int) int {
	f := make([]int64, eggs+1)
	t := 0
	for f[eggs] < int64(k) {
		t++
		for e := eggs; e >= 1; e-- {
			f[e] = f[e] + f[e-1] + 1
		}
	}
	return t
}

func main() {
	fmt.Println(eggDrop(1, 5)) // 5
}
