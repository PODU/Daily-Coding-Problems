// Day 82: readN(n) built on read7() by buffering leftover characters between calls.
// Time O(n) per call amortized, Space O(7) buffer.
package main

import "fmt"

type Reader struct {
	file   string
	pos    int
	buffer string
}

// Returns up to 7 characters from the file, advancing the cursor.
func (r *Reader) read7() string {
	end := r.pos + 7
	if end > len(r.file) {
		end = len(r.file)
	}
	chunk := r.file[r.pos:end]
	r.pos = end
	return chunk
}

func (r *Reader) readN(n int) string {
	result := ""
	for len(result) < n {
		if len(r.buffer) == 0 {
			chunk := r.read7()
			if len(chunk) == 0 {
				break // EOF
			}
			r.buffer = chunk
		}
		take := len(r.buffer)
		if take > n-len(result) {
			take = n - len(result)
		}
		result += r.buffer[:take]
		r.buffer = r.buffer[take:]
	}
	return result
}

func main() {
	r := &Reader{file: "Hello world"}
	fmt.Printf("%q\n", r.readN(7)) // "Hello w"
	fmt.Printf("%q\n", r.readN(7)) // "orld"
	fmt.Printf("%q\n", r.readN(7)) // ""
}
