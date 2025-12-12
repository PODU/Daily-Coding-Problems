// Quack (push/pop left, pull right) via three stacks.
// On underflow of one side, rebalance by moving half from the other side using a temp stack.
// Amortized O(1) per operation, O(1) extra memory beyond the three stacks.
#include <bits/stdc++.h>
using namespace std;

struct Quack {
    vector<int> front, back, temp; // front.top = leftmost, back.top = rightmost

    void push(int x){ front.push_back(x); }

    void refillFront(){            // front empty, pull half of list from back
        int n = back.size();
        int leftCount = (n + 1) / 2, rightCount = n - leftCount;
        for(int i=0;i<rightCount;i++){ temp.push_back(back.back()); back.pop_back(); }
        for(int i=0;i<leftCount;i++){ front.push_back(back.back()); back.pop_back(); }
        while(!temp.empty()){ back.push_back(temp.back()); temp.pop_back(); }
    }
    void refillBack(){            // back empty, pull half of list from front
        int n = front.size();
        int rightCount = (n + 1) / 2, leftCount = n - rightCount;
        for(int i=0;i<leftCount;i++){ temp.push_back(front.back()); front.pop_back(); }
        for(int i=0;i<rightCount;i++){ back.push_back(front.back()); front.pop_back(); }
        while(!temp.empty()){ front.push_back(temp.back()); temp.pop_back(); }
    }

    int pop(){                    // remove from left
        if(front.empty()) refillFront();
        if(front.empty()) throw runtime_error("empty");
        int v = front.back(); front.pop_back(); return v;
    }
    int pull(){                   // remove from right
        if(back.empty()) refillBack();
        if(back.empty()) throw runtime_error("empty");
        int v = back.back(); back.pop_back(); return v;
    }
};

int main(){
    Quack q;
    q.push(1); q.push(2); q.push(3);
    cout << "pop: "  << q.pop()  << "\n"; // 3
    cout << "pull: " << q.pull() << "\n"; // 1
    q.push(4);
    cout << "pull: " << q.pull() << "\n"; // 2
    cout << "pop: "  << q.pop()  << "\n"; // 4
    return 0;
}
