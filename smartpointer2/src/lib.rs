// RefCell<T>

// 予備
// Rc<T> は所有者複数可・不変参照のみ
// Box<T> は単独所有者・不変か可変かはコンパイル時に精査・強制
//
// RefCell<T> は可変か不変か、実行時に精査される。（ unsafe な使用
// 異なるスコープ内で不変として・可変としてそれぞれふるまえる
// 同じスコープ内で２回可変参照するみたいなのはやはりできない

trait Messenger {
    fn send(&self, msg: &str);
}

struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
        fn new(messenger: &T, max: usize) -> LimitTracker<T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
                self.messenger.send("75~90");
            } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
                self.messenger.send("90~");
            } else if percentage_of_max >= 1.0 {
                self.messenger.send("error");
            }
        }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell; // 読み込み必要

    struct MockMessenger {
        // 不変・可変それぞれで借用しないとならいので、Vec<> を RefCell<> で包む
        sent_messeges: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            // RefCell::new() で初期化する
            MockMessenger { sent_messeges: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // 可変で借りる場合は .borror_mut() => RefMut<T> スマポ型を返してくれる
            //
            // あくまでも同じスコープ内で可変は１回、不変と可変の混同は不可、なのは同じ
            // ただスコープごとに設定ができる、というやつ
            let mut one_borrow = self.sent_messeges.borrow_mut();
            // let mut two_borrow = self.sent_messeges.borrow_mut(); (実行時エラー / already borrowed
            one_borrow.push(String::from(message));
        }
    }

    #[test]
    fn teste() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(75);

        // 不変で借りる場合は .borrow() => Ref<T> スマポ型を返してくれる
        // 同一スコープ、不変はいくつでも可・可変との混在不可なのはいつもと同じ
        assert_eq!(mock_messenger.sent_messeges.borrow().len(), 1);
    }
}





use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
use List::{Cons, Nil};

#[cfg(test)]
mod test2 {
    use super::*;

    #[test]
    fn tetete() {
        // RefCell を Rc で包むことによって、可変参照に複数の所有者を持たせられる（あるある使用法の様
        let value = Rc::new( RefCell::new(5) );

        let a = Rc::new( Cons(Rc::clone(&value), Rc::new(Nil) ) );

        let b = Rc::new( Cons(Rc::new(RefCell::new(6)), Rc::clone(&a)));
        let c = Rc::new( Cons(Rc::new(RefCell::new(10)), Rc::clone(&a)));

        *value.borrow_mut() += 10; // Rc<> の参照はずしで、中のRefCellを読む。そしてborrow_mut で可変借り

        // &value, &a の部分がすべて 15 になる
        println!("a after: {:?}", a);
        println!("b after: {:?}", b);
        println!("c after: {:?}", c);
    }
}
