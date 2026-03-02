// Day 1141: Min cost to pack people (remove gaps).
// Transform p_i -> p_i - i, answer = sum |q_i - median(q)|. Time O(n log n), Space O(m).
#include <bits/stdc++.h>
using namespace std;

long long minCost(const vector<int>& seats) {
    vector<long long> q;
    int idx = 0;
    for (int i = 0; i < (int)seats.size(); ++i)
        if (seats[i]) q.push_back(i - idx++);
    if (q.empty()) return 0;
    nth_element(q.begin(), q.begin() + q.size() / 2, q.end());
    long long med = q[q.size() / 2], cost = 0;
    for (long long v : q) cost += llabs(v - med);
    return cost;
}

int main() {
    vector<int> seats = {0, 1, 1, 0, 1, 0, 0, 0, 1};
    cout << minCost(seats) << "\n"; // 5
    return 0;
}
