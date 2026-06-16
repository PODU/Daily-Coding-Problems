// Day 1674: Low-bandwidth file sync (rsync algorithm).
// Receiver sends per-block weak(rolling)+strong checksums; sender emits COPY/LITERAL
// tokens using a rolling hash to find matches. Time O(n) amortized, transfers only diffs.
package main

import (
	"bytes"
	"fmt"
)

const M = 1 << 16

func weakInit(d []byte, i, L int) (int, int) {
	a, b := 0, 0
	for k := i; k < i+L; k++ {
		a = (a + int(d[k])) % M
		b = (b + (L-(k-i))*int(d[k])) % M
	}
	return a, b
}
func weakRoll(a, b int, d []byte, i, L int) (int, int) {
	out, in := int(d[i]), int(d[i+L])
	a = ((a-out+in)%M + M) % M
	b = ((b-L*out+a)%M + M) % M
	return a, b
}
func strong(s []byte, i, L int) uint64 {
	h := uint64(1469598103934665603)
	for k := i; k < i+L; k++ {
		h ^= uint64(s[k])
		h *= 1099511628211
	}
	return h
}

type tok struct {
	copy bool
	idx  int
	lit  []byte
}

func diff(old, neu []byte, L int) []tok {
	table := map[int64]map[uint64]int{}
	nblocks := len(old) / L
	for idx := 0; idx < nblocks; idx++ {
		a, b := weakInit(old, idx*L, L)
		w := int64(b)<<16 | int64(a)
		if table[w] == nil {
			table[w] = map[uint64]int{}
		}
		table[w][strong(old, idx*L, L)] = idx
	}
	var tokens []tok
	var lit []byte
	i, n, a, b, have := 0, len(neu), 0, 0, false
	for i < n {
		if i+L <= n {
			if !have {
				a, b = weakInit(neu, i, L)
				have = true
			}
			w := int64(b)<<16 | int64(a)
			if bucket, ok := table[w]; ok {
				if idx, ok2 := bucket[strong(neu, i, L)]; ok2 {
					if len(lit) > 0 {
						tokens = append(tokens, tok{false, 0, lit})
						lit = nil
					}
					tokens = append(tokens, tok{true, idx, nil})
					i += L
					have = false
					continue
				}
			}
			lit = append(lit, neu[i])
			if i+L <= n-1 {
				a, b = weakRoll(a, b, neu, i, L)
			} else {
				have = false
			}
			i++
		} else {
			lit = append(lit, neu[i])
			i++
		}
	}
	if len(lit) > 0 {
		tokens = append(tokens, tok{false, 0, lit})
	}
	return tokens
}
func patch(old []byte, tokens []tok, L int) []byte {
	var out bytes.Buffer
	for _, t := range tokens {
		if t.copy {
			out.Write(old[t.idx*L : t.idx*L+L])
		} else {
			out.Write(t.lit)
		}
	}
	return out.Bytes()
}

func main() {
	L := 5
	old := []byte("the quick brown fox jumps over")
	neu := []byte("the quick brown cat jumps over")
	tokens := diff(old, neu, L)
	rebuilt := patch(old, tokens, L)
	litBytes := 0
	for _, t := range tokens {
		if !t.copy {
			litBytes += len(t.lit)
		}
	}
	fmt.Println(bytes.Equal(rebuilt, neu)) // true
	fmt.Println(litBytes, "of", len(neu))
}
