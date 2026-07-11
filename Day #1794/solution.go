// Misere Nim: P-position (loss for mover) iff (all heaps==1 with even count) or (some heap>1 and xor==0). First wins otherwise.
// Time O(n), Space O(1).
package main

import "fmt"

func firstPlayerWins(heaps []int) bool {
	x, mx := 0, 0
	for _, h := range heaps {
		x ^= h
		if h > mx {
			mx = h
		}
	}
	var pPosition bool
	if mx <= 1 {
		pPosition = x == 0 // all heaps == 1: P iff even count
	} else {
		pPosition = x == 0 // some heap > 1: P iff xor == 0
	}
	return !pPosition
}

func main() {
	heaps := []int{3, 4, 5}
	fmt.Println(firstPlayerWins(heaps)) // expected true
}
