// Peekable iterator wrapper: cache one element ahead. peek()/next()/hasNext() O(1) time, O(1) space.
#include <iostream>
#include <vector>

class Peekable {
    std::vector<int> data;
    size_t idx = 0;
public:
    Peekable(const std::vector<int>& v) : data(v) {}
    bool hasNext() const { return idx < data.size(); }
    int peek() const { return data[idx]; }      // does not advance
    int next() { return data[idx++]; }          // advances
};

int main() {
    Peekable p({1, 2, 3});
    std::cout << "peek()    -> " << p.peek() << "\n";
    std::cout << "next()    -> " << p.next() << "\n";
    std::cout << "peek()    -> " << p.peek() << "\n";
    std::cout << "hasNext() -> " << (p.hasNext() ? "true" : "false") << "\n";
    std::cout << "next()    -> " << p.next() << "\n";
    std::cout << "next()    -> " << p.next() << "\n";
    std::cout << "hasNext() -> " << (p.hasNext() ? "true" : "false") << "\n";
    return 0;
}
