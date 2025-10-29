// Contiguous subarray summing to K via prefix-sum hash map. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

vector<int> subarraySum(const vector<int>& a, int K) {
    unordered_map<long long, int> seen; // prefix sum -> end index
    seen[0] = -1;
    long long pre = 0;
    for (int i = 0; i < (int)a.size(); i++) {
        pre += a[i];
        auto it = seen.find(pre - K);
        if (it != seen.end()) {
            return vector<int>(a.begin() + it->second + 1, a.begin() + i + 1);
        }
        seen.emplace(pre, i);
    }
    return {};
}

int main() {
    vector<int> r = subarraySum({1, 2, 3, 4, 5}, 9);
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]\n";
    return 0;
}
