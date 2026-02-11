// Day 1054: Dual-instance singleton. Holds two lazily-created instances and an
// alternating counter; getInstance() returns instance[count % 2], then bumps the
// counter. Time O(1) per call, Space O(1).

package main

import (
	"fmt"
	"sync"
)

type DualSingleton struct {
	id int
}

var (
	instances []*DualSingleton
	count     int
	mu        sync.Mutex
)

func getInstance() *DualSingleton {
	mu.Lock()
	defer mu.Unlock()
	if instances == nil {
		instances = []*DualSingleton{{id: 1}, {id: 2}}
	}
	inst := instances[count%2]
	count++
	return inst
}

func main() {
	for call := 0; call < 6; call++ {
		kind, parity := "first", "even"
		if call%2 != 0 {
			kind, parity = "second", "odd"
		}
		fmt.Printf("call %d (%s) -> %s instance (id=%d)\n", call, parity, kind, getInstance().id)
	}
}
