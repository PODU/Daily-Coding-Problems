// Day 1627: Debounce f by N ms. Each call resets an N-ms timer; f fires
// only after a quiet gap of N ms. Time O(1) per call.
#include <bits/stdc++.h>
using namespace std;

class Debounce {
    function<void()> f;
    chrono::milliseconds n;
    atomic<uint64_t> gen{0};
public:
    Debounce(function<void()> fn, int ms) : f(move(fn)), n(ms) {}
    void operator()() {
        uint64_t my = ++gen;
        thread([this, my]() {
            this_thread::sleep_for(n);
            if (gen.load() == my) f();
        }).detach();
    }
};

int main() {
    atomic<int> calls{0};
    Debounce g([&]() { calls++; }, 100);
    g(); g(); g();
    this_thread::sleep_for(chrono::milliseconds(200));
    cout << "f was called " << calls.load() << " time(s)" << endl;
    return 0;
}
