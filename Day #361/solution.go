// Day 361: Mastermind feasibility.
// Brute-force all 6-position codes with distinct digits; accept if some code
// matches every guess's score. Time O(P*G*6), P=151200, Space O(1).
package main

import "fmt"

func scoreOf(code [6]byte, guess string) int {
	s := 0
	for i := 0; i < 6; i++ {
		if code[i] == guess[i] {
			s++
		}
	}
	return s
}

func feasible(codes []string, scores []int) bool {
	var code [6]byte
	var rec func(pos, used int) bool
	rec = func(pos, used int) bool {
		if pos == 6 {
			for i, c := range codes {
				if scoreOf(code, c) != scores[i] {
					return false
				}
			}
			return true
		}
		for d := 0; d < 10; d++ {
			if used&(1<<uint(d)) == 0 {
				code[pos] = byte('0' + d)
				if rec(pos+1, used|(1<<uint(d))) {
					return true
				}
			}
		}
		return false
	}
	return rec(0, 0)
}

func main() {
	fmt.Println(map[bool]string{true: "True", false: "False"}[
		feasible([]string{"175286", "293416", "654321"}, []int{2, 3, 0})])
	fmt.Println(map[bool]string{true: "True", false: "False"}[
		feasible([]string{"123456", "345678", "567890"}, []int{4, 4, 4})])
}
