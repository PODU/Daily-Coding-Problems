// Day 1211: Interleave first half of a stack with the reversed second half, using one queue.
// Load stack bottom->top into the queue, then alternately push front/back (back via rotation). Time O(n^2), Space O(n).
#include <bits/stdc++.h>
using namespace std;

void interleave(stack<int>& st) {
    queue<int> q;
    int n = 0;
    while (!st.empty()) { q.push(st.top()); st.pop(); n++; } // front = old top
    for (int i = 0; i < n; i++) { st.push(q.front()); q.pop(); } // top = old bottom
    for (int i = 0; i < n; i++) { q.push(st.top()); st.pop(); }   // front = bottom
    int remaining = n;
    bool takeFront = true;
    while (remaining > 0) {
        if (takeFront) { st.push(q.front()); q.pop(); }
        else {
            for (int i = 0; i < remaining - 1; i++) { q.push(q.front()); q.pop(); }
            st.push(q.front()); q.pop();
        }
        remaining--; takeFront = !takeFront;
    }
}

int main() {
    stack<int> st;
    for (int x : {1, 2, 3, 4, 5}) st.push(x); // bottom -> top
    interleave(st);
    vector<int> out;
    while (!st.empty()) { out.push_back(st.top()); st.pop(); }
    reverse(out.begin(), out.end()); // bottom -> top
    cout << "[";
    for (size_t i = 0; i < out.size(); i++) { if (i) cout << ", "; cout << out[i]; }
    cout << "]\n"; // [1, 5, 2, 4, 3]
    return 0;
}
