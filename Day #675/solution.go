// Day 675: Sentence equivalence under (non-transitive) synonym pairs. For each position,
// words must be equal or a known synonym pair. Time O(W) with a hashed pair set.
package main

import (
	"fmt"
	"regexp"
)

var re = regexp.MustCompile(`[a-z0-9]+`)

func tokens(s string) []string {
	return re.FindAllString(toLower(s), -1)
}

func toLower(s string) string {
	b := []byte(s)
	for i, c := range b {
		if c >= 'A' && c <= 'Z' {
			b[i] = c + 32
		}
	}
	return string(b)
}

func equivalent(synonyms [][2]string, s1, s2 string) bool {
	pairs := map[[2]string]bool{}
	for _, p := range synonyms {
		pairs[[2]string{p[0], p[1]}] = true
		pairs[[2]string{p[1], p[0]}] = true
	}
	w1, w2 := tokens(s1), tokens(s2)
	if len(w1) != len(w2) {
		return false
	}
	for i := range w1 {
		if w1[i] != w2[i] && !pairs[[2]string{w1[i], w2[i]}] {
			return false
		}
	}
	return true
}

func main() {
	syn := [][2]string{{"big", "large"}, {"eat", "consume"}}
	if equivalent(syn, "He wants to eat food.", "He wants to consume food.") {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
