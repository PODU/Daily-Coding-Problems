// Mastermind: brute force all 6-permutations of digits 0-9 (10P6=151200),
// keep one consistent with every guess score. Time: O(10P6 * G), Space: O(1).
package main

import "fmt"

type guess struct {
	code  string
	score int
}

func score(secret []int, g string) int {
	s := 0
	for i := 0; i < 6; i++ {
		if secret[i] == int(g[i]-'0') {
			s++
		}
	}
	return s
}

func search(secret []int, pos int, used []bool, guesses []guess) bool {
	if pos == 6 {
		for _, g := range guesses {
			if score(secret, g.code) != g.score {
				return false
			}
		}
		return true
	}
	for d := 0; d < 10; d++ {
		if used[d] {
			continue
		}
		used[d] = true
		secret[pos] = d
		if search(secret, pos+1, used, guesses) {
			return true
		}
		used[d] = false
	}
	return false
}

func consistent(guesses []guess) bool {
	secret := make([]int, 6)
	used := make([]bool, 10)
	return search(secret, 0, used, guesses)
}

func main() {
	ex1 := []guess{{"175286", 2}, {"293416", 3}, {"654321", 0}}
	ex2 := []guess{{"123456", 4}, {"345678", 4}, {"567890", 4}}
	if consistent(ex1) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
	if consistent(ex2) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
