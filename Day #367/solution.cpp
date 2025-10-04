// Day 367: Lazily merge two sorted input streams into one sorted stream.
// Peek the head of each iterator and emit the smaller; never materializes both.
// Time O(n+m), Space O(1) beyond the iterators.
#include <bits/stdc++.h>
using namespace std;

template <class It>
class MergeIterator {
    It a, aEnd, b, bEnd;
public:
    MergeIterator(It a, It aEnd, It b, It bEnd) : a(a), aEnd(aEnd), b(b), bEnd(bEnd) {}
    bool hasNext() const { return a != aEnd || b != bEnd; }
    int next() {
        if (b == bEnd || (a != aEnd && *a <= *b)) return *a++;
        return *b++;
    }
};

int main() {
    vector<int> foo = {5, 10, 15}, bar = {3, 8, 9};
    MergeIterator<vector<int>::iterator> it(foo.begin(), foo.end(), bar.begin(), bar.end());
    while (it.hasNext()) cout << it.next() << "\n"; // 3 5 8 9 10 15
    return 0;
}
