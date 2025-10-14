// Day 428: Min cost to carve a strict pyramid (1,2,..,P,..,2,1); only lowering allowed.
// left[i]/right[i] cap each stone by distance from the ends; peak H = max min(left,right).
// A strict tent of peak H has sum H*H, so cost = sum(stones) - H*H. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<int> stones = {1, 1, 3, 3, 2, 1};
    int n = stones.size();
    vector<int> left(n), right(n);
    left[0] = min(stones[0], 1);
    for (int i = 1; i < n; i++) left[i] = min(stones[i], left[i - 1] + 1);
    right[n - 1] = min(stones[n - 1], 1);
    for (int i = n - 2; i >= 0; i--) right[i] = min(stones[i], right[i + 1] + 1);
    int H = 0, p = 0;
    for (int i = 0; i < n; i++) {
        int b = min(left[i], right[i]);
        if (b > H) { H = b; p = i; }
    }
    long long total = 0;
    for (int x : stones) total += x;
    long long cost = total - (long long)H * H;
    cout << cost << "  (resulting in [";
    for (int i = 0; i < n; i++) {
        int h = max(0, H - abs(i - p));
        cout << h;
        if (i + 1 < n) cout << ", ";
    }
    cout << "])" << endl;
    return 0;
}
