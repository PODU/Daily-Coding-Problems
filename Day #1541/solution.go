// Fisher-Yates shuffle: uniform random permutation using only swaps.
// rand(k) gives a uniform int in [1,k]; each of N! orderings is equally likely.
// Time O(N), Space O(1).
package main

import (
	"fmt"
	"math/rand"
	"strconv"
	"strings"
)

var rng = rand.New(rand.NewSource(12345))

// uniform random number between 1 and k inclusive
func randk(k int) int { return rng.Intn(k) + 1 }

func shuffle(deck []int) {
	for i := len(deck) - 1; i > 0; i-- {
		j := randk(i+1) - 1 // index in [0, i]
		deck[i], deck[j] = deck[j], deck[i]
	}
}

func main() {
	deck := make([]int, 52)
	for i := range deck {
		deck[i] = i + 1
	}
	shuffle(deck)
	parts := make([]string, len(deck))
	for i, v := range deck {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println(strings.Join(parts, " "))
}
