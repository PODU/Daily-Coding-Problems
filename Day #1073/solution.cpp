// Job scheduler: schedule f() after n ms using std::thread + sleep_for. O(1) schedule, waits for completion.
#include <iostream>
#include <thread>
#include <chrono>
#include <functional>

void schedule(std::function<void()> f, int delayMs) {
    std::thread([f, delayMs]() {
        std::this_thread::sleep_for(std::chrono::milliseconds(delayMs));
        f();
    }).detach();
}

int main() {
    std::cout << "Scheduling job..." << std::endl;
    bool done = false;
    schedule([&done]() {
        std::cout << "Job executed!" << std::endl;
        done = true;
    }, 100);
    // Wait for the job to complete
    while (!done) {
        std::this_thread::sleep_for(std::chrono::milliseconds(10));
    }
    return 0;
}
