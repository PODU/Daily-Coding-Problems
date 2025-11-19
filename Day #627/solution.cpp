// Peekable iterator: cache one element ahead so peek() returns next without advancing.
// next/peek/hasNext all O(1).
#include <iostream>
#include <vector>

class Peekable {
    std::vector<int> data;
    size_t idx;
public:
    Peekable(const std::vector<int>& v) : data(v), idx(0) {}
    bool hasNext() const { return idx < data.size(); }
    int peek() const { return data[idx]; }      // returns next without advancing
    int next() { return data[idx++]; }
};

int main() {
    Peekable it({1, 2, 3, 4});
    std::cout << it.peek() << "\n";  // 1
    std::cout << it.next() << "\n";  // 1
    std::cout << it.next() << "\n";  // 2
    std::cout << it.peek() << "\n";  // 3
    std::cout << it.next() << "\n";  // 3
    std::cout << (it.hasNext() ? "true" : "false") << "\n"; // true
    std::cout << it.next() << "\n";  // 4
    std::cout << (it.hasNext() ? "true" : "false") << "\n"; // false
    return 0;
}
