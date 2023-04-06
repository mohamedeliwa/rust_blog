use blog::Post;

pub mod rust_blog;

fn main() {
    // implementing the state pattern of OOP
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    post.add_text("This content shouldn't be added");
    assert_eq!("I ate a salad for lunch today", post.content());
    println!("{}", post.content());

    // implementing the state in the posts types
    let mut post = rust_blog::Post::new();
    post.add_text("I ate a salad for lunch today");
    // assert_eq!("", rpost.content());

    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
