fn main() {
    let mut post = obj::Post::new();

    post.add_text("ate salad");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("ate salad", post.content());
}
