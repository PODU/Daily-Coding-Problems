// Wrap an iterator and buffer one element for peek(). next/hasNext/peek all O(1).
// Time O(1) per op; Space O(1).
#include <iostream>
#include <vector>
using namespace std;

// Simple underlying iterator over a vector.
struct Iterator {
    vector<int> data;
    size_t idx = 0;
    Iterator(vector<int> d) : data(move(d)) {}
    int next() { return data[idx++]; }
    bool hasNext() { return idx < data.size(); }
};

class PeekableInterface {
    Iterator it;
    int buffer;
    bool buffered = false;
public:
    PeekableInterface(Iterator iter) : it(move(iter)) {}
    int peek() {
        if (!buffered) { buffer = it.next(); buffered = true; }
        return buffer;
    }
    int next() {
        if (buffered) { buffered = false; return buffer; }
        return it.next();
    }
    bool hasNext() { return buffered || it.hasNext(); }
};

int main() {
    PeekableInterface p(Iterator({1, 2, 3}));
    // Evaluate in separate statements to guarantee left-to-right order.
    cout << "peek=" << p.peek();
    cout << " next=" << p.next();
    cout << " peek=" << p.peek();
    cout << " next=" << p.next();
    cout << " next=" << p.next();
    cout << " hasNext=" << (p.hasNext() ? "true" : "false") << endl;
    // peek=1 next=1 peek=2 next=2 next=3 hasNext=false
    return 0;
}
