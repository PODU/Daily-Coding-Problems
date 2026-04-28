// Day 1435: Singleton holding two instances; even call -> first, odd call -> second.
// Approach: Two package instances + call counter, return by parity of count.
// Time: O(1) per call, Space: O(1).
package main

import (
	"fmt"
	"sync"
)

type dualSingleton struct {
	id int
}

var (
	first   = &dualSingleton{1}
	second  = &dualSingleton{2}
	counter int
	mu      sync.Mutex
)

func getInstance() *dualSingleton {
	mu.Lock()
	defer mu.Unlock()
	n := counter // 0-indexed call number
	counter++
	if n%2 == 0 {
		return first
	}
	return second
}

func main() {
	for i := 0; i < 4; i++ {
		inst := getInstance()
		fmt.Printf("Call %d -> instance %d\n", i, inst.id)
	}
	// Even calls (0,2) -> instance 1, odd calls (1,3) -> instance 2
}
