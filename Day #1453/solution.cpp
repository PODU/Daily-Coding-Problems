// Day 1453: Apply a permutation P (P[i] = destination of element i) to an array
// in place by following cycles. Time O(n), Space O(1) extra (P is consumed).
#include <bits/stdc++.h>
using namespace std;

void applyPermutation(vector<string>& arr, vector<int>& P) {
    for (int i = 0; i < (int)arr.size(); i++) {
        while (P[i] != i) {
            swap(arr[i], arr[P[i]]);
            swap(P[i], P[P[i]]);
        }
    }
}

int main() {
    vector<string> arr = {"a", "b", "c"};
    vector<int> P = {2, 1, 0};
    applyPermutation(arr, P);
    cout << "[";
    for (size_t i = 0; i < arr.size(); i++)
        cout << "\"" << arr[i] << "\"" << (i + 1 < arr.size() ? ", " : "");
    cout << "]\n"; // ["c", "b", "a"]
    return 0;
}
