// readN using read7: buffer leftover chars from read7 between calls; pull until n satisfied or EOF.
// Time O(n) per readN call.
package main

import "fmt"

type Reader struct {
	file string
	pos  int    // internal pointer for read7
	buf  string // leftover chars buffered between readN calls
}

// read7 primitive: up to 7 chars, advances pointer, "" at EOF
func (r *Reader) read7() string {
	end := r.pos + 7
	if end > len(r.file) {
		end = len(r.file)
	}
	res := r.file[r.pos:end]
	r.pos = end
	return res
}

// readN: read exactly n chars (or fewer at EOF), buffering leftovers
func (r *Reader) readN(n int) string {
	out := make([]byte, 0, n)
	for len(out) < n {
		if len(r.buf) == 0 {
			r.buf = r.read7()
			if len(r.buf) == 0 {
				break // EOF
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
	r1 := &Reader{file: "Hello world"}
	fmt.Printf("read7: \"%s\"\n", r1.read7())
	fmt.Printf("read7: \"%s\"\n", r1.read7())
	fmt.Printf("read7: \"%s\"\n", r1.read7())

	r2 := &Reader{file: "Hello world"}
	fmt.Printf("readN(5): \"%s\"\n", r2.readN(5))
	fmt.Printf("readN(100): \"%s\"\n", r2.readN(100))
	fmt.Printf("readN(3): \"%s\"\n", r2.readN(3))
}
