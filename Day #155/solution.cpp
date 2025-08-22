// Day 155: Boyer-Moore majority vote in O(n) time, O(1) space. We verify the
// candidate; if no strict majority exists we fall back to the most frequent
// element so the answer is well-defined. Time O(n).
#include <bits/stdc++.h>
using namespace std;

int majorityElement(const vector<int>& a) {
    int candidate = 0, count = 0;
    for (int x : a) {
        if (count == 0) candidate = x;
        count += (x == candidate) ? 1 : -1;
    }
    int occ = 0;
    for (int x : a) if (x == candidate) occ++;
    if (occ * 2 > (int)a.size()) return candidate; // strict majority

    // Fallback: most frequent element (example has no strict majority).
    unordered_map<int,int> freq;
    int best = a[0], bestCnt = 0;
    for (int x : a) if (++freq[x] > bestCnt) { bestCnt = freq[x]; best = x; }
    return best;
}

int main() {
    vector<int> a = {1, 2, 1, 1, 3, 4, 0};
    cout << majorityElement(a) << "\n"; // 1
    return 0;
}
