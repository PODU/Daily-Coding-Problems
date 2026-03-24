// Busiest period: sort events by timestamp, sweep occupancy, track max-occupancy interval [t_i, t_{i+1}). O(n log n) time, O(n) space.
#include <iostream>
#include <vector>
#include <algorithm>
#include <string>
using namespace std;

struct Event { long timestamp; int count; string type; };

int main() {
    vector<Event> events = {
        {1526579928, 3, "enter"},
        {1526580000, 2, "enter"},
        {1526580382, 2, "exit"},
        {1526580500, 1, "enter"},
        {1526580700, 4, "exit"}
    };
    sort(events.begin(), events.end(), [](const Event& a, const Event& b) {
        return a.timestamp < b.timestamp;
    });

    long current = 0, maxOcc = -1;
    long bestStart = 0, bestEnd = 0;
    for (size_t i = 0; i < events.size(); ++i) {
        if (events[i].type == "enter") current += events[i].count;
        else current -= events[i].count;
        long nextTs = (i + 1 < events.size()) ? events[i + 1].timestamp : events[i].timestamp;
        if (current > maxOcc) {
            maxOcc = current;
            bestStart = events[i].timestamp;
            bestEnd = nextTs;
        }
    }
    cout << "(" << bestStart << ", " << bestEnd << ")" << "\n";
    return 0;
}
