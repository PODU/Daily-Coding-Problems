// Day 1179: Debounce - call f only after N ms elapse with no further invocations.
// Deterministic simulation over timestamped calls: a pending call fires only once
// a gap of >= N ms occurs with no new call. Time O(1) per call, Space O(1).
// (A real implementation would use a timer/thread; this is thread-free and portable.)
#include <bits/stdc++.h>
using namespace std;

struct Debouncer {
    long long N;
    function<void()> f;
    bool pending = false;
    long long deadline = 0;

    Debouncer(long long n, function<void()> fn) : N(n), f(move(fn)) {}

    void call(long long t) {                 // invoke at time t (ms)
        if (pending && t >= deadline) f();   // previous burst settled before this call
        pending = true;
        deadline = t + N;
    }
    void flush() {                           // no more calls: let the last one fire
        if (pending) { f(); pending = false; }
    }
};

int main() {
    int count = 0;
    Debouncer d(100, [&]() { count++; });
    for (int i = 0; i < 5; i++) d.call(i * 20); // rapid burst every 20ms, N=100ms
    d.flush();
    cout << "f executed " << count << " time(s)\n"; // 1
    return 0;
}
