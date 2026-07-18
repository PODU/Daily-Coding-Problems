// Smallest positive int not a subset sum of a sorted array. Greedy O(N).
// Keep reachable range [1..res-1]; if a[i] <= res extend, else res is the answer.
package main

import "fmt"

func smallestMissing(a []int64) int64 {
	var res int64 = 1
	for _, x := range a {
		if x > res {
			break
		}
		res += x
	}
	return res
}

func main() {
	fmt.Println(smallestMissing([]int64{1, 2, 3, 10})) // 7
}
