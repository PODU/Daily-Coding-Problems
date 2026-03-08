// Day 1179: Debounce - call f only after N ms elapse with no further invocations.
// Each call stops the pending time.Timer and starts a fresh one (mutex-guarded).
// Time O(1) per call.
package main

import (
	"fmt"
	"sync"
	"sync/atomic"
	"time"
)

type Debouncer struct {
	mu    sync.Mutex
	timer *time.Timer
	delay time.Duration
	f     func()
}

func NewDebouncer(delay time.Duration, f func()) *Debouncer {
	return &Debouncer{delay: delay, f: f}
}

func (d *Debouncer) Call() {
	d.mu.Lock()
	defer d.mu.Unlock()
	if d.timer != nil {
		d.timer.Stop()
	}
	d.timer = time.AfterFunc(d.delay, d.f)
}

func main() {
	var count int64
	d := NewDebouncer(100*time.Millisecond, func() { atomic.AddInt64(&count, 1) })
	for i := 0; i < 5; i++ { // rapid burst, every 20ms
		d.Call()
		time.Sleep(20 * time.Millisecond)
	}
	time.Sleep(300 * time.Millisecond)
	fmt.Printf("f executed %d time(s)\n", atomic.LoadInt64(&count)) // 1
}
