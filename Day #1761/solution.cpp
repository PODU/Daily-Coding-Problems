// Day 1761: Interleave first half of a stack with the reversed second half,
// in-place using ONE auxiliary queue (only push/pop/enqueue/dequeue).
// Approach: dump stack into queue (reverses), then rebuild taking alternately
// from back/front of the queue. Time O(n^2), Space O(n).
#include <bits/stdc++.h>
using namespace std;

vector<int> interleave(vector<int> stack) {
    queue<int> q;
    while (!stack.empty()) { q.push(stack.back()); stack.pop_back(); }
    bool takeBack = true;
    while (!q.empty()) {
        int v;
        if (takeBack) {
            for (size_t i = 0; i + 1 < q.size(); ++i) { q.push(q.front()); q.pop(); }
            v = q.front(); q.pop();
        } else {
            v = q.front(); q.pop();
        }
        stack.push_back(v);
        takeBack = !takeBack;
    }
    return stack;
}

void print(const vector<int>& v) {
    cout << "[";
    for (size_t i = 0; i < v.size(); ++i) cout << v[i] << (i + 1 < v.size() ? ", " : "");
    cout << "]\n";
}

int main() {
    print(interleave({1, 2, 3, 4, 5}));
    print(interleave({1, 2, 3, 4}));
    return 0;
}
