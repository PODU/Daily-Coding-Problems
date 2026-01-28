// Cryptarithmetic 3-word solver (word1 + word2 = word3) via backtracking.
// Time: O(10!/(10-k)!) for k unique letters; Space: O(k).
package main

import "fmt"

var A, B, C string
var letters []byte
var leading = map[byte]bool{}
var assign = map[byte]int{}
var used [10]bool

func val(w string) int64 {
	var v int64
	for i := 0; i < len(w); i++ {
		v = v*10 + int64(assign[w[i]])
	}
	return v
}

func dfs(i int) bool {
	if i == len(letters) {
		return val(A)+val(B) == val(C)
	}
	c := letters[i]
	for d := 0; d < 10; d++ {
		if used[d] {
			continue
		}
		if d == 0 && leading[c] {
			continue
		}
		used[d] = true
		assign[c] = d
		if dfs(i + 1) {
			return true
		}
		used[d] = false
	}
	return false
}

func main() {
	A, B, C = "SEND", "MORE", "MONEY"
	seen := map[byte]bool{}
	for _, w := range []string{A, B, C} {
		for i := 0; i < len(w); i++ {
			if !seen[w[i]] {
				seen[w[i]] = true
				letters = append(letters, w[i])
			}
		}
	}
	leading[A[0]] = true
	leading[B[0]] = true
	leading[C[0]] = true
	dfs(0)
	order := "SENDMORY"
	out := "{"
	for i := 0; i < len(order); i++ {
		out += fmt.Sprintf("'%c': %d", order[i], assign[order[i]])
		if i+1 < len(order) {
			out += ", "
		}
	}
	out += "}"
	fmt.Println(out)
}
