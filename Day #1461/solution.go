// Egg drop: DP on trials. f(t,e)=f(t-1,e-1)+f(t-1,e)+1 = max floors with t trials, e eggs.
// Smallest t with f(t,N)>=k. Time O(N*answer), Space O(N).
package main

import "fmt"

// eggDrop returns minimum worst-case trials for N eggs and k floors.
func eggDrop(N, k int) int {
	if k == 0 {
		return 0
	}
	f := make([]int64, N+1)
	t := 0
	for f[N] < int64(k) {
		t++
		for e := N; e >= 1; e-- {
			f[e] = f[e] + f[e-1] + 1
		}
	}
	return t
}

func main() {
	fmt.Println(eggDrop(1, 5))
}
