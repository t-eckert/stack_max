mod stack_max;
use stack_max::StackMax;

fn main() {
    let mut stack_max = StackMax::new();
    stack_max.push(9);
    assert_eq!(&9, stack_max.peek().unwrap());
    assert_eq!(&9, stack_max.get_max().unwrap());

    stack_max.push(4);
    assert_eq!(&4, stack_max.peek().unwrap());
    assert_eq!(&9, stack_max.get_max().unwrap());

    stack_max.push(14);
    assert_eq!(&14, stack_max.peek().unwrap());
    assert_eq!(&14, stack_max.get_max().unwrap());
    assert_eq!(3, stack_max.len());

    stack_max.pop();
    assert_eq!(&4, stack_max.peek().unwrap());
    assert_eq!(&9, stack_max.get_max().unwrap());

    stack_max.pop();
    assert_eq!(&9, stack_max.peek().unwrap());
    assert_eq!(&9, stack_max.get_max().unwrap());
}
