// rand7 from rand5: rejection sampling over 5*(rand5-1)+rand5 in 1..25,
// reject >21, map ((v-1)%7)+1. Expected O(1) amortized. rand5 from a seeded LCG.
package main

import (
	"fmt"
	"strings"
)

var state int64 = 1 // deterministic seed

func rand5() int {
	state = (state*75 + 74) % 65537
	return int(state%5) + 1 // uniform-ish 1..5 for the demo
}

func rand7() int {
	for {
		v := 5*(rand5()-1) + rand5() // 1..25
		if v <= 21 {
			return (v-1)%7 + 1
		}
	}
}

func main() {
	samples := make([]string, 20)
	for i := 0; i < 20; i++ {
		samples[i] = fmt.Sprintf("%d", rand7())
	}
	fmt.Println("rand7 samples: " + strings.Join(samples, " "))

	counts := make([]int, 8)
	for i := 0; i < 7000; i++ {
		counts[rand7()]++
	}
	parts := make([]string, 0, 7)
	for v := 1; v <= 7; v++ {
		parts = append(parts, fmt.Sprintf("%d:%d", v, counts[v]))
	}
	fmt.Println("counts over 7000 trials: " + strings.Join(parts, " "))
}
