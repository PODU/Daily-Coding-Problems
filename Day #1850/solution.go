// Day 1850: rand7() from rand5() via rejection sampling on the 1..25 grid.
// Expected O(1) amortized calls (acceptance 21/25); uniform over 1..7.
package main

import (
	"fmt"
	"math/rand"
)

var rng = rand.New(rand.NewSource(12345))

func rand5() int { return rng.Intn(5) + 1 }

func rand7() int {
	for {
		v := 5*(rand5()-1) + rand5() // 1..25
		if v <= 21 {
			return (v-1)%7 + 1
		}
	}
}

func main() {
	counts := make([]int, 8)
	for i := 0; i < 70000; i++ {
		counts[rand7()]++
	}
	fmt.Print("Sample of 10:")
	for i := 0; i < 10; i++ {
		fmt.Printf(" %d", rand7())
	}
	fmt.Println()
	fmt.Println("Histogram over 70000 draws (each ~10000):")
	for i := 1; i <= 7; i++ {
		fmt.Printf("  %d: %d\n", i, counts[i])
	}
}
