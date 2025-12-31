// Day 826: rsync-style file sync over a low-bandwidth link.
// Receiver signs old file's fixed-size blocks (weak rolling Adler checksum +
// strong FNV-1a hash); sender rolls a window over the new file to find matching
// blocks, emitting COPY/LITERAL tokens; receiver rebuilds.
// Time O(n) average (rolling hash), Space O(n).
package main

import (
	"bytes"
	"fmt"
)

const MOD = 1 << 16
const L = 4

func strongHash(b []byte) uint64 {
	var h uint64 = 1469598103934665603
	for _, x := range b {
		h ^= uint64(x)
		h *= 1099511628211
	}
	return h
}

func weakBlock(b []byte) (int, int, int) {
	a, s := 0, 0
	n := len(b)
	for k := 0; k < n; k++ {
		a += int(b[k])
		s += (n - k) * int(b[k])
	}
	a %= MOD
	s %= MOD
	return a, s, a + MOD*s
}

type token struct {
	copy bool
	idx  int
	lit  []byte
}

type entry struct {
	strong uint64
	idx    int
}

func main() {
	oldF := []byte("the quick brown fox jumps")
	newF := []byte("the quick red fox leaps over")

	// signature
	var blocks [][]byte
	for i := 0; i < len(oldF); i += L {
		end := i + L
		if end > len(oldF) {
			end = len(oldF)
		}
		blocks = append(blocks, oldF[i:end])
	}
	table := map[int][]entry{}
	for idx, blk := range blocks {
		if len(blk) == L {
			_, _, w := weakBlock(blk)
			table[w] = append(table[w], entry{strongHash(blk), idx})
		}
	}

	// diff
	var delta []token
	var lit []byte
	n := len(newF)
	i := 0
	a, s, cs := 0, 0, 0
	have := false
	for i < n {
		if i+L <= n {
			if !have {
				a, s, cs = weakBlock(newF[i : i+L])
				have = true
			}
			matched := false
			if bucket, ok := table[cs]; ok {
				win := newF[i : i+L]
				hh := strongHash(win)
				for _, e := range bucket {
					if e.strong == hh && bytes.Equal(blocks[e.idx], win) {
						if len(lit) > 0 {
							l := make([]byte, len(lit))
							copy(l, lit)
							delta = append(delta, token{lit: l})
							lit = lit[:0]
						}
						delta = append(delta, token{copy: true, idx: e.idx})
						i += L
						have = false
						matched = true
						break
					}
				}
			}
			if matched {
				continue
			}
			lit = append(lit, newF[i])
			if i+L < n {
				out, in := int(newF[i]), int(newF[i+L])
				a = ((a-out+in)%MOD + MOD) % MOD
				s = ((s-L*out+a)%MOD + MOD) % MOD
				cs = a + MOD*s
			}
			i++
		} else {
			lit = append(lit, newF[i])
			i++
		}
	}
	if len(lit) > 0 {
		l := make([]byte, len(lit))
		copy(l, lit)
		delta = append(delta, token{lit: l})
	}

	// reconstruct
	var rebuilt []byte
	copies := 0
	for _, t := range delta {
		if t.copy {
			rebuilt = append(rebuilt, blocks[t.idx]...)
			copies++
		} else {
			rebuilt = append(rebuilt, t.lit...)
		}
	}

	fmt.Println(string(rebuilt))
	if bytes.Equal(rebuilt, newF) {
		fmt.Println("reconstruction OK: True")
	} else {
		fmt.Println("reconstruction OK: False")
	}
	fmt.Println("blocks reused (copy tokens):", copies)
}
