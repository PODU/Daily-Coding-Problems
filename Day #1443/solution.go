// Day 1443: Word sense disambiguation (simplified Lesk algorithm).
// For each ambiguous word pick the meaning whose words overlap most with the
// rest of the sentence's words. Time O(W * M * L), Space O(vocab).
package main

import (
	"fmt"
	"regexp"
	"strings"
)

var nonAlnum = regexp.MustCompile(`[^a-z0-9]+`)

func tokenize(s string) []string {
	var out []string
	for _, t := range nonAlnum.Split(strings.ToLower(s), -1) {
		if t != "" {
			out = append(out, t)
		}
	}
	return out
}

func disambiguate(meanings map[string][]string, sentence string) map[string]string {
	words := tokenize(sentence)
	context := map[string]bool{}
	for _, w := range words {
		context[w] = true
	}
	result := map[string]string{}
	for _, w := range words {
		senses, ok := meanings[w]
		if !ok || len(senses) <= 1 {
			continue
		}
		best, bestMeaning := -1, ""
		for _, m := range senses {
			seen := map[string]bool{}
			score := 0
			for _, t := range tokenize(m) {
				if seen[t] {
					continue
				}
				seen[t] = true
				if t != w && context[t] {
					score++
				}
			}
			if score > best {
				best, bestMeaning = score, m
			}
		}
		result[w] = bestMeaning
	}
	return result
}

func main() {
	meanings := map[string][]string{
		"bank": {
			"financial institution where people deposit money",
			"sloping land beside a river or lake",
		},
	}
	sentence := "I went to the bank to deposit money"
	res := disambiguate(meanings, sentence)
	fmt.Println("bank:", res["bank"])
}
