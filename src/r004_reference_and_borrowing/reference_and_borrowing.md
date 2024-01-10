# Borrowing
```Rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_len(&s1);
    println!("the len of {} is {}", s1, len);
}

fn calculate_len(s: &String) -> usize {
    s.len()
}
```
&操作是引用，*操作是解引用。
在calculate_len函数中，s是一个字符串引用。构建引用的动作叫做借用。引用默认都是不可变的，不能通过一个不可变引用去修改变量的值。

# Mutable References
可变引用，当变量和引用都用mut修饰时，变量是可变的，引用也是可变的。这时，可以通过可变引用修改变量的值。
```Rust
fn main() {
    let mut s = String::from("hello");
    change_string(&mut s);
    println!("s: {}", s);
}

fn change_string(s: &mut String) {
    s.push_str(" world!");
}
```

`error[E0499]`: cannot borrow `s` as mutable more than once at a time
任何一个作用域内，只允许有一个可变引用存在。
```Rust
fn main() {
    let mut s = String::from("hello");

    let s1 = &mut s;
    let s2 = &mut s;

    println!("s1: {}, s2: {}", s1, s2);
}
```

`error[E0502]`: cannot borrow `s` as mutable because it is also borrowed as immutable
可变引用和不可变引用不能同时存在
```Rust
fn main() {
    let mut s = String::from("hello");

    let s1 = &s;
    let s2 = &mut s;

    println!("s1: {}, s2: {}", s1, s2);
}
```

一个引用的作用范围是，从引用第一次声明开始，到最后一次使用结束。即，如果一个引用已经结束了作用范围，那么之后可以创建新的可变引用。
```Rust
fn main() {
    let mut s = String::from("hello");

    let s1 = &s;
    println!("s1: {}", s1);

    let s2 = &mut s;
    change_string(s2);
    println!("s2: {}", s2);
}
```
# Dangling References
`error[E0106]`: missing lifetime specifier
‘野指针’，或者‘悬引用’，一个指针指向的变量值已经被释放，这在许多其他语言中会造成非法数据访问的问题。Rust在编译期间检查野指针的问题并报错。
```Rust
fn main() {
    let s = dangling();

    println!("s: {}", s);
}

fn dangling() -> &String{
    let s = String::from("hello");

    &s
}
```