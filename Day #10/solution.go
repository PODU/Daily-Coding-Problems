// Job scheduler: use time.AfterFunc to invoke f after n milliseconds.
// schedule: O(1); each job runs on the runtime timer.
package main

import (
	"fmt"
	"sync"
	"time"
)

func schedule(f func(), n int) *time.Timer {
	return time.AfterFunc(time.Duration(n)*time.Millisecond, f)
}

func main() {
	var wg sync.WaitGroup
	wg.Add(1)
	schedule(func() {
		fmt.Println("Executed after delay!")
		wg.Done()
	}, 100)
	wg.Wait() // let the job fire before exit
}
