// Day 803: Does a secret (6 distinct digits) exist matching all guess scores?
// Brute force all 6-digit distinct-digit codes, verify every guess's score.
// Time O(10^6 * G), Space O(G).
package main

import "fmt"

func digits(x int) [6]int {
	var d [6]int
	for i := 5; i >= 0; i-- {
		d[i] = x % 10
		x /= 10
	}
	return d
}

func distinct(d [6]int) bool {
	seen := 0
	for _, v := range d {
		if seen&(1<<v) != 0 {
			return false
		}
		seen |= 1 << v
	}
	return true
}

func score(code, guess [6]int) int {
	s := 0
	for i := 0; i < 6; i++ {
		if code[i] == guess[i] {
			s++
		}
	}
	return s
}

func feasible(guesses map[int]int) bool {
	type gp struct {
		d  [6]int
		sc int
	}
	var gs []gp
	for g, sc := range guesses {
		gs = append(gs, gp{digits(g), sc})
	}
	for code := 0; code <= 999999; code++ {
		d := digits(code)
		if !distinct(d) {
			continue
		}
		ok := true
		for _, g := range gs {
			if score(d, g.d) != g.sc {
				ok = false
				break
			}
		}
		if ok {
			return true
		}
	}
	return false
}

func main() {
	fmt.Println(feasible(map[int]int{175286: 2, 293416: 3, 654321: 0})) // true
	fmt.Println(feasible(map[int]int{123456: 4, 345678: 4, 567890: 4})) // false
}
