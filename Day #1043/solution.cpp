// PeekableIterator: wrap an iterator and cache one element ahead so peek() returns
// the next value without consuming it. O(1) per op, O(1) extra space.
#include <iostream>
#include <vector>

class PeekableInterface {
    std::vector<int> data;
    size_t idx = 0;
    int cached;
    bool hasCached = false;
public:
    PeekableInterface(const std::vector<int>& v) : data(v) {}

    bool hasNext() {
        return hasCached || idx < data.size();
    }
    int next() {
        if (hasCached) { hasCached = false; return cached; }
        return data[idx++];
    }
    int peek() {
        if (!hasCached) { cached = data[idx++]; hasCached = true; }
        return cached;
    }
};

int main() {
    PeekableInterface it({1, 2, 3});
    std::cout << "peek() -> " << it.peek() << "\n";
    std::cout << "next() -> " << it.next() << "\n";
    std::cout << "peek() -> " << it.peek() << "\n";
    std::cout << "next() -> " << it.next() << "\n";
    std::cout << "next() -> " << it.next() << "\n";
    std::cout << "hasNext() -> " << (it.hasNext() ? "true" : "false") << "\n";
    return 0;
}
