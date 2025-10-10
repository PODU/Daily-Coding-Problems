// rand7 from rand5 via rejection sampling: idx=(rand5-1)*5+rand5 in 1..25, reject>21,
// return (idx-1)%7+1. O(1) expected calls. Space O(1).
package main

import (
	"fmt"
	"math/rand"
	"strings"
)

var rng = rand.New(rand.NewSource(12345))

func rand5() int { return rng.Intn(5) + 1 }

func rand7() int {
	for {
		idx := (rand5()-1)*5 + rand5() // uniform 1..25
		if idx <= 21 {
			return (idx-1)%7 + 1
		}
	}
}

func main() {
	const N = 70000
	counts := make([]int, 8)
	for i := 0; i < N; i++ {
		counts[rand7()]++
	}
	for v := 1; v <= 7; v++ {
		fmt.Printf("%d: %d %s\n", v, counts[v], strings.Repeat("#", counts[v]/250))
	}
}
