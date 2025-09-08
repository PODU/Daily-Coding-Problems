// Blackjack with known deck: deal alternately (player,dealer,player,dealer), then the player
// may take k>=0 hits. Try every k (until bust), simulate the forced dealer, pick the best net
// score (+1 win / 0 push / -1 loss). Time: O(deck), Space: O(1).
package main

import "fmt"

func bestScore(deck []int) int {
	player2 := deck[0] + deck[2]
	dealer2 := deck[1] + deck[3]
	best := -1 << 30
	psum, idx := player2, 4
	for {
		var outcome int
		if psum > 21 {
			outcome = -1
		} else {
			dsum, di := dealer2, idx
			for dsum <= 16 && di < len(deck) {
				dsum += deck[di]
				di++
			}
			if dsum > 21 || psum > dsum {
				outcome = 1
			} else if psum < dsum {
				outcome = -1
			} else {
				outcome = 0
			}
		}
		if outcome > best {
			best = outcome
		}
		if psum > 21 || idx >= len(deck) {
			break
		}
		psum += deck[idx]
		idx++
	}
	return best
}

func main() {
	deck := []int{5, 10, 6, 9, 10, 2, 3, 7, 8, 4}
	fmt.Println("Best score:", bestScore(deck)) // 1
}
