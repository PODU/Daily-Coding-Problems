// Day 1447: Does a secret code (6 distinct digits) exist consistent with all
// (guess, score) pairs? Brute force all 6-permutations of 0-9. Time O(P*G*6).
package main

import "fmt"

type guess struct {
	digits [6]int
	score  int
}

func scoreMatch(code [6]int, g [6]int) int {
	s := 0
	for i := 0; i < 6; i++ {
		if code[i] == g[i] {
			s++
		}
	}
	return s
}

func consistent(guesses []guess) bool {
	var code [6]int
	var used [10]bool
	var dfs func(pos int) bool
	dfs = func(pos int) bool {
		if pos == 6 {
			for _, g := range guesses {
				if scoreMatch(code, g.digits) != g.score {
					return false
				}
			}
			return true
		}
		for d := 0; d < 10; d++ {
			if used[d] {
				continue
			}
			if pos == 0 && d == 0 {
				continue // no leading zero
			}
			used[d] = true
			code[pos] = d
			if dfs(pos + 1) {
				used[d] = false
				return true
			}
			used[d] = false
		}
		return false
	}
	return dfs(0)
}

func toGuess(n, score int) guess {
	var g guess
	g.score = score
	for i := 5; i >= 0; i-- {
		g.digits[i] = n % 10
		n /= 10
	}
	return g
}

func main() {
	e1 := []guess{toGuess(175286, 2), toGuess(293416, 3), toGuess(654321, 0)}
	e2 := []guess{toGuess(123456, 4), toGuess(345678, 4), toGuess(567890, 4)}
	pr := func(b bool) {
		if b {
			fmt.Println("True")
		} else {
			fmt.Println("False")
		}
	}
	pr(consistent(e1)) // True
	pr(consistent(e2)) // False
}
