// Blackjack with known deck. DFS over deck index: player hits/stands to maximize
// net (+1/0/-1); dealer then plays fixed rule (hit while <=16). O(deck) states.
package main

import "fmt"

var deck = []int{10, 7, 5, 9, 6, 4, 10, 2}

func dealerPlay(p, playerTotal int) int {
	dealerTotal := deck[1] + deck[3]
	for dealerTotal <= 16 && p < len(deck) {
		dealerTotal += deck[p]
		p++
	}
	if dealerTotal > 21 {
		return 1
	}
	if playerTotal > dealerTotal {
		return 1
	}
	if playerTotal < dealerTotal {
		return -1
	}
	return 0
}

func playerPlay(p, playerTotal int) int {
	best := dealerPlay(p, playerTotal) // stand
	if p < len(deck) {
		nt := playerTotal + deck[p]
		hit := -1
		if nt <= 21 {
			hit = playerPlay(p+1, nt)
		}
		if hit > best {
			best = hit
		}
	}
	return best
}

func main() {
	playerTotal := deck[0] + deck[2]
	fmt.Println(playerPlay(4, playerTotal))
}
