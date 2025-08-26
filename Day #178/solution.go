// Monte Carlo simulation of two dice stopping games; average rolls. O(trials) time, O(1) space.
// Theory: E[rolls "5 then 6"]=36, E[rolls "5 then 5"]=42. Exact sim values depend on RNG/seed.
package main

import (
	"fmt"
	"math/rand"
)

func trial(r *rand.Rand, second int) int {
	rolls, prev := 0, 0
	for {
		c := r.Intn(6) + 1
		rolls++
		if prev == 5 && c == second {
			return rolls
		}
		prev = c
	}
}

func main() {
	r := rand.New(rand.NewSource(12345))
	const T = 100000
	s1, s2 := 0, 0
	for i := 0; i < T; i++ {
		s1 += trial(r, 6)
	}
	for i := 0; i < T; i++ {
		s2 += trial(r, 5)
	}
	e1 := float64(s1) / float64(T)
	e2 := float64(s2) / float64(T)
	fmt.Printf("Game 1 (five then six) expected rolls: %.2f\n", e1)
	fmt.Printf("Game 2 (five then five) expected rolls: %.2f\n", e2)
	if e1 < e2 {
		fmt.Println("Alice should play Game 1 (five then six), it has lower expected cost.")
	} else {
		fmt.Println("Alice should play Game 2 (five then five), it has lower expected cost.")
	}
}
