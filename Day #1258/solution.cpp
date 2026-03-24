// Peeking iterator: buffer one element ahead. peek/next/hasNext all O(1) time, O(1) extra space.
#include <iostream>
#include <vector>
using namespace std;

class Iterator {
    vector<int> data;
    size_t idx = 0;
public:
    Iterator(const vector<int>& v) : data(v) {}
    int next() { return data[idx++]; }
    bool hasNext() { return idx < data.size(); }
};

class PeekableInterface {
    Iterator it;
    int buffer = 0;
    bool hasBuffer = false;
public:
    PeekableInterface(const Iterator& iterator) : it(iterator) {}
    int peek() {
        if (!hasBuffer) { buffer = it.next(); hasBuffer = true; }
        return buffer;
    }
    int next() {
        if (hasBuffer) { hasBuffer = false; return buffer; }
        return it.next();
    }
    bool hasNext() { return hasBuffer || it.hasNext(); }
};

int main() {
    PeekableInterface p(Iterator({1, 2, 3}));
    cout << p.peek() << "\n";
    cout << p.next() << "\n";
    cout << p.next() << "\n";
    cout << p.peek() << "\n";
    cout << (p.hasNext() ? "true" : "false") << "\n";
    cout << p.next() << "\n";
    cout << (p.hasNext() ? "true" : "false") << "\n";
    return 0;
}
