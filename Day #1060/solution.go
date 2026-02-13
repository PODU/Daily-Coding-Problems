// Debounce: wrap f so it runs only after N ms of silence; each call resets the timer.
// Uses time.AfterFunc with Stop/reset. Time: O(1) per call, Space: O(1).
package main

import (
	"fmt"
	"sync"
	"time"
)

func debounce(f func(int), n time.Duration) func(int) {
	var mu sync.Mutex
	var timer *time.Timer
	return func(arg int) {
		mu.Lock()
		defer mu.Unlock()
		if timer != nil {
			timer.Stop()
		}
		timer = time.AfterFunc(n, func() { f(arg) })
	}
}

func main() {
	var mu sync.Mutex
	calls := 0

	debounced := debounce(func(x int) {
		mu.Lock()
		calls++
		fmt.Printf("f called with %d; total calls = %d\n", x, calls)
		mu.Unlock()
	}, 100*time.Millisecond)

	for i := 1; i <= 5; i++ { // burst of 5 calls
		debounced(i)
	}

	time.Sleep(300 * time.Millisecond) // wait > N ms
	mu.Lock()
	fmt.Printf("done; f ran %d time(s)\n", calls)
	mu.Unlock()
}
