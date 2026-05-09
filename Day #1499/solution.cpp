// Day 1499: Job scheduler that calls f after n ms. Spawn a detached-style
// thread that sleeps n ms then invokes f; join to wait. Time O(1), Space O(1).
#include <iostream>
#include <thread>
#include <chrono>
#include <functional>
using namespace std;

void schedule(function<void()> f, int n, thread& t) {
    t = thread([f, n]() {
        this_thread::sleep_for(chrono::milliseconds(n));
        f();
    });
}

int main() {
    thread t;
    schedule([]() { cout << "Job executed after delay" << endl; }, 100, t);
    t.join(); // wait for the scheduled job before exiting
    return 0;
}
