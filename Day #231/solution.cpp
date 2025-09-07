// Reorganize string: greedily place the most frequent remaining char that differs from the last.
// Max-heap by count. Time: O(n log A), Space: O(A).
#include <bits/stdc++.h>
using namespace std;

string reorganize(const string& s) {
    unordered_map<char, int> cnt;
    for (char c : s) cnt[c]++;
    // highest count first; ties broken by smallest char for determinism
    // (negate the char so the max-heap's tie order prefers smaller chars)
    priority_queue<pair<int, int>> pq;
    for (auto& p : cnt) pq.push({p.second, -(int)p.first});
    string res;
    pair<int, int> prev = {0, 0};
    while (!pq.empty()) {
        auto cur = pq.top(); pq.pop();
        res += (char)(-cur.second);
        if (prev.first > 0) pq.push(prev);
        cur.first--;
        prev = cur;
    }
    return (int)res.size() == (int)s.size() ? res : "None";
}

int main() {
    cout << reorganize("aaabbc") << "\n"; // ababac (a valid arrangement)
    cout << reorganize("aaab") << "\n";   // None
}
