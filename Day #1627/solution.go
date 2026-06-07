// Day 1627: Debounce f by N ms. Each call resets an N-ms timer; f fires
// only after a quiet gap of N ms. Time O(1) per call.
package main

import (
	"fmt"
	"sync"
	"time"
)

func debounce(f func(), n time.Duration) func() {
	var mu sync.Mutex
	var timer *time.Timer
	return func() {
		mu.Lock()
		defer mu.Unlock()
		if timer != nil {
			timer.Stop()
		}
		timer = time.AfterFunc(n, f)
	}
}

func main() {
	var mu sync.Mutex
	calls := 0
	g := debounce(func() {
		mu.Lock()
		calls++
		mu.Unlock()
	}, 100*time.Millisecond)
	g()
	g()
	g()
	time.Sleep(200 * time.Millisecond)
	mu.Lock()
	fmt.Printf("f was called %d time(s)\n", calls)
	mu.Unlock()
}
