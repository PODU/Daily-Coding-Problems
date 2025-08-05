// rand5 from rand7 via rejection sampling: call rand7() until <=5. Uniform 1..5. Time O(1) expected, Space O(1).
package main

import (
	"fmt"
	"math"
	"math/rand"
)

func rand7() int {
	return rand.Intn(7) + 1
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
	trials := 100000
	counts := make([]int, 6)
	for i := 0; i < trials; i++ {
		v := rand5()
		if v < 1 || v > 5 {
			panic("out of range")
		}
		counts[v]++
	}
	expected := float64(trials) / 5.0
	for v := 1; v <= 5; v++ {
		if math.Abs(float64(counts[v])-expected) >= expected*0.1 {
			panic("not uniform")
		}
	}
	fmt.Println("rand5 OK")
}
