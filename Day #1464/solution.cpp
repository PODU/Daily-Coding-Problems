// Two single elements (rest in pairs): XOR all -> a^b, split by a set bit, XOR groups.
// Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

pair<long long,long long> twoUnique(const vector<long long>& arr) {
    long long x = 0;
    for (long long v : arr) x ^= v;
    long long bit = x & (-x); // lowest set bit
    long long a = 0, b = 0;
    for (long long v : arr) {
        if (v & bit) a ^= v;
        else b ^= v;
    }
    if (a > b) swap(a, b);
    return {a, b};
}

int main() {
    vector<long long> arr = {2, 4, 6, 8, 10, 2, 6, 10};
    auto r = twoUnique(arr);
    cout << r.first << " and " << r.second << endl;
    return 0;
}
