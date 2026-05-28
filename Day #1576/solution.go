// Day 1576: Blackjack solver with perfect deck knowledge.
// Player enumerates how many cards to hit (stops if bust); dealer plays forced rules.
// Time: O(N^2) total; Space: O(1).
package main

import "fmt"

func dealerPlay(deck []int, idx, dealerTotal int) int {
	for dealerTotal <= 16 && idx < len(deck) {
		dealerTotal += deck[idx]
		idx++
	}
	return dealerTotal
}

func compareScore(player, dealer int) int {
	if player > 21 {
		return -1
	}
	if dealer > 21 {
		return 1
	}
	if player > dealer {
		return 1
	}
	if player < dealer {
		return -1
	}
	return 0
}

func bestScore(deck []int) int {
	playerTotal := deck[0] + deck[1]
	best := -1 << 30
	k := 0
	for {
		if playerTotal <= 21 {
			dealer := dealerPlay(deck, 4+k, deck[2]+deck[3])
			if s := compareScore(playerTotal, dealer); s > best {
				best = s
			}
		} else {
			break
		}
		if 4+k >= len(deck) {
			break
		}
		playerTotal += deck[4+k]
		k++
	}
	return best
}

func main() {
	deck := []int{10, 6, 9, 7, 5, 10, 2}
	fmt.Println("Optimal player score:", bestScore(deck))
}
