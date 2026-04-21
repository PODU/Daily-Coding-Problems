// rsync-style delta sync: index fixed blocks of the old file by a rolling
// (Adler-like) weak hash + exact-block strong check; slide a rolling window over
// the new file emitting COPY(block) or literal bytes. Time O(N) avg, Space O(old/B).
package main

import "fmt"

const B = 4

func weak(s []byte, off, length int) (int, int) {
	a, b := 0, 0
	for i := 0; i < length; i++ {
		c := int(s[off+i])
		a = (a + c) & 0xFFFF
		b = (b + (length-i)*c) & 0xFFFF
	}
	return a, b
}

type tok struct {
	copy bool
	idx  int
	lit  byte
}

type entry struct {
	idx int
	blk string
}

func makeDelta(old, nw []byte) []tok {
	table := map[int][]entry{}
	for idx := 0; idx+B <= len(old); idx += B {
		a, b := weak(old, idx, B)
		h := (b << 16) | a
		table[h] = append(table[h], entry{idx, string(old[idx : idx+B])})
	}
	var delta []tok
	n, i, a, b := len(nw), 0, 0, 0
	if n >= B {
		a, b = weak(nw, 0, B)
	}
	for i < n {
		if i+B <= n {
			h := (b << 16) | a
			match := -1
			win := string(nw[i : i+B])
			for _, e := range table[h] {
				if e.blk == win {
					match = e.idx
					break
				}
			}
			if match >= 0 {
				delta = append(delta, tok{copy: true, idx: match})
				i += B
				if i+B <= n {
					a, b = weak(nw, i, B)
				}
				continue
			}
		}
		delta = append(delta, tok{lit: nw[i]})
		if i+B < n {
			out, in := int(nw[i]), int(nw[i+B])
			a = (a - out + in) & 0xFFFF
			b = (b - B*out + a) & 0xFFFF
		}
		i++
	}
	return delta
}

func rebuild(old []byte, delta []tok) []byte {
	var out []byte
	for _, t := range delta {
		if t.copy {
			out = append(out, old[t.idx:t.idx+B]...)
		} else {
			out = append(out, t.lit)
		}
	}
	return out
}

func main() {
	old := []byte("the quick brown fox jumps over the lazy dog")
	nw := []byte("the quick brown cat jumps over the lazy dog")
	delta := makeDelta(old, nw)
	copies, lits := 0, 0
	for _, t := range delta {
		if t.copy {
			copies++
		} else {
			lits++
		}
	}
	rb := rebuild(old, delta)
	fmt.Println("Reconstructed:", string(rb))
	fmt.Println("Match:", string(rb) == string(nw))
	fmt.Println("copied blocks:", copies, "literal bytes:", lits)
}
