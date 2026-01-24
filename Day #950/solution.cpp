// Day 950: busiest period - interval with the most people inside the building.
// Sort events by timestamp, sweep maintaining running count. Time O(n log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct Event { long long ts; int count; bool enter; };

pair<long long, long long> busiest(vector<Event> ev) {
    sort(ev.begin(), ev.end(), [](const Event& a, const Event& b) {
        return a.ts < b.ts;
    });
    long long cur = 0, best = -1;
    pair<long long, long long> ans = {0, 0};
    for (size_t i = 0; i < ev.size(); ++i) {
        cur += ev[i].enter ? ev[i].count : -ev[i].count;
        long long nextTs = (i + 1 < ev.size()) ? ev[i + 1].ts : ev[i].ts;
        if (cur > best) { best = cur; ans = {ev[i].ts, nextTs}; }
    }
    return ans;
}

int main() {
    vector<Event> ev = {
        {1526579928, 3, true},
        {1526579943, 4, true},
        {1526580382, 2, false},
        {1526581000, 5, false},
    };
    auto p = busiest(ev);
    cout << "(" << p.first << ", " << p.second << ")\n"; // (1526579943, 1526580382)
    return 0;
}
