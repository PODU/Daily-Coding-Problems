// Three stacks in one array via fixed equal regions, each with its own top. O(1) push/pop.
#include <bits/stdc++.h>
using namespace std;

class ThreeStacks {
    int cap;                 // capacity per stack
    vector<int> arr;
    int top[3];
public:
    ThreeStacks(int perStack = 3) : cap(perStack), arr(3 * perStack) {
        top[0] = top[1] = top[2] = 0;
    }
    void push(int item, int sn) {
        if (top[sn] >= cap) throw overflow_error("stack full");
        arr[sn * cap + top[sn]++] = item;
    }
    int pop(int sn) {
        if (top[sn] <= 0) throw underflow_error("stack empty");
        return arr[sn * cap + (--top[sn])];
    }
};

int main() {
    ThreeStacks s(3);
    s.push(1, 0); s.push(2, 0);
    s.push(10, 1); s.push(20, 1);
    s.push(100, 2);
    cout << "stack0 pop: " << s.pop(0) << "\n";
    cout << "stack1 pop: " << s.pop(1) << "\n";
    cout << "stack2 pop: " << s.pop(2) << "\n";
    cout << "stack0 pop: " << s.pop(0) << "\n";
    return 0;
}
