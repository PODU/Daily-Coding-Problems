// rsync-style sync: split destination into fixed blocks indexed by a weak rolling Adler-32 checksum
// + strong FNV-1a hash; slide a rolling window over the source to emit COPY/LITERAL tokens (only
// diffs sent), then reconstruct new = old blocks + literals. Time O(n) (constant block size).
package main

import "fmt"

const M = 65521
const BLOCK = 4

func fnv(d []byte, from, length int) uint64 {
	var h uint64 = 0xcbf29ce484222325
	for k := from; k < from+length; k++ {
		h ^= uint64(d[k])
		h *= 0x100000001b3
	}
	return h
}

type token struct {
	copy int // >= 0: copy block index; -1: literal
	lit  []byte
}

func main() {
	oldS := "the quick brown fox jumps over the lazy dog"
	newS := "the quick BROWN fox jumps over the lazy cat"
	oldB := []byte(oldS)
	newB := []byte(newS)

	idx := map[uint64][][2]uint64{} // weak -> list of {blockIndex, strong}
	nb := len(oldB) / BLOCK
	for bi := 0; bi < nb; bi++ {
		s := bi * BLOCK
		a, b := 0, 0
		for k := s; k < s+BLOCK; k++ {
			a = (a + int(oldB[k])) % M
			b = (b + a) % M
		}
		weak := uint64(a) + uint64(b)<<16
		idx[weak] = append(idx[weak], [2]uint64{uint64(bi), fnv(oldB, s, BLOCK)})
	}

	var tokens []token
	var lit []byte
	n := len(newB)
	pos, a, b := 0, 0, 0
	have := false
	for pos < n {
		if pos+BLOCK <= n {
			if !have {
				a, b = 0, 0
				for k := pos; k < pos+BLOCK; k++ {
					a = (a + int(newB[k])) % M
					b = (b + a) % M
				}
				have = true
			}
			weak := uint64(a) + uint64(b)<<16
			matched := -1
			if cand, ok := idx[weak]; ok {
				strong := fnv(newB, pos, BLOCK)
				for _, c := range cand {
					if c[1] == strong {
						matched = int(c[0])
						break
					}
				}
			}
			if matched >= 0 {
				if len(lit) > 0 {
					tokens = append(tokens, token{copy: -1, lit: append([]byte{}, lit...)})
					lit = lit[:0]
				}
				tokens = append(tokens, token{copy: matched})
				pos += BLOCK
				have = false
				continue
			}
			lit = append(lit, newB[pos])
			if pos+BLOCK < n { // roll forward one byte
				out, in := int(newB[pos]), int(newB[pos+BLOCK])
				a = ((a-out+in)%M + M) % M
				b = ((b-BLOCK*out+a)%M + M) % M
			} else {
				have = false
			}
			pos++
		} else {
			lit = append(lit, newB[pos])
			pos++
		}
	}
	if len(lit) > 0 {
		tokens = append(tokens, token{copy: -1, lit: append([]byte{}, lit...)})
	}

	var out []byte
	mb, lb := 0, 0
	for _, t := range tokens {
		if t.copy >= 0 {
			s := t.copy * BLOCK
			out = append(out, oldB[s:s+BLOCK]...)
			mb++
		} else {
			out = append(out, t.lit...)
			lb += len(t.lit)
		}
	}
	recon := string(out)
	fmt.Printf("Matched blocks: %d (%d bytes), Literal bytes: %d\n", mb, mb*BLOCK, lb)
	fmt.Println("Reconstructed: " + recon)
	fmt.Printf("Equals new file: %v\n", recon == newS)
}
