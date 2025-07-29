// Day 51: Uniform shuffle via Fisher-Yates, using only rand(1..k) and swaps.
// Each of n! permutations equally likely. Time: O(n), Space: O(1).
package main

import (
	"fmt"
	"math/rand"
	"sort"
	"strings"
	"time"
)

var rng = rand.New(rand.NewSource(time.Now().UnixNano()))

// randK returns a perfectly random integer in [1, k].
func randK(k int) int { return rng.Intn(k) + 1 }

func shuffle(deck []int) {
	for i := len(deck) - 1; i > 0; i-- {
		j := randK(i+1) - 1 // uniform index in [0, i]
		deck[i], deck[j] = deck[j], deck[i]
	}
}

func main() {
	deck := make([]int, 52)
	for i := range deck {
		deck[i] = i
	}
	shuffle(deck)
	parts := make([]string, len(deck))
	for i, v := range deck {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println(strings.Join(parts, " "))
	seen := append([]int(nil), deck...)
	sort.Ints(seen)
	ok := true
	for i, v := range seen {
		if v != i {
			ok = false
		}
	}
	fmt.Println("valid permutation:", ok)
}
