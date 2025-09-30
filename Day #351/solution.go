// Simplified Lesk: score each gloss by word overlap with sentence context; pick max (ties->first).
// Time O(words * meanings * glossLen), Space O(vocab).
package main

import (
	"fmt"
	"strings"
)

func tokSet(s string) map[string]bool {
	m := map[string]bool{}
	for _, w := range strings.Fields(strings.ToLower(s)) {
		m[w] = true
	}
	return m
}

func main() {
	meanings := map[string][]string{
		"bank": {"place where people deposit and withdraw money",
			"sloping land beside a river or lake of water"},
		"money": {"currency cash that people deposit"},
		"river": {"large natural stream of water"},
	}
	sentence := "I went to get money from the bank"
	tokens := strings.Fields(strings.ToLower(sentence))

	for i, w := range tokens {
		senses, ok := meanings[w]
		if !ok || len(senses) < 2 {
			continue // not ambiguous
		}
		context := map[string]bool{}
		for j, t := range tokens {
			if j != i {
				context[t] = true
			}
		}
		bestIdx, bestScore := 0, -1
		for idx, gloss := range senses {
			score := 0
			for g := range tokSet(gloss) {
				if context[g] {
					score++
				}
			}
			if score > bestScore {
				bestScore, bestIdx = score, idx
			}
		}
		fmt.Printf("%s: %s\n", w, senses[bestIdx])
	}
}
