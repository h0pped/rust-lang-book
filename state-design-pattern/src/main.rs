use state_design_pattern::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("Some first text");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();

    assert_eq!("Some first text", post.content());
}
