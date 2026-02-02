// Day 1005: Blackjack solver with perfect knowledge of the deck order.
// Player gets deck[0],deck[2]; dealer gets deck[1],deck[3]; rest is the draw pile.
// Try every number of player hits k, keep best outcome; dealer hits while <= 16.
// O(N^2) over the deck.
package main

import "fmt"

func bestScore(deck []int) int {
	n := len(deck)
	playerBase := deck[0] + deck[2]
	dealerBase := deck[1] + deck[3]
	best := -1
	for k := 0; ; k++ {
		player := playerBase
		for i := 0; i < k; i++ {
			player += deck[4+i]
		}
		if player > 21 {
			break
		}
		idx, dealer := 4+k, dealerBase
		for dealer <= 16 && idx < n {
			dealer += deck[idx]
			idx++
		}
		outcome := 0
		if dealer > 21 || player > dealer {
			outcome = 1
		} else if player < dealer {
			outcome = -1
		}
		if outcome > best {
			best = outcome
		}
		if 4+k >= n {
			break
		}
	}
	return best
}

func main() {
	deck := []int{10, 10, 6, 9, 5, 7, 3, 8}
	fmt.Println("Best player score:", bestScore(deck)) // 1
}
