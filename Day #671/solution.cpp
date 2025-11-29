// Day 671: Debounce. A call records (arg, time); f fires only once the clock has advanced
// N ms past the most recent call (i.e. N ms of silence). Deterministic, thread-free. Per-call O(1).
#include <bits/stdc++.h>
using namespace std;

struct Debouncer {
    long long N;
    function<void(string)> f;
    bool pending = false;
    long long lastTime = 0;
    string lastArg;
    Debouncer(long long n, function<void(string)> fn) : N(n), f(fn) {}
    void call(const string& arg, long long now) { pending = true; lastTime = now; lastArg = arg; }
    // advance the clock; fires the pending call if N ms of silence elapsed
    void tick(long long now) {
        if (pending && now - lastTime >= N) { f(lastArg); pending = false; }
    }
};

int main() {
    Debouncer d(50, [](const string& s) { cout << "f called with: " << s << "\n"; });
    long long t = 0;
    for (string s : {"a", "b", "c", "d", "e"}) d.call(s, t++); // rapid burst at t=0..4ms
    d.tick(200); // 200ms later: prints once -> f called with: e
    return 0;
}
