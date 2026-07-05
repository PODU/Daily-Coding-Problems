// Day 1772: Min jumps to reach N. Grow k until triangular sum >= |N| and (sum-|N|) even.
// Flipping any jump j changes parity of (sum-N) by 2j, so even surplus is reachable. O(sqrt(N)).
package main

import "fmt"

func minJumps(n int64) int {
	s := n
	if s < 0 {
		s = -s
	}
	var sum int64 = 0
	k := 0
	for sum < s || (sum-s)%2 != 0 {
		k++
		sum += int64(k)
	}
	return k
}

func main() {
	fmt.Println(minJumps(10))
}
