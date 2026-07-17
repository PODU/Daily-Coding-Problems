// Egg drop: min trials t s.t. floors(t,eggs)=sum_{i=1..eggs} C(t,i) >= k.
// O(eggs * answer). For N=1,k=5 -> 5.
package main

import "fmt"

func floorsCovered(t, eggs, cap int64) int64 {
	var total, c int64 = 0, 1
	for i := int64(1); i <= eggs; i++ {
		c = c * (t - i + 1) / i // C(t,i) incrementally
		total += c
		if total >= cap {
			return cap
		}
	}
	return total
}

func minDrops(eggs, k int64) int64 {
	var t int64 = 0
	for floorsCovered(t, eggs, k) < k {
		t++
	}
	return t
}

func main() {
	fmt.Println(minDrops(1, 5)) // 5
}
