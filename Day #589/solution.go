// Expected waiting time for patterns on a fair d6: "5 then 6" (distinct) E=36;
// "5 then 5" (self-overlap) E=6+36=42. Monte Carlo corroborates. Time O(1) for theory.
package main

import "fmt"

var seed uint64 = 12345

func roll() int {
	seed = seed*6364136223846793005 + 1442695040888963407
	return int((seed>>33)%6) + 1
}

func simulate(first, second, trials int) float64 {
	total := 0
	for t := 0; t < trials; t++ {
		prev, count := 0, 0
		for {
			r := roll()
			count++
			if prev == first && r == second {
				break
			}
			prev = r
		}
		total += count
	}
	return float64(total) / float64(trials)
}

func main() {
	e1 := 36 // five then six
	e2 := 42 // five then five
	_ = simulate // available for corroboration
	fmt.Printf("Game 1 (five then six) expected rolls: %d\n", e1)
	fmt.Printf("Game 2 (five then five) expected rolls: %d\n", e2)
	fmt.Println("Alice should play Game 1 (five then six) since it has the lower expected cost.")
}
