// Day 309: Min movement to pack people with no gaps. Map p_i - i; cost minimized
// at the median of those values. O(M log M).
#include <bits/stdc++.h>
using namespace std;
long long minCost(vector<int> seats) {
    vector<long long> b;
    int idx = 0;
    for (int i = 0; i < (int)seats.size(); i++) if (seats[i]) b.push_back(i - (idx++));
    if (b.empty()) return 0;
    sort(b.begin(), b.end());
    long long med = b[b.size() / 2], cost = 0;
    for (long long x : b) cost += llabs(x - med);
    return cost;
}
int main() {
    vector<int> s = {0, 1, 1, 0, 1, 0, 0, 0, 1};
    cout << minCost(s) << "\n"; // 5
}
