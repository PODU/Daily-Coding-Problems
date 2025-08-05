// Count attacking bishop pairs: group by diagonals d1=row+col, d2=row-col; sum k*(k-1)/2. Time O(N), Space O(N).
#include <iostream>
#include <vector>
#include <unordered_map>
using namespace std;

long long countAttackingPairs(const vector<pair<int,int>>& bishops) {
    unordered_map<int,long long> diag1, diag2;
    for (auto& b : bishops) {
        diag1[b.first + b.second]++;
        diag2[b.first - b.second]++;
    }
    long long pairs = 0;
    for (auto& kv : diag1) pairs += kv.second * (kv.second - 1) / 2;
    for (auto& kv : diag2) pairs += kv.second * (kv.second - 1) / 2;
    return pairs;
}

int main() {
    vector<pair<int,int>> bishops = {{0,0},{1,2},{2,2},{4,0}};
    cout << countAttackingPairs(bishops) << endl;
    return 0;
}
