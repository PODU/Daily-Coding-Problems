// Count attacking bishop pairs by grouping on diagonals (row-col) and anti-diagonals (row+col).
// For each group of size k, add k*(k-1)/2. Time O(N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

long long countAttackingPairs(const vector<pair<int,int>>& bishops) {
    unordered_map<int,int> diag, anti;
    for (auto& b : bishops) {
        diag[b.first - b.second]++;
        anti[b.first + b.second]++;
    }
    long long pairs = 0;
    for (auto& kv : diag) pairs += (long long)kv.second * (kv.second - 1) / 2;
    for (auto& kv : anti) pairs += (long long)kv.second * (kv.second - 1) / 2;
    return pairs;
}

int main() {
    vector<pair<int,int>> bishops = {{0,0},{1,2},{2,2},{4,0}};
    cout << countAttackingPairs(bishops) << endl; // 2
    return 0;
}
