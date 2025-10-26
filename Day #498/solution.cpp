// Smallest window to sort so the whole array is sorted.
// Two passes: left->right track max for right bound; right->left track min for left bound.
// Time: O(n), Space: O(1).
#include <iostream>
#include <vector>
using namespace std;

pair<int,int> windowToSort(const vector<int>& a) {
    int n = a.size();
    int left = -1, right = -1;
    int maxSoFar = a[0];
    for (int i = 1; i < n; i++) {
        if (a[i] < maxSoFar) right = i;
        else maxSoFar = a[i];
    }
    int minSoFar = a[n-1];
    for (int i = n-2; i >= 0; i--) {
        if (a[i] > minSoFar) left = i;
        else minSoFar = a[i];
    }
    return {left, right};
}

int main() {
    vector<int> a = {3, 7, 5, 6, 9};
    auto p = windowToSort(a);
    cout << "(" << p.first << ", " << p.second << ")" << '\n';
    return 0;
}
