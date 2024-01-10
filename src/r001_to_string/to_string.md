# Rust 中的 ToString 方法
rust中，要实现一个Value的toString方法，需要实现 std::fmt::Display，而不是直接实现 std::string::ToString。
参考：[ToString trait](https://doc.rust-lang.org/std/string/trait.ToString.html)

```rust
struct Point {
    x: i32,
    y: i32,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point {
        x: 1,
        y: 2,
    };

    assert_eq!("(1, 2)", p.to_string());
    assert_eq!("p: (1, 2)", format!("p: {}", p));
}
```
# std::fmt::Display 和 std::fmt::Debug 的区别
1. Display 是面向用户的一个trait，不能使用 derive
2. Debug 是面向程序员调试的一个trait，可以使用 derive，并且derive的Debug是不稳定的，可能随着Rust版本变化。

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point {
        x: 1,
        y: 2,
    };

    assert_eq!("Point { x: 1, y: 2 }", format!("{:?}", p));
    assert_eq!("Point { x: 1, y: 2 }", format!("{p:?}"));
    assert_eq!("pretty: Point {
    x: 1,
    y: 2,
}", format!("pretty: {:#?}", p));
    assert_eq!("pretty: Point {
    x: 1,
    y: 2,
}", format!("pretty: {p:#?}"));
}
```

# 派生的Debug
一个struct 实现了derive 的 Debug，则其成员也需要实现 Debug。
```rust
struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Line {
    from: Point,
    to: Point,
}

fn main() {
    let l = Line {
        from: Point { x: 0, y: 0 },
        to: Point { x: 1, y: 1 },
    };

    assert_eq!("l: Line { from: (0, 0), to: (1, 1) }", format!("l: {l:?}"));
}
```