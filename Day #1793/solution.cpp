// Collect positions p_i of people, set q_i = p_i - i, answer = sum |q_i - median(q)|.
// Time O(n), Space O(m).
#include <iostream>
#include <vector>
#include <algorithm>
#include <cstdlib>
using namespace std;

long long minCost(const vector<int>& seats) {
    vector<int> q;
    int i = 0;
    for (int j = 0; j < (int)seats.size(); ++j)
        if (seats[j] == 1) q.push_back(j - i++);
    if (q.empty()) return 0;
    int med = q[q.size() / 2];
    long long total = 0;
    for (int v : q) total += abs(v - med);
    return total;
}

int main() {
    vector<int> seats = {0, 1, 1, 0, 1, 0, 0, 0, 1};
    cout << minCost(seats) << endl; // expected 5
    return 0;
}
