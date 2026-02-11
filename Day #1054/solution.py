# Day 1054: Dual-instance singleton. Holds two lazily-created instances and an
# alternating counter; getInstance() returns instance[count % 2], then bumps the
# counter. Time O(1) per call, Space O(1).


class DualSingleton:
    _instances = None
    _count = 0

    def __init__(self, id):
        self.id = id

    @classmethod
    def get_instance(cls):
        if cls._instances is None:
            cls._instances = [cls(1), cls(2)]
        inst = cls._instances[cls._count % 2]
        cls._count += 1
        return inst


if __name__ == "__main__":
    for call in range(6):
        kind = "first" if call % 2 == 0 else "second"
        print(f"call {call} ({'even' if call % 2 == 0 else 'odd'}) -> "
              f"{kind} instance (id={DualSingleton.get_instance().id})")
