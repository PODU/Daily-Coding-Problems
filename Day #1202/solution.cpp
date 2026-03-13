// Day 1202: Minimum swaps so couples sit side by side.
// Greedy: for each even seat, swap partner of its occupant into the next seat. Time O(N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

int minSwaps(vector<int> row) {
    int n = row.size();
    vector<int> pos(n);
    for (int i = 0; i < n; i++) pos[row[i]] = i;
    int swaps = 0;
    for (int i = 0; i < n; i += 2) {
        int partner = row[i] ^ 1;          // couples are (0,1),(2,3),...
        if (row[i + 1] != partner) {
            int j = pos[partner];
            swap(pos[row[i + 1]], pos[row[j]]);
            swap(row[i + 1], row[j]);
            swaps++;
        }
    }
    return swaps;
}

int main() {
    // N=2 couples (0,1) and (2,3); row [0,2,1,3] needs 1 swap.
    cout << minSwaps({0, 2, 1, 3}) << "\n"; // 1
    return 0;
}
