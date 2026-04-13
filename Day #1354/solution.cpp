// Minimum jumps to reach end. Greedy: track current reach, farthest reach, jumps. O(N) time, O(1) space.
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
    cout << minJumps(a) << endl;
    return 0;
}
