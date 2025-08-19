// Three stacks in one list via interleaved indexing: logical pos p of stack s -> physical p*3+s.
// push/pop O(1) amortized. Space O(total elements). Single backing list.
#include <bits/stdc++.h>
using namespace std;

class Stack {
    vector<int> list;          // single backing list
    int sizes[3] = {0, 0, 0};  // logical height of each stack
public:
    void push(int item, int stack_number) {
        int phys = sizes[stack_number] * 3 + stack_number;
        if (phys >= (int)list.size()) list.resize(phys + 1, 0);
        list[phys] = item;
        sizes[stack_number]++;
    }
    int pop(int stack_number) {
        if (sizes[stack_number] == 0) throw runtime_error("empty stack");
        sizes[stack_number]--;
        int phys = sizes[stack_number] * 3 + stack_number;
        return list[phys];
    }
};

int main() {
    Stack s;
    s.push(1, 0); s.push(2, 0);
    s.push(10, 1);
    s.push(100, 2); s.push(200, 2);
    // Separate statements guarantee left-to-right pop order.
    cout << s.pop(0);
    cout << " " << s.pop(2);
    cout << " " << s.pop(1);
    cout << " " << s.pop(2);
    cout << " " << s.pop(0) << endl;
    // 2 200 10 100 1
    return 0;
}
