// Day 948: Tower of Hanoi - print all moves to move n disks from rod 1 to rod 3.
// Classic recursion. Time O(2^n) moves, Space O(n) recursion depth.
package main

import "fmt"

func hanoi(n, from, to, aux int) {
	if n == 0 {
		return
	}
	hanoi(n-1, from, aux, to)
	fmt.Printf("Move %d to %d\n", from, to)
	hanoi(n-1, aux, to, from)
}

func main() {
	hanoi(3, 1, 3, 2)
}
