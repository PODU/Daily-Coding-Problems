// Fixed point (arr[i]==i) in sorted distinct array via binary search.
// Time O(log n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

// Returns index i where arr[i]==i, or -1 if none.
int fixedPoint(const vector<int>& arr) {
    int lo = 0, hi = (int)arr.size() - 1;
    while (lo <= hi) {
        int mid = lo + (hi - lo) / 2;
        if (arr[mid] == mid) return mid;
        else if (arr[mid] < mid) lo = mid + 1;
        else hi = mid - 1;
    }
    return -1;
}

void run(const vector<int>& arr) {
    int r = fixedPoint(arr);
    if (r == -1) cout << "False\n";
    else cout << r << '\n';
}

int main() {
    run({-6, 0, 2, 40});
    run({1, 5, 7, 8});
    return 0;
}
