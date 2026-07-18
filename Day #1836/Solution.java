// Day 1836: Demonstrate ad-hoc, parametric, and subtype polymorphism.
// Each demo is O(1); compile-time (overload/generic) vs runtime (override) dispatch.
import java.util.*;

public class Solution {
    // Ad-hoc polymorphism: same name, type-specific overloads.
    static int add(int a, int b) { return a + b; }
    static String add(String a, String b) { return a + b; }

    // Parametric polymorphism: one generic definition for any type.
    static <T> T identity(T x) { return x; }

    // Subtype polymorphism: base type with overriding subtypes, dispatched at runtime.
    static class Shape { String name() { return "shape"; } }
    static class Circle extends Shape { String name() { return "circle"; } }
    static class Square extends Shape { String name() { return "square"; } }

    public static void main(String[] args) {
        System.out.println("Ad-hoc:     add(2,3)=" + add(2, 3) + ", add(\"a\",\"b\")=" + add("a", "b"));
        System.out.println("Parametric: identity(7)=" + identity(7) + ", identity(\"hi\")=" + identity("hi"));
        List<Shape> shapes = Arrays.asList(new Circle(), new Square());
        StringBuilder sb = new StringBuilder("Subtype:    ");
        for (Shape s : shapes) sb.append(s.name()).append(" ");
        System.out.println(sb.toString().trim());
    }
}
