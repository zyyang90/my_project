/// 1. integer division rounds towards 0
/// 2. what is floor, ceil, round
/// 3. for usize, ceil(a / b) = (a + b - 1) / b, floor(a/b)= a/b, round(a/b) = (a + b/2) / b
#[test]
fn why_the_output_is_different() {
    let h = -3.7_f32;
    println!("{}", h.floor());

    println!("{}", -3.7_f32.floor());
}

#[test]
fn be_careful_the_expression_precedence() {
    let h = -3.7_f32;
    println!("{}", h.floor());

    println!("{}", (-3.7_f32).floor());
}

#[test]
fn test_ceil_floor_round() {
    print_ceil_round_floor(10.0, 3.0);
    print_ceil_round_floor(11.0, 3.0);
    print_ceil_round_floor(12.0, 3.0);

    print_ceil_round_floor(-10.0, 3.0);
    print_ceil_round_floor(-11.0, 3.0);
    print_ceil_round_floor(-12.0, 3.0);
}

fn print_ceil_round_floor(numerator: f64, denominator: f64) {
    let floor = ((numerator / denominator) as f64).floor();
    let ceil = ((numerator / denominator) as f64).ceil();
    let round = ((numerator / denominator) as f64).round();
    println!("{numerator}/{denominator}, floor: {}, round: {}, ceil: {}", floor, round, ceil);
}

#[test]
fn test_usize_ceil_round_floor() {
    print_usize_ceil_round_floor(10, 3);
    print_usize_ceil_round_floor(11, 3);
    print_usize_ceil_round_floor(12, 3);

    print_isize_ceil_round_floor(-10, 3);
    print_isize_ceil_round_floor(-11, 3);
    print_isize_ceil_round_floor(-12, 3);

    print_isize_ceil_round_floor(-10, -3);
    print_isize_ceil_round_floor(-11, -3);
    print_isize_ceil_round_floor(-12, -3);
}

fn print_usize_ceil_round_floor(a: usize, b: usize) {
    let floor = a / b;
    let ceil = (a + b - 1) / b;
    let round = (a + b / 2) / b;
    println!("{a} / {b}, floor: {floor}. ceil: {ceil}. round: {round}");
}

fn print_isize_ceil_round_floor(a: isize, b: isize) {
    let floor = a / b;
    let ceil = (a + b - 1) / b;
    let round = (a + b / 2) / b;

    println!("{a} / {b}, floor: {floor}. ceil: {ceil}. round: {round}");
}