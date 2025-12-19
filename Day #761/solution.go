// Day 761: rand5() from rand7() via rejection sampling.
// Accept values 1..5, reject 6..7 and retry. Uniform; expected O(1) calls (7/5).
package main

import "fmt"

var s uint64 = 1

func rand7() int {
	s = (s*1103515245 + 12345) & 0x7fffffff
	return int(s%7) + 1 // uniform 1..7
}

func rand5() int {
	for {
		x := rand7()
		if x <= 5 {
			return x
		}
	}
}

func main() {
	N := 100000
	cnt := make([]int, 6)
	for i := 0; i < N; i++ {
		cnt[rand5()]++
	}
	fmt.Printf("counts over %d samples:\n", N)
	for v := 1; v <= 5; v++ {
		fmt.Printf("%d: %d\n", v, cnt[v])
	}
}
