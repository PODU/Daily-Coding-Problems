// Day 105: Debounce. Each call stops the pending timer and starts a fresh N-ms
// one, so f only fires after N ms of quiet. O(1) per call.
package main

import (
	"fmt"
	"sync"
	"time"
)

func debounce(d time.Duration, f func()) func() {
	var mu sync.Mutex
	var t *time.Timer
	return func() {
		mu.Lock()
		defer mu.Unlock()
		if t != nil {
			t.Stop()
		}
		t = time.AfterFunc(d, f)
	}
}

func main() {
	count := 0
	done := make(chan struct{})
	deb := debounce(100*time.Millisecond, func() {
		count++
		fmt.Println("f called")
		close(done)
	})
	deb(); deb(); deb() // 3 rapid calls
	<-done
	fmt.Println("Total calls to f:", count) // 1
}
