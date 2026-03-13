// Job scheduler: call f after n ms using the native setTimeout one-shot timer.
// Time: O(1) to schedule; Space: O(1). Node keeps the event loop alive until it fires.
function schedule(f, n) {
    setTimeout(f, n);
}

console.log("Scheduling job...");
schedule(() => console.log("Job executed after 100 ms"), 100);
// Node will not exit while a pending timer is scheduled, so the job runs before exit.
