// readN(n) on top of read7(): buffer leftover chars between calls.
// Time O(n) per readN call, Space O(1) extra (small buffer).
package main

import "fmt"

type Reader struct {
	content []rune
	pos     int    // read7 pointer
	buf     []rune // leftover unconsumed chars
}

func NewReader(s string) *Reader {
	return &Reader{content: []rune(s)}
}

func (r *Reader) read7() []rune {
	end := r.pos + 7
	if end > len(r.content) {
		end = len(r.content)
	}
	chunk := r.content[r.pos:end]
	r.pos = end
	return chunk
}

func (r *Reader) readN(n int) string {
	for len(r.buf) < n {
		chunk := r.read7()
		if len(chunk) == 0 {
			break
		}
		r.buf = append(r.buf, chunk...)
	}
	take := n
	if take > len(r.buf) {
		take = len(r.buf)
	}
	out := string(r.buf[:take])
	r.buf = r.buf[take:]
	return out
}

func main() {
	r := NewReader("Hello world")
	fmt.Printf("%q\n", r.readN(7))
	fmt.Printf("%q\n", r.readN(7))
	fmt.Printf("%q\n", r.readN(7))
}
