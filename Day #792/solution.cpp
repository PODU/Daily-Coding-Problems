// Pyramid: left[i]=min(h,left[i-1]+1), right[i]=min(h,right[i+1]+1), cap=min(left,right).
// Peak P=max(cap); target descends from P both sides. cost=sum(h)-sum(target). O(n) time/space.
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<int> h = {1, 1, 3, 3, 2, 1};
    int n = h.size();
    vector<int> left(n), right(n);
    left[0] = min(h[0], 1);
    for (int i = 1; i < n; i++) left[i] = min(h[i], left[i-1] + 1);
    right[n-1] = min(h[n-1], 1);
    for (int i = n - 2; i >= 0; i--) right[i] = min(h[i], right[i+1] + 1);
    vector<int> cap(n);
    int P = 0, k = 0;
    for (int i = 0; i < n; i++) {
        cap[i] = min(left[i], right[i]);
        if (cap[i] > P) { P = cap[i]; k = i; }
    }
    vector<int> target(n, 0);
    target[k] = P;
    for (int j = 1; k - j >= 0; j++) target[k-j] = max(0, P - j);
    for (int j = 1; k + j < n; j++) target[k+j] = max(0, P - j);
    int cost = 0;
    for (int i = 0; i < n; i++) cost += h[i] - target[i];
    cout << cost << " (resulting in [";
    for (int i = 0; i < n; i++) {
        cout << target[i];
        if (i + 1 < n) cout << ", ";
    }
    cout << "])" << endl;
    return 0;
}
