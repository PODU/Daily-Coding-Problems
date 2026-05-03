// Sentence similarity (non-transitive). Store synonym pairs both directions in a set; compare word by word.
// Time O(words + pairs), Space O(pairs).
package main

import (
	"fmt"
	"strings"
)

type pair struct{ a, b string }

func areSimilar(s1, s2 []string, pairs []pair) bool {
	if len(s1) != len(s2) {
		return false
	}
	syn := make(map[pair]bool)
	for _, p := range pairs {
		syn[pair{p.a, p.b}] = true
		syn[pair{p.b, p.a}] = true
	}
	for i := range s1 {
		if s1[i] == s2[i] {
			continue
		}
		if syn[pair{s1[i], s2[i]}] {
			continue
		}
		return false
	}
	return true
}

func tokenize(s string) []string {
	return strings.Fields(strings.ReplaceAll(s, ".", ""))
}

func main() {
	synonyms := []pair{{"big", "large"}, {"eat", "consume"}}
	a := tokenize("He wants to eat food.")
	b := tokenize("He wants to consume food.")
	if areSimilar(a, b, synonyms) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
