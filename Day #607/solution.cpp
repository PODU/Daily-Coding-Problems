// Day 607: Min total movement to seat M people contiguously in a row.
// Approach: target = median of (pos[i]-i); cost = sum |(pos[i]-i) - median|. Time O(n), Space O(M).
#include <bits/stdc++.h>
using namespace std;

long long minCost(const vector<int>& seats) {
    vector<long long> b;
    int idx = 0;
    for (int i = 0; i < (int)seats.size(); i++)
        if (seats[i] == 1) b.push_back(i - idx++);
    if (b.empty()) return 0;
    nth_element(b.begin(), b.begin() + b.size() / 2, b.end());
    long long med = b[b.size() / 2];
    long long cost = 0;
    for (long long v : b) cost += llabs(v - med);
    return cost;
}

int main() {
    vector<int> seats = {0, 1, 1, 0, 1, 0, 0, 0, 1};
    cout << minCost(seats) << "\n"; // 5
    return 0;
}
