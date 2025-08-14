// Day 120: Dual singleton. Two fixed instances; odd call -> 2nd, even call -> 1st.
// getInstance() is O(1); instances created once via sync.Once.
package main

import (
	"fmt"
	"sync"
)

type instance struct{ name string }

var (
	first, second *instance
	count         int64
	once          sync.Once
	mu            sync.Mutex
)

func getInstance() *instance {
	once.Do(func() {
		first = &instance{name: "first"}
		second = &instance{name: "second"}
	})
	mu.Lock()
	defer mu.Unlock()
	count++ // 1-based call number
	if count%2 == 0 {
		return first // even -> first
	}
	return second // odd -> second
}

func main() {
	for i := 0; i < 4; i++ {
		fmt.Println(getInstance().name) // second, first, second, first
	}
}
