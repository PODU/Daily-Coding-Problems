// Day 1300: Find a contiguous subarray summing to K (handles negatives).
// Prefix-sum hashmap: for each prefix p, look for p-K seen earlier. O(N) time, O(N) space.
#include <bits/stdc++.h>
using namespace std;

vector<long long> subarraySum(const vector<long long>& a, long long K) {
    unordered_map<long long, int> firstIndex; // prefix sum -> earliest index (exclusive end)
    firstIndex[0] = -1;
    long long prefix = 0;
    for (int j = 0; j < (int)a.size(); j++) {
        prefix += a[j];
        auto it = firstIndex.find(prefix - K);
        if (it != firstIndex.end())
            return vector<long long>(a.begin() + it->second + 1, a.begin() + j + 1);
        if (!firstIndex.count(prefix)) firstIndex[prefix] = j;
    }
    return {}; // none found
}

int main() {
    auto r = subarraySum({1, 2, 3, 4, 5}, 9);
    cout << "[";
    for (size_t i = 0; i < r.size(); i++)
        cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]" << endl; // [2, 3, 4]
    return 0;
}
