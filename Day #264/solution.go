// Day 264: De Bruijn sequence B(k, n) over a character set.
// Approach: Fredricksen-Kessler-Maiorana algorithm — concatenate Lyndon words
// whose length divides n, generated via Duval-style recursion.
// Time O(k^n) (size of the output), Space O(n).

package main

import (
	"fmt"
	"strings"
)

type deBruijn struct {
	alphabet []byte
	n, k     int
	a        []int
	sequence strings.Builder
}

func (d *deBruijn) db(t, p int) {
	if t > d.n {
		if d.n%p == 0 {
			for i := 1; i <= p; i++ {
				d.sequence.WriteByte(d.alphabet[d.a[i]])
			}
		}
	} else {
		d.a[t] = d.a[t-p]
		d.db(t+1, p)
		for j := d.a[t-p] + 1; j < d.k; j++ {
			d.a[t] = j
			d.db(t+1, t)
		}
	}
}

func build(alphabet []byte, n int) string {
	d := &deBruijn{alphabet: alphabet, n: n, k: len(alphabet)}
	d.a = make([]int, d.k*n)
	d.db(1, 1)
	return d.sequence.String()
}

func main() {
	// C = {0, 1}, k = 3
	fmt.Println(build([]byte{'0', '1'}, 3))
}
