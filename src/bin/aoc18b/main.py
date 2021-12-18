from __future__ import annotations

from dataclasses import dataclass
from copy import deepcopy


@dataclass
class Pair:
    left: Pair | None = None
    right: Pair | None = None
    value: int | None = None
    parent: Pair | None = None

    def is_leaf(self):
        if self.value is not None:
            assert self.left is None and self.right is None
            return True
        return False

    def __str__(self):
        if self.is_leaf():
            return str(self.value)
        else:
            return f"[{self.left},{self.right}]"

    def __iter__(self):
        if self.is_leaf():
            yield self
        else:
            for v in self.left:
                yield v
            for v in self.right:
                yield v

    def __reversed__(self):
        if self.is_leaf():
            yield self
        else:
            for v in reversed(self.right):
                yield v
            for v in reversed(self.left):
                yield v

    def get_depth(self):
        p = self
        c = 0
        while p.parent is not None:
            c += 1
            p = p.parent
        return c

    def next_left(self):
        if self.parent is None:
            return None
        if self is self.parent.right:
            p = self.parent.left
            while not p.is_leaf():
                p = p.right
            return p
        return self.parent.next_left()

    def next_right(self):
        if self.parent is None:
            return None
        if self is self.parent.left:
            p = self.parent.right
            while not p.is_leaf():
                p = p.left
            return p
        return self.parent.next_right()

    def explode(self):
        assert self.parent is not None
        assert not self.is_leaf(), self
        assert self.left.is_leaf(), self
        assert self.right.is_leaf(), self
        left = self.next_left()
        if left is not None:
            left.value += self.left.value
        right = self.next_right()
        if right is not None:
            right.value += self.right.value
        self.left = None
        self.right = None
        self.value = 0

    def split(self):
        assert self.is_leaf(), self
        self.left = Pair(value=self.value // 2, parent=self)
        self.right = Pair(value=(self.value + 1) // 2, parent=self)
        self.value = None

    def __add__(self, other):
        assert self.parent is None and other.parent is None
        p = Pair(self, other)
        self.parent = p
        other.parent = p
        return p.reduce()

    def reduce(self):
        while True:
            # print(f"Loop {self}")
            if not self.reduce_explode() and not self.reduce_split():
                break
        return self

    def reduce_explode(self):
        for leaf in self:
            if leaf.get_depth() >= 5:
                leaf.parent.explode()
                return True
        return False

    def reduce_split(self):
        for leaf in self:
            if leaf.value >= 10:
                leaf.split()
                return True
        return False

    def magnitude(self):
        if self.is_leaf():
            return self.value
        return self.left.magnitude() * 3 + self.right.magnitude() * 2


def read_input():
    with open("../../../inputs/18.txt") as f:
        for line in f.readlines():
            pair = Pair()
            stack = [pair]
            for c in line.strip():
                current = stack[-1]
                if c.isdigit():
                    current.value = int(c)
                elif c == "[":
                    current.left = Pair(parent=current)
                    current.right = Pair(parent=current)
                    stack.append(current.left)
                elif c == ",":
                    stack.pop()
                    stack.append(current.parent.right)
                elif c == "]":
                    stack.pop()
                else:
                    assert False, repr(c)
            yield pair


if __name__ == "__main__":
    pairs = list(read_input())
    m = 0
    for p1 in pairs:
        for p2 in pairs:
            if p1 is p2:
                continue
            m = max(m, (deepcopy(p1) + deepcopy(p2)).magnitude())
    print(m)
