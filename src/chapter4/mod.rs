use std::{marker::PhantomPinned, pin::Pin};

struct Test {
    a: String,
    b: *const String,
}

impl Test {
    fn new(text: &str) -> Self {
        Test {
            a: String::from(text),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        assert!(!self.b.is_null(), "b is null, call init you dummy");
        unsafe { &*(self.b) }
    }
}

pub fn _4_1() {
    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();

    println!("a: {}, b: {}", test1.a(), test1.b());
    std::mem::swap(&mut test1, &mut test2);
    test1.a = "I've totally changed now!".to_string();
    println!("a: {}, b: {}", test2.a(), test2.b());
}

#[derive(Debug)]
struct PinnedTest {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl PinnedTest {
    fn new(text: &str) -> Pin<Box<Self>> {
        let t = PinnedTest {
            a: String::from(text),
            b: std::ptr::null(),
            _marker: PhantomPinned, //this makes it !Unpin
        };
        let mut boxed = Box::pin(t);
        let self_ptr: *const String = &boxed.a;
        unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };
        boxed
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        unsafe { &*(self.b) }
    }
}

pub fn _4_2() {
    let mut test1 = PinnedTest::new("test1");
    let test2 = PinnedTest::new("test2");

    println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
    //following is not allowed as it is !Unpin
    //std::mem::swap(test1.get_mut(), test2.get_mut());
    unsafe { test1.as_mut().get_unchecked_mut().a = "I've totally changed now!".to_string(); }
    println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
    println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());
}