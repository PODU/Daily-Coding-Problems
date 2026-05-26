// Busiest period: sort events by timestamp, sweep current occupancy, track max interval.
// Time O(n log n), space O(n).
#include <bits/stdc++.h>
using namespace std;

struct Event { long long ts; long long count; bool enter; };

pair<long long,long long> busiestPeriod(vector<Event> events) {
    sort(events.begin(), events.end(), [](const Event& a, const Event& b){
        return a.ts < b.ts;
    });
    long long cur = 0, best = -1, bestStart = 0, bestEnd = 0;
    for (size_t i = 0; i < events.size(); i++) {
        cur += events[i].enter ? events[i].count : -events[i].count;
        if (cur > best && i + 1 < events.size()) {
            best = cur;
            bestStart = events[i].ts;
            bestEnd = events[i + 1].ts;
        }
    }
    return {bestStart, bestEnd};
}

int main() {
    vector<Event> events = {
        {1, 3, true},
        {4, 2, true},
        {6, 5, false}
    };
    auto r = busiestPeriod(events);
    cout << "(" << r.first << ", " << r.second << ")" << endl;
    return 0;
}
