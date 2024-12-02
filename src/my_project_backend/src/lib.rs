use std::cell::RefCell;

thread_local! {
    static COUNTER: RefCell<String> = RefCell::new(String::new());
}

#[ic_cdk_macros::query]
fn get() -> String {
    let a = String::from("123");
    let b = String::from("3121");
    COUNTER.with(|counter| (*counter.borrow()).clone())
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
