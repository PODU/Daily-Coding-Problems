// Job scheduler: spawn a worker thread that sleeps n ms then invokes f.
// schedule: O(1); each job runs on its own timer thread.
#include <bits/stdc++.h>
#include <thread>
#include <chrono>
using namespace std;

void schedule(function<void()> f, int n) {
    thread([f, n]() {
        this_thread::sleep_for(chrono::milliseconds(n));
        f();
    }).detach();
}

int main() {
    schedule([]() { cout << "Executed after delay!\n"; }, 100);
    this_thread::sleep_for(chrono::milliseconds(200)); // let the job fire before exit
    return 0;
}
