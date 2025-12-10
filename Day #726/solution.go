// Day 726: Singleton holding TWO instances; alternate on even/odd getInstance() calls.
// Approach: Package counter; odd call -> instance #2, even call -> instance #1.
// Time: O(1) per call, Space: O(1).
package main

import (
	"fmt"
	"sync"
)

type dualSingleton struct{ id int }

var (
	first, second *dualSingleton
	count         int
	once          sync.Once
	mu            sync.Mutex
)

func getInstance() *dualSingleton {
	once.Do(func() {
		first = &dualSingleton{1}
		second = &dualSingleton{2}
	})
	mu.Lock()
	defer mu.Unlock()
	count++
	if count%2 == 0 {
		return first
	}
	return second
}

func main() {
	for i := 1; i <= 4; i++ {
		fmt.Printf("Call %d: instance %d\n", i, getInstance().id)
	}
}
