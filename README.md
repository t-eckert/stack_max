# StackMax

A learning project in Rust. An instance of `StackMax` works as a stack normally does, giving a user access to `push()`, `peek()`, and `pop()` functions. This stack additionally has the ability to return the maximum value in the stack.

This is accomplished by using two `Vec<i32>` to handle state inside of `StackMax`. One of these `Vec<i32>`, called `numbers`, is a stack of all numbers pushed onto `StackMax`. The other, called `maximums` (though should it be `maxima`?), is a stack of only the maximum numbers pushed to the `numbers` stack.

When a number is pushed to `StackMax`, it is added to the `numbers` stack then compared to the number at the top of the `maximums` stack. If the number being pushed is greater than the value at the top of the `maximums` stack or if the stack is empty, the number is pushed to the `maximums` stack.

When a number is popped from `StackMax`, it is removed from the `numbers` stack and compared to the number at the top of the `maximums` stack. If they are equal, the number at the top of the `maximums` stack is also removed.

## Demo

The StackMax object is initialized.

``` text
StackMax {
    numbers: 
    maximums:
}
```

4 is pushed to StackMax.

``` text
StackMax {
    numbers: 4
    maximums: 4
}
```

2 is pushed to StackMax.

``` text
StackMax {
    numbers: 4, 2
    maximums: 4
}
```

6 is pushed to StackMax.

``` text
StackMax {
    numbers: 4, 2, 6
    maximums: 4, 6
}
```

6 is popped from StackMax.

``` text
StackMax {
    numbers: 4, 2
    maximums: 4
}
```

2 is popped from StackMax.

``` text
StackMax {
    numbers: 4
    maximums: 4
}
```

4 is popped from StackMax.

``` text
StackMax {
    numbers: 
    maximums:
}
```