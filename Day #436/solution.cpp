// Day 436: Three stacks backed by one list using node-based singly-linked
// indices + a free list. push/pop are O(1) time, O(n) space overall.
#include <bits/stdc++.h>
using namespace std;

class ThreeStacks {
    struct Node { int val; int prev; };
    vector<Node> data;      // single backing list
    int tops[3];            // head index of each stack
    vector<int> freeList;   // recycled slots
public:
    ThreeStacks() { tops[0] = tops[1] = tops[2] = -1; }

    void push(int item, int s) {
        int idx;
        if (!freeList.empty()) { idx = freeList.back(); freeList.pop_back(); data[idx] = {item, tops[s]}; }
        else { idx = (int)data.size(); data.push_back({item, tops[s]}); }
        tops[s] = idx;
    }

    int pop(int s) {
        if (tops[s] == -1) throw runtime_error("stack " + to_string(s) + " is empty");
        int idx = tops[s];
        int v = data[idx].val;
        tops[s] = data[idx].prev;
        freeList.push_back(idx);
        return v;
    }
};

int main() {
    ThreeStacks st;
    st.push(1, 0); st.push(2, 0);
    st.push(10, 1);
    st.push(100, 2); st.push(200, 2);
    cout << st.pop(0) << "\n";   // 2
    cout << st.pop(0) << "\n";   // 1
    cout << st.pop(1) << "\n";   // 10
    cout << st.pop(2) << "\n";   // 200
    cout << st.pop(2) << "\n";   // 100
    return 0;
}
