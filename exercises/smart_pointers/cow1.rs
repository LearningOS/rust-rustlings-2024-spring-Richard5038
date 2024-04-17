// cow1.rs
//
// This exercise explores the Cow, or Clone-On-Write type. Cow is a
// clone-on-write smart pointer. It can enclose and provide immutable access to
// borrowed data, and clone the data lazily when mutation or ownership is
// required. The type is designed to work with general borrowed data via the
// Borrow trait.
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for ，) and Cow::Borrowed(_) at the
// TODO markers.
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.

// Cow 的设计目的是提高性能（减少复制）同时增加灵活性，因为大部分情况下，多用于读多写少的场景。利用 Cow，可以用统一，规范的形式实现，需要写的时候才做一次对象复制。

//     创建语义：Cow::Borrowed(v) 或者 Cow::Owned(v)
//     获得本体：Cow::into_owned()，得到具有 所有权的值 ，如果之前Cow是Borrowed借用状态，调用into_owned将会克隆，如果已经是Owned状态，将不会克隆
//     可变借用：Cow::to_mut()，得到一个具有所有权的值的 可变引用 ，注意在已经具有所有权的情况下，也可以调用to_mut但不会产生新的克隆，多次调用to_mut只会产生一次克隆


use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            // TODO
            Cow::Borrowed(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // We can also pass `slice` without `&` so Cow owns it directly. In this
        // case no mutation occurs and thus also no clone, but the result is
        // still owned because it was never borrowed or mutated.
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            // TODO
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // Of course this is also the case if a mutation does occur. In this
        // case the call to `to_mut()` returns a reference to the same data as
        // before.
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            // TODO
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }
}
