use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    // 1.补充需求：添加reject方法，它可以将文章的状态从PendingReview修改为Draft。
    post.rejec();
    assert_eq!("", post.content());

    // 如果不再次请求审批，则后面逻辑会报错，证明状态没法流转到Published，逻辑就是没错的
    post.request_review();

    // 2.补充需求：为了将文章状态修改为Published，用户需要调用两次approve
    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
