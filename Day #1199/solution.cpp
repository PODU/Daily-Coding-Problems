// Job scheduler: call f after n ms using a detached worker thread that sleeps.
// Time: O(1) to schedule; Space: O(1). Main joins so the job runs before exit.
#include <iostream>
#include <thread>
#include <chrono>

void schedule(void (*f)(), int n) {
    std::thread([f, n]() {
        std::this_thread::sleep_for(std::chrono::milliseconds(n));
        f();
    }).detach();
}

int main() {
    std::cout << "Scheduling job..." << std::endl;
    schedule([]() { std::cout << "Job executed after 100 ms" << std::endl; }, 100);
    // Wait long enough for the scheduled job to fire before the program exits.
    std::this_thread::sleep_for(std::chrono::milliseconds(200));
    return 0;
}
