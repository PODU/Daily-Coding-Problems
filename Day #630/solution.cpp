// Job scheduler: run f after n ms using a background thread that sleeps then calls f.
// std::thread + sleep_for + join. Time O(1) to schedule; main joins to wait.
#include <iostream>
#include <thread>
#include <chrono>
#include <functional>
using namespace std;

void schedule(function<void()> f, int n) {
    thread t([f, n]() {
        this_thread::sleep_for(chrono::milliseconds(n));
        f();
    });
    t.join(); // wait for the job to run before returning
}

int main() {
    schedule([]() {
        cout << "Job executed after 100 ms" << "\n";
    }, 100);
    cout << "Main: job completed, exiting" << "\n";
    return 0;
}
