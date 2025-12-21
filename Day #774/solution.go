// Day 774: Implement readN(n) on top of a read7() primitive.
// Buffer leftover chars from read7 between calls. O(n) per readN call.
package main

import "fmt"

type Reader struct {
	file string
	pos  int
	buf  string
}

func (r *Reader) read7() string {
	end := r.pos + 7
	if end > len(r.file) {
		end = len(r.file)
	}
	s := r.file[r.pos:end]
	r.pos = end
	return s
}

func (r *Reader) readN(n int) string {
	out := make([]byte, 0, n)
	for len(out) < n {
		if len(r.buf) == 0 {
			r.buf = r.read7()
			if len(r.buf) == 0 {
				break
			}
		}
		take := n - len(out)
		if take > len(r.buf) {
			take = len(r.buf)
		}
		out = append(out, r.buf[:take]...)
		r.buf = r.buf[take:]
	}
	return string(out)
}

func main() {
	r := &Reader{file: "Hello world"}
	fmt.Printf("\"%s\", \"%s\", \"%s\"\n", r.readN(7), r.readN(7), r.readN(7))
	// "Hello w", "orld", ""
}
