// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.

pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// auto trait 才可以组合配合 &dyn xxx 使用
// auto trait 是没有方法的trait, 这样才可以保证编译器预先知道大小，进行内存布局；
// 普通的trait进行组合，可能无法让编译器准确的进行内存布局
trait ParentTrait: SomeTrait + OtherTrait {}
impl<T: SomeTrait + OtherTrait> ParentTrait for T {}

// YOU MAY ONLY CHANGE THE NEXT LINE
// 这个只能 auto trait 才可以进行 +， 普通 trait 不能相加
// fn some_func<T: SomeTrait + OtherTrait>(item: impl T) -> bool {
fn some_func(item: &dyn ParentTrait) -> bool {
    item.some_function() && item.other_function()
}

// 这个肯定可以的
// fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
//     item.some_function() && item.other_function()
// }

fn main() {
    // some_func(SomeStruct {});
    // some_func(OtherStruct {});
    some_func(&SomeStruct {});
    some_func(&OtherStruct {});
}
