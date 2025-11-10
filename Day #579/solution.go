// Min jumps: smallest k with S=k(k+1)/2 >= |N| and (S-|N|) even. Time O(sqrt(N)), Space O(1).
package main

import "fmt"

func minJumps(N int) int {
	n := N
	if n < 0 {
		n = -n
	}
	k, s := 0, 0
	for s < n || (s-n)%2 != 0 {
		k++
		s += k
	}
	return k
}

func main() {
	fmt.Printf("Minimum jumps to reach 10: %d\n", minJumps(10))
}
