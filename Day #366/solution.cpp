// Day 366: Rearrange so no two adjacent chars match (else null).
// Greedy with a max-heap by frequency; always place the most frequent char that
// isn't the one just placed. Feasible iff maxFreq <= (n+1)/2. Time O(n log k).
#include <bits/stdc++.h>
using namespace std;

string reorganize(const string& s) {
    unordered_map<char,int> cnt;
    for (char c : s) cnt[c]++;
    priority_queue<pair<int,char>> pq;
    for (auto& kv : cnt) pq.push({kv.second, kv.first});
    string res;
    pair<int,char> prev = {0, 0};
    while (!pq.empty()) {
        auto cur = pq.top(); pq.pop();
        res += cur.second;
        if (prev.first > 0) pq.push(prev);
        cur.first--;
        prev = cur;
    }
    return (int)res.size() == (int)s.size() ? res : "null";
}

int main() {
    cout << reorganize("yyz") << "\n";   // yzy
    cout << reorganize("yyy") << "\n";   // null
    return 0;
}
