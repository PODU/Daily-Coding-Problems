// Day 837: Sentence checker over a character stream.
// Accumulate runes until a terminal mark, then validate the buffered sentence by regex; print if valid.
// O(N) over the stream; O(L) per sentence buffer.
package main

import (
	"fmt"
	"regexp"
	"strings"
)

var terminals = map[rune]bool{'.': true, '?': true, '!': true, '‽': true}
var pattern = regexp.MustCompile(`^[A-Z][a-z]*[,;:]?( [a-z]+[,;:]?)*[.?!‽]$`)

func checkStream(stream string) []string {
	var results []string
	var buf strings.Builder
	for _, ch := range stream {
		buf.WriteRune(ch)
		if terminals[ch] {
			sentence := strings.TrimLeft(buf.String(), " ")
			if pattern.MatchString(sentence) {
				results = append(results, sentence)
			}
			buf.Reset()
		}
	}
	return results
}

func main() {
	stream := "Hello, world. this is wrong. The cat sat."
	for _, s := range checkStream(stream) {
		fmt.Println(s)
	}
}
