// Day 1775: Mastermind consistency. Brute-force every 6-digit code with distinct
// digits (P(10,6)=151200); a code is valid if it reproduces every guess's score.
// O(P(10,6) * G) time, O(1) extra space.
package main

import "fmt"

type guess struct {
	d     [6]int
	score int
}

func mk(num, score int) guess {
	var g guess
	g.score = score
	for i := 5; i >= 0; i-- {
		g.d[i] = num % 10
		num /= 10
	}
	return g
}

func consistent(gs []guess) bool {
	var code [6]int
	var used [10]bool

	var check func() bool
	check = func() bool {
		for _, g := range gs {
			m := 0
			for i := 0; i < 6; i++ {
				if code[i] == g.d[i] {
					m++
				}
			}
			if m != g.score {
				return false
			}
		}
		return true
	}

	var rec func(pos int) bool
	rec = func(pos int) bool {
		if pos == 6 {
			return check()
		}
		for d := 0; d < 10; d++ {
			if used[d] {
				continue
			}
			used[d] = true
			code[pos] = d
			if rec(pos + 1) {
				used[d] = false
				return true
			}
			used[d] = false
		}
		return false
	}
	return rec(0)
}

func main() {
	a := []guess{mk(175286, 2), mk(293416, 3), mk(654321, 0)}
	b := []guess{mk(123456, 4), mk(345678, 4), mk(567890, 4)}
	fmt.Println(map[bool]string{true: "True", false: "False"}[consistent(a)])
	fmt.Println(map[bool]string{true: "True", false: "False"}[consistent(b)])
}
