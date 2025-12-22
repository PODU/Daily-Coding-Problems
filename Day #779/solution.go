// Day 779: Egg drop - minimum worst-case trials for N eggs, k floors.
// Find smallest m with sum_{i=1..N} C(m,i) >= k. O(N * answer) time, O(1) space.
package main

import "fmt"

func eggDrop(eggs, floors int) int {
	if floors == 0 {
		return 0
	}
	m := 0
	for {
		m++
		var reach, c int64 = 0, 1
		for i := 1; i <= eggs; i++ {
			c = c * int64(m-i+1) / int64(i)
			reach += c
			if reach >= int64(floors) {
				break
			}
		}
		if reach >= int64(floors) {
			return m
		}
	}
}

func main() {
	fmt.Println(eggDrop(1, 5))   // 5
	fmt.Println(eggDrop(2, 100)) // 14
}
