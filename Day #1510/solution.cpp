// Three stacks sharing ONE backing array of nodes (value, prevIndex) + free list.
// Three head pointers index into the single shared list. O(1) push/pop, O(n) space.
#include <bits/stdc++.h>
using namespace std;

struct ThreeStacks {
    vector<int> val, prev;
    int head[3] = {-1, -1, -1};
    int freeHead = -1;

    int alloc(int v, int p) {
        int idx;
        if (freeHead != -1) {
            idx = freeHead;
            freeHead = prev[freeHead];
            val[idx] = v; prev[idx] = p;
        } else {
            idx = (int)val.size();
            val.push_back(v); prev.push_back(p);
        }
        return idx;
    }

    void push(int item, int s) {
        int idx = alloc(item, head[s]);
        head[s] = idx;
    }

    int pop(int s) {
        int idx = head[s];
        int v = val[idx];
        head[s] = prev[idx];
        prev[idx] = freeHead; // return node to free list
        freeHead = idx;
        return v;
    }
};

int main() {
    ThreeStacks st;
    st.push(1, 0); st.push(2, 0);
    st.push(3, 1);
    st.push(4, 2); st.push(5, 2);
    int a = st.pop(0), b = st.pop(2), c = st.pop(1), d = st.pop(0);
    cout << a << " " << b << " " << c << " " << d << "\n";
    return 0;
}
