// Day 757: Optimal blackjack with a fully known deck.
// Player draws the next cards off the top; try every stand point, dealer then
// follows the fixed rule. Pick the player's best net score. Time: O(n^2), Space: O(1).
package main

import "fmt"

// deck values: 2..10, face=10, ace=1. [0,1]=player, [2,3]=dealer, [4..]=draw pile.
func bestScore(deck []int) int {
	n := len(deck)
	player := deck[0] + deck[1]
	dealerStart := deck[2] + deck[3]
	best := -1 << 30
	ptot, idx := player, 4
	for {
		if ptot > 21 {
			break
		}
		dtot, di := dealerStart, idx
		for dtot <= 16 && di < n {
			dtot += deck[di]
			di++
		}
		var result int
		switch {
		case dtot > 21:
			result = 1
		case ptot > dtot:
			result = 1
		case ptot < dtot:
			result = -1
		default:
			result = 0
		}
		if result > best {
			best = result
		}
		if idx >= n {
			break
		}
		ptot += deck[idx]
		idx++
	}
	return best
}

func main() {
	deck := []int{10, 9, 2, 3, 10}
	fmt.Println(bestScore(deck)) // 1
}
