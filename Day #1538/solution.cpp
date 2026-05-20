// Minimum rooms for overlapping intervals: sort starts & ends, sweep with two pointers.
// Time O(n log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

int minRooms(vector<pair<int,int>> intervals) {
    int n = intervals.size();
    vector<int> starts(n), ends(n);
    for (int i = 0; i < n; i++) { starts[i] = intervals[i].first; ends[i] = intervals[i].second; }
    sort(starts.begin(), starts.end());
    sort(ends.begin(), ends.end());
    int rooms = 0, maxRooms = 0, j = 0;
    for (int i = 0; i < n; i++) {
        while (j < n && ends[j] <= starts[i]) { rooms--; j++; }
        rooms++;
        maxRooms = max(maxRooms, rooms);
    }
    return maxRooms;
}

int main() {
    vector<pair<int,int>> iv = {{30,75},{0,50},{60,150}};
    cout << minRooms(iv) << "\n";
    return 0;
}
