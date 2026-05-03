// Pythagorean triplet: square all, sort, for each largest c^2 two-pointer for a^2+b^2.
// Time: O(n^2), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

bool hasTriplet(vector<long long> arr) {
    for (auto& x : arr) x = x * x;
    sort(arr.begin(), arr.end());
    int n = arr.size();
    for (int c = n - 1; c >= 2; --c) {
        int l = 0, r = c - 1;
        while (l < r) {
            long long s = arr[l] + arr[r];
            if (s == arr[c]) return true;
            else if (s < arr[c]) ++l;
            else --r;
        }
    }
    return false;
}

int main() {
    cout << (hasTriplet({3, 1, 4, 6, 5}) ? "True" : "False") << "\n";
    cout << (hasTriplet({10, 4, 6, 12, 5}) ? "True" : "False") << "\n";
    return 0;
}
