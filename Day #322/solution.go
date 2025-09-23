// Min jumps on number line: smallest k with S=k(k+1)/2 >= |N| and (S-|N|) even.
// Time: O(sqrt N), Space: O(1).
package main

import "fmt"

func minJumps(N int64) int {
	target := N
	if target < 0 {
		target = -target
	}
	var k, S int64 = 0, 0
	for S < target || (S-target)%2 != 0 {
		k++
		S += k
	}
	return int(k)
}

func main() {
	var N int64 = 10
	fmt.Printf("Minimum jumps to reach %d: %d\n", N, minJumps(N))
}
