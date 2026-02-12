// Word sense disambiguation: for each ambiguous word pick the meaning whose words
// overlap most with the sentence context (other words). Tie-break -> first meaning.
// Time: O(W * M * L), Space: O(L) for the context set.
package main

import (
	"fmt"
	"regexp"
	"strings"
)

var re = regexp.MustCompile(`[^a-z0-9]+`)

func tokenize(s string) []string {
	var out []string
	for _, t := range re.Split(strings.ToLower(s), -1) {
		if t != "" {
			out = append(out, t)
		}
	}
	return out
}

func main() {
	dict := map[string][]string{
		"bank": {
			"financial institution where people deposit money",
			"land beside a river or lake",
		},
	}
	sentence := "I went to get money from the bank"
	words := tokenize(sentence)

	for i, w := range words {
		meanings, ok := dict[w]
		if !ok || len(meanings) <= 1 {
			continue
		}
		context := map[string]bool{}
		for j, x := range words {
			if j != i {
				context[x] = true
			}
		}
		best, bestScore := 0, -1
		for m, meaning := range meanings {
			score := 0
			for _, t := range tokenize(meaning) {
				if context[t] {
					score++
				}
			}
			if score > bestScore {
				bestScore, best = score, m
			}
		}
		fmt.Printf("%s: %s\n", w, meanings[best])
	}
}
