// Day 1297: Implement readN(n) on top of a read7() primitive.
// Keep a leftover buffer of unused chars; refill via read7 until n chars (or EOF). O(n) amortized.
package main

import "fmt"

type Reader struct {
	file string
	pos  int
}

func (r *Reader) Read7() string { // up to 7 chars, "" at EOF
	end := r.pos + 7
	if end > len(r.file) {
		end = len(r.file)
	}
	s := r.file[r.pos:end]
	r.pos = end
	return s
}

type NReader struct {
	r   *Reader
	buf string
}

func (nr *NReader) ReadN(n int) string {
	for len(nr.buf) < n {
		chunk := nr.r.Read7()
		if chunk == "" {
			break
		}
		nr.buf += chunk
	}
	take := n
	if take > len(nr.buf) {
		take = len(nr.buf)
	}
	out := nr.buf[:take]
	nr.buf = nr.buf[take:]
	return out
}

func main() {
	nr := &NReader{r: &Reader{file: "Hello world"}}
	fmt.Printf("'%s'\n", nr.ReadN(5))  // 'Hello'
	fmt.Printf("'%s'\n", nr.ReadN(4))  // ' wor'
	fmt.Printf("'%s'\n", nr.ReadN(10)) // 'ld'
}
