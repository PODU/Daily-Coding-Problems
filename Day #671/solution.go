// Day 671: Debounce. Each call stops the pending timer and arms a fresh N-ms one;
// f fires only after N ms of silence. Per-call O(1).
package main

import (
	"fmt"
	"sync"
	"time"
)

type Debouncer struct {
	mu    sync.Mutex
	timer *time.Timer
	n     time.Duration
	f     func(string)
}

func NewDebouncer(n time.Duration, f func(string)) *Debouncer {
	return &Debouncer{n: n, f: f}
}
func (d *Debouncer) Call(arg string) {
	d.mu.Lock()
	defer d.mu.Unlock()
	if d.timer != nil {
		d.timer.Stop()
	}
	d.timer = time.AfterFunc(d.n, func() { d.f(arg) })
}

func main() {
	d := NewDebouncer(50*time.Millisecond, func(s string) {
		fmt.Println("f called with:", s)
	})
	for _, s := range []string{"a", "b", "c", "d", "e"} {
		d.Call(s) // rapid burst
	}
	time.Sleep(200 * time.Millisecond) // prints once: f called with: e
}
