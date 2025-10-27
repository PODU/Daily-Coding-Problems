// Fisher-Yates shuffle using a rand(k) helper returning [1,k]; O(N) time, O(1) extra space.
// Each of the N! permutations is equally likely. Fixed seed -> reproducible output.
package main

import (
	"fmt"
	"math/rand"
	"strconv"
	"strings"
)

var rng = rand.New(rand.NewSource(12345))

// Uniform integer in [1, k] using the provided RNG.
func randk(k int) int {
	return rng.Intn(k) + 1
}

func shuffleDeck(arr []int) {
	n := len(arr)
	for i := n - 1; i >= 1; i-- {
		j := randk(i+1) - 1 // index in [0, i]
		arr[i], arr[j] = arr[j], arr[i]
	}
}

func main() {
	deck := make([]int, 52)
	for i := 0; i < 52; i++ {
		deck[i] = i + 1 // cards 1..52
	}
	shuffleDeck(deck)
	parts := make([]string, len(deck))
	for i, v := range deck {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println(strings.Join(parts, " "))
}
