# Day 120: Dual singleton. Two fixed instances; odd call -> 2nd, even call -> 1st.
# getInstance() is O(1); instances created once and cached on the class.
class DualSingleton:
    _first = None
    _second = None
    _count = 0

    def __init__(self, name):
        self.name = name

    @classmethod
    def get_instance(cls):
        if cls._first is None:
            cls._first = DualSingleton("first")
            cls._second = DualSingleton("second")
        cls._count += 1  # 1-based call number
        return cls._first if cls._count % 2 == 0 else cls._second  # even->first, odd->second


if __name__ == "__main__":
    for _ in range(4):
        print(DualSingleton.get_instance().name)  # second, first, second, first
