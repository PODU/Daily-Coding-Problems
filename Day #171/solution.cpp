// Busiest period: sort events by timestamp, sweep occupancy, track max-occupancy interval. O(n log n) time, O(n) space.
#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

enum Type { ENTER, EXIT };
struct Event { long long ts; int count; Type type; };

pair<long long,long long> busiestPeriod(vector<Event> events) {
    sort(events.begin(), events.end(), [](const Event& a, const Event& b){ return a.ts < b.ts; });
    long long occ = 0, maxOcc = -1, bestStart = 0, bestEnd = 0;
    for (size_t i = 0; i < events.size(); i++) {
        occ += (events[i].type == ENTER) ? events[i].count : -events[i].count;
        if (occ > maxOcc && i + 1 < events.size()) {
            maxOcc = occ;
            bestStart = events[i].ts;
            bestEnd = events[i + 1].ts;
        }
    }
    return {bestStart, bestEnd};
}

int main() {
    vector<Event> events = {
        {1526579928, 3, ENTER},
        {1526580382, 2, EXIT},
        {1526579999, 1, ENTER},
        {1526580001, 5, ENTER}
    };
    auto r = busiestPeriod(events);
    cout << "(" << r.first << ", " << r.second << ")" << endl;
    return 0;
}
