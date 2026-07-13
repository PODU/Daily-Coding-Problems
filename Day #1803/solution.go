// Day 1803: Singleton holding two instances; getInstance alternates first (even call) / second (odd call).
// Lazy init guarded by sync.Once + mutex; O(1) per call, O(1) space.
package main

import (
	"fmt"
	"sync"
)

type dualSingleton struct{ id int }

var (
	first, second *dualSingleton
	counter       int
	once          sync.Once
	mu            sync.Mutex
)

func getInstance() *dualSingleton {
	once.Do(func() { first = &dualSingleton{1}; second = &dualSingleton{2} })
	mu.Lock()
	defer mu.Unlock()
	c := counter // call count starts at 0
	counter++
	if c%2 == 0 {
		return first
	}
	return second
}

func main() {
	var prevEven *dualSingleton
	for i := 0; i < 4; i++ {
		inst := getInstance()
		fmt.Printf("call%d->%d\n", i, inst.id)
		if i%2 == 0 {
			if prevEven != nil {
				fmt.Printf("  even-call identity same: %t\n", prevEven == inst)
			}
			prevEven = inst
		}
	}
}
