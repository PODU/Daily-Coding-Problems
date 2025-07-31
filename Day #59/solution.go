// Day 59: File sync over low-bandwidth link, rsync-style.
// Receiver sends per-block (weak rolling + strong) checksums of its OLD file;
// sender scans NEW file with a rolling checksum, emitting COPY(block) tokens for
// matches and LITERAL bytes otherwise -> only differing bytes cross the wire.
// Time: O(n) expected, bandwidth ~ size of the changes.
package main

import (
	"bytes"
	"fmt"
)

const M = 1 << 16
const B = 4

func fnv1a(d []byte, s, e int) uint64 {
	h := uint64(1469598103934665603)
	for i := s; i < e; i++ {
		h ^= uint64(d[i])
		h *= 1099511628211
	}
	return h
}

func weakFull(d []byte, s, e int) (int, int) {
	a, b := 0, 0
	for i := s; i < e; i++ {
		a = (a + int(d[i])) % M
		b = (b + (e-i)*int(d[i])) % M
	}
	return a, b
}

type token struct {
	copy bool
	idx  int
	lit  []byte
}

// signatures: receiver side, weak -> (strong -> index).
func signatures(old []byte) map[int]map[uint64]int {
	sigs := make(map[int]map[uint64]int)
	n := len(old) / B
	for i := 0; i < n; i++ {
		a, b := weakFull(old, i*B, i*B+B)
		weak := (b << 16) | a
		if sigs[weak] == nil {
			sigs[weak] = make(map[uint64]int)
		}
		sigs[weak][fnv1a(old, i*B, i*B+B)] = i
	}
	return sigs
}

func diff(nw []byte, sigs map[int]map[uint64]int) []token {
	var tokens []token
	var literal []byte
	pos, n := 0, len(nw)
	a, b := 0, 0
	haveWindow := false
	for pos+B <= n {
		if !haveWindow {
			a, b = weakFull(nw, pos, pos+B)
			haveWindow = true
		}
		weak := (b << 16) | a
		idx := -1
		if m, ok := sigs[weak]; ok {
			if j, ok2 := m[fnv1a(nw, pos, pos+B)]; ok2 {
				idx = j
			}
		}
		if idx >= 0 {
			if len(literal) > 0 {
				tokens = append(tokens, token{lit: append([]byte(nil), literal...)})
				literal = literal[:0]
			}
			tokens = append(tokens, token{copy: true, idx: idx})
			pos += B
			haveWindow = false
		} else {
			first := int(nw[pos])
			literal = append(literal, nw[pos])
			a = ((a-first+int(nw[pos+B]))%M + M) % M
			b = ((b-B*first+a)%M + M) % M
			pos++
		}
	}
	literal = append(literal, nw[pos:]...)
	if len(literal) > 0 {
		tokens = append(tokens, token{lit: append([]byte(nil), literal...)})
	}
	return tokens
}

func applyDelta(old []byte, tokens []token) []byte {
	var out []byte
	for _, t := range tokens {
		if t.copy {
			out = append(out, old[t.idx*B:t.idx*B+B]...)
		} else {
			out = append(out, t.lit...)
		}
	}
	return out
}

func main() {
	old := []byte("the quick brown fox jumps over the lazy dog")
	nw := []byte("the quick brown cat jumps over the lazy dog")
	sigs := signatures(old)
	tokens := diff(nw, sigs)
	rebuilt := applyDelta(old, tokens)
	literalBytes, copied := 0, 0
	for _, t := range tokens {
		if t.copy {
			copied++
		} else {
			literalBytes += len(t.lit)
		}
	}
	fmt.Println("synced:", bytes.Equal(rebuilt, nw))
	fmt.Println("literal bytes sent:", literalBytes, "of", len(nw))
	fmt.Println("blocks reused:", copied)
}
