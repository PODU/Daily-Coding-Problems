// Debounce: wrap f so it runs only after N ms of silence; each call resets the timer.
// A detached worker waits N ms then fires only if its generation is still the latest
// (atomic generation counter). Time: O(1) per call, Space: O(1).
#include <atomic>
#include <chrono>
#include <functional>
#include <iostream>
#include <mutex>
#include <thread>

class Debouncer {
    std::atomic<uint64_t> gen{0};
    std::chrono::milliseconds delay;
public:
    explicit Debouncer(int n_ms) : delay(n_ms) {}

    void call(std::function<void(int)> f, int arg) {
        uint64_t myGen = ++gen;
        auto d = delay;
        std::thread([this, f, arg, myGen, d]() {
            std::this_thread::sleep_for(d);
            if (gen.load() == myGen) f(arg); // fire only if still newest
        }).detach();
    }
};

int main() {
    std::mutex mtx;
    int calls = 0;
    Debouncer deb(100);

    auto target = [&](int x) {
        std::lock_guard<std::mutex> lock(mtx);
        ++calls;
        std::cout << "f called with " << x << "; total calls = " << calls << "\n";
    };

    for (int i = 1; i <= 5; ++i) deb.call(target, i); // burst of 5 calls

    std::this_thread::sleep_for(std::chrono::milliseconds(300)); // wait > N ms
    std::lock_guard<std::mutex> lock(mtx);
    std::cout << "done; f ran " << calls << " time(s)\n";
    return 0;
}
