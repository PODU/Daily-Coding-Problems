// Day 1836: Demonstrate ad-hoc, parametric, and subtype polymorphism.
// Each demo is O(1); compile-time (overload/template) vs runtime (virtual) dispatch.
#include <bits/stdc++.h>
using namespace std;

// Ad-hoc polymorphism: one name, type-specific implementations chosen by overload resolution.
int add(int a, int b) { return a + b; }
string add(const string& a, const string& b) { return a + b; }

// Parametric polymorphism: a single generic definition valid for every type.
template <typename T> T identity(T x) { return x; }

// Subtype polymorphism: a base interface, overridden in derived types, dispatched at runtime.
struct Shape { virtual string name() const { return "shape"; } virtual ~Shape() {} };
struct Circle : Shape { string name() const override { return "circle"; } };
struct Square : Shape { string name() const override { return "square"; } };

int main() {
    cout << "Ad-hoc:     add(2,3)=" << add(2, 3)
         << ", add(\"a\",\"b\")=" << add(string("a"), string("b")) << "\n";
    cout << "Parametric: identity(7)=" << identity(7)
         << ", identity(\"hi\")=" << identity(string("hi")) << "\n";
    vector<unique_ptr<Shape>> shapes;
    shapes.push_back(make_unique<Circle>());
    shapes.push_back(make_unique<Square>());
    cout << "Subtype:    ";
    for (auto& s : shapes) cout << s->name() << " ";
    cout << "\n";
}
