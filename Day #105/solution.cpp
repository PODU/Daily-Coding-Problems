// Day 105: Debounce (trailing edge). Deterministic virtual-clock model: replay
// call timestamps; f fires only after a quiet gap of >= N ms. O(1) per call.
#include <iostream>
#include <functional>
using namespace std;

class Debouncer {
    long long n;
    function<void()> f;
    bool pending = false;
    long long lastCall = 0;
public:
    Debouncer(function<void()> fn, long long ms) : n(ms), f(move(fn)) {}
    // Register a call at virtual time `now`. If the previous burst already went
    // quiet for >= N, it fires first; otherwise this call resets the timer.
    void callAt(long long now) {
        if (pending && now - lastCall >= n) f();
        pending = true;
        lastCall = now;
    }
    // No more calls coming: let the final interval elapse and fire once.
    void flush(long long now) {
        if (pending && now - lastCall >= n) { f(); pending = false; }
    }
};

int main() {
    int count = 0;
    Debouncer d([&]() { count++; cout << "f called\n"; }, 100);
    d.callAt(0); d.callAt(10); d.callAt(20); // 3 rapid calls within the window
    d.flush(120);                            // quiet period elapses
    cout << "Total calls to f: " << count << "\n"; // 1
    return 0;
}
