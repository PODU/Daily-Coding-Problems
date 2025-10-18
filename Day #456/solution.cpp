// Day 456: Busiest period in a building from enter/exit events.
// Sort by timestamp, sweep occupancy, track interval of max. Time O(n log n).
#include <iostream>
#include <vector>
#include <algorithm>
#include <string>
using namespace std;

struct Event { long ts; int count; string type; };

pair<long, long> busiest(vector<Event> ev) {
    sort(ev.begin(), ev.end(), [](const Event& a, const Event& b) { return a.ts < b.ts; });
    long cur = 0, best = -1, bestStart = 0, bestEnd = 0;
    int n = (int)ev.size();
    for (int i = 0; i < n; i++) {
        cur += (ev[i].type == "enter" ? ev[i].count : -ev[i].count);
        long end = (i + 1 < n) ? ev[i + 1].ts : ev[i].ts;
        if (cur > best) { best = cur; bestStart = ev[i].ts; bestEnd = end; }
    }
    return {bestStart, bestEnd};
}

int main() {
    vector<Event> ev = {
        {1526579928, 3, "enter"},
        {1526579940, 2, "enter"},
        {1526580000, 1, "exit"},
        {1526580382, 4, "exit"},
    };
    auto r = busiest(ev);
    cout << "(" << r.first << ", " << r.second << ")" << endl;
    return 0;
}
