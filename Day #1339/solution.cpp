// Bishops attack along diagonals: group by (row-col) and (row+col); each group of k adds k*(k-1)/2. O(N) time, O(N) space.
#include <bits/stdc++.h>
using namespace std;

long long countAttackingPairs(int /*M*/, const vector<pair<int,int>>& bishops) {
    unordered_map<int,long long> diag, anti;
    for (auto& b : bishops) {
        diag[b.first - b.second]++;
        anti[b.first + b.second]++;
    }
    long long pairs = 0;
    for (auto& kv : diag) pairs += kv.second * (kv.second - 1) / 2;
    for (auto& kv : anti) pairs += kv.second * (kv.second - 1) / 2;
    return pairs;
}

int main() {
    int M = 5;
    vector<pair<int,int>> bishops = {{0,0},{1,2},{2,2},{4,0}};
    cout << countAttackingPairs(M, bishops) << "\n";
    return 0;
}
