// Job scheduler: schedule f after n ms using time.AfterFunc. O(1) schedule, WaitGroup waits for completion.
package main

import (
	"fmt"
	"sync"
	"time"
)

func schedule(f func(), delayMs int) *time.Timer {
	return time.AfterFunc(time.Duration(delayMs)*time.Millisecond, f)
}

func main() {
	fmt.Println("Scheduling job...")
	var wg sync.WaitGroup
	wg.Add(1)
	schedule(func() {
		fmt.Println("Job executed!")
		wg.Done()
	}, 100)
	wg.Wait()
}
