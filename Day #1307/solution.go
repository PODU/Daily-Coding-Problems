// Fisher-Yates shuffle using a uniform rand(1..k). Each of N! permutations
// equally likely. Time O(N), Space O(1) extra.
package main

import (
	"fmt"
	"math/rand"
	"strings"
)

var rng = rand.New(rand.NewSource(12345))

// randK returns a uniform random integer in [1, k].
func randK(k int) int { return rng.Intn(k) + 1 }

func shuffleDeck(deck []int) {
	for i := len(deck) - 1; i > 0; i-- {
		j := randK(i+1) - 1 // uniform in [0, i]
		deck[i], deck[j] = deck[j], deck[i]
	}
}

func main() {
	deck := make([]int, 52)
	for i := range deck {
		deck[i] = i + 1
	}
	shuffleDeck(deck)
	parts := make([]string, len(deck))
	for i, v := range deck {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println(strings.Join(parts, " "))
}
