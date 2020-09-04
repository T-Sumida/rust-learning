#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // CustomSmartPointerをデータ`{}`とともにドロップするよ
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    {
        let b = Box::new(5);
        println!("b = {}", b);
    }
    // {
    //     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // }
    {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    {
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
    }
    {
        let m = MyBox::new(String::from("Rust"));
        hello(&(*m)[..]);
    }
    {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        }; // 俺のもの
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        }; // 別のもの
        println!("CustomSmartPointers created.");
    }
    {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created.");
        drop(c);
        // CustomSmartPointerはmainが終わる前にドロップされた
        println!("CustomSmartPointer dropped before the end of main.");
    }
    // {
    //     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    //     let b = Cons(3, Rc::clone(&a));
    //     let c = Cons(4, Rc::clone(&a));
    // }
    // {
    //     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    //     // a生成後のカウント = {}
    //     println!("count after creating a = {}", Rc::strong_count(&a));
    //     let b = Cons(3, Rc::clone(&a));
    //     // b生成後のカウント = {}
    //     println!("count after creating b = {}", Rc::strong_count(&a));
    //     {
    //         let c = Cons(4, Rc::clone(&a));
    //         // c生成後のカウント = {}
    //         println!("count after creating c = {}", Rc::strong_count(&a));
    //     }
    //     // cがスコープを抜けた後のカウント = {}
    //     println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // }
    {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        // aの最初の参照カウント = {}
        println!("a initial rc count = {}", Rc::strong_count(&a));
        // aの次の要素は = {:?}
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        // b作成後のaの参照カウント = {}
        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        // bの最初の参照カウント = {}
        println!("b initial rc count = {}", Rc::strong_count(&b));
        // bの次の要素 = {:?}
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        // aを変更後のbの参照カウント = {}
        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        // aを変更後のaの参照カウント = {}
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // 次の行のコメントを外して循環していると確認してください; スタックオーバーフローします
        // println!("a next item = {:?}", a.tail());        // aの次の要素 = {:?}
    }
}
