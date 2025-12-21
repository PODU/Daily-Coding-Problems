// Day 775: Minimum meeting rooms = max overlapping intervals.
// Sort starts and ends, two-pointer sweep. O(n log n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

int minRooms(vector<pair<int,int>> iv) {
    int n = iv.size();
    vector<int> starts(n), ends(n);
    for (int i = 0; i < n; i++) { starts[i] = iv[i].first; ends[i] = iv[i].second; }
    sort(starts.begin(), starts.end());
    sort(ends.begin(), ends.end());
    int rooms = 0, best = 0, i = 0, j = 0;
    while (i < n) {
        if (starts[i] < ends[j]) { rooms++; i++; best = max(best, rooms); }
        else { rooms--; j++; }
    }
    return best;
}

int main() {
    cout << minRooms({{30, 75}, {0, 50}, {60, 150}}) << endl; // 2
    return 0;
}
