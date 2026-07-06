// Count attacking bishop pairs: group by diagonal (r-c) and anti-diagonal (r+c), sum c*(c-1)/2.
// Time: O(N), Space: O(N).
#include <bits/stdc++.h>
using namespace std;

int main() {
    int M = 5;
    vector<pair<int,int>> bishops = {{0,0},{1,2},{2,2},{4,0}};
    (void)M;
    map<int,long long> diag, anti;
    for (auto &b : bishops) { diag[b.first - b.second]++; anti[b.first + b.second]++; }
    long long pairs = 0;
    for (auto &kv : diag) pairs += kv.second * (kv.second - 1) / 2;
    for (auto &kv : anti) pairs += kv.second * (kv.second - 1) / 2;
    cout << pairs << "\n";
    return 0;
}
