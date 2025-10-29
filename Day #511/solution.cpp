// Day 511: Minimum jumps to reach end of array (each value = max jump length).
// Greedy level-by-level reachability. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int minJumps(const vector<int>& a) {
    int n = a.size();
    if (n <= 1) return 0;
    int jumps = 0, curEnd = 0, farthest = 0;
    for (int i = 0; i < n - 1; i++) {
        farthest = max(farthest, i + a[i]);
        if (i == curEnd) {
            jumps++;
            curEnd = farthest;
            if (curEnd >= n - 1) break;
        }
    }
    return jumps;
}

int main() {
    vector<int> a = {6, 2, 4, 0, 5, 1, 1, 4, 2, 9};
    cout << minJumps(a) << "\n";
    return 0;
}
