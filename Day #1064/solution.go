// Day 1064: Implement rand7() from rand5() via rejection sampling.
// (rand5()-1)*5 + rand5() -> uniform 1..25; reject >21; ((v-1)%7)+1. Expected O(1) calls, O(1) space.
package main

import (
	"fmt"
	"math/rand"
)

var rng = rand.New(rand.NewSource(42))

func rand5() int {
	return rng.Intn(5) + 1
}

func rand7() int {
	for {
		v := (rand5()-1)*5 + rand5() // uniform 1..25
		if v <= 21 {
			return ((v - 1) % 7) + 1
		}
	}
}

func main() {
	var counts [8]int
	for i := 0; i < 70000; i++ {
		counts[rand7()]++
	}
	for i := 1; i <= 7; i++ {
		fmt.Printf("%d: %d\n", i, counts[i])
	}
}
