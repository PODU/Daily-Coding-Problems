// Day 1323: Min lowering cost to form a pyramid (rise by 1 to a peak, fall by 1, unit ends).
// left[i]=min(a[i],left[i-1]+1), right[i] symmetric; best peak h=max(min(left,right)); a full pyramid sums to h^2.
// Cost = sum(a) - h^2. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<int> a = {1, 1, 3, 3, 2, 1};
    int n = a.size();
    vector<int> left(n), right(n);
    for (int i = 0; i < n; i++) left[i] = min(a[i], (i ? left[i-1] : 0) + 1);
    for (int i = n - 1; i >= 0; i--) right[i] = min(a[i], (i < n-1 ? right[i+1] : 0) + 1);

    int h = 0, peak = 0;
    for (int i = 0; i < n; i++) { int hi = min(left[i], right[i]); if (hi > h) { h = hi; peak = i; } }

    vector<int> target(n, 0);
    for (int i = 0; i < n; i++) {
        int d = abs(i - peak);
        if (d < h) target[i] = h - d;
    }
    long total = accumulate(a.begin(), a.end(), 0L);
    long cost = total - (long)h * h;

    cout << cost << endl; // 2
    cout << "[";
    for (int i = 0; i < n; i++) cout << target[i] << (i + 1 < n ? ", " : "");
    cout << "]" << endl;   // [0, 1, 2, 3, 2, 1]
    return 0;
}
