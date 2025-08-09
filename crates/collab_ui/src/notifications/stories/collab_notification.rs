use gpui::prelude::*;
use story::{Story, StoryItem, StorySection};
use ui::prelude::*;

use crate::notifications::collab_notification::CollabNotification;

pub struct CollabNotificationStory;

impl Render for CollabNotificationStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let window_container = |width, height| div().w(px(width)).h(px(height));

        Story::container(cx)
            .child(Story::title_for::<CollabNotification>(cx))
            .child(
                StorySection::new().child(StoryItem::new(
                    "Incoming Call Notification",
                    window_container(400., 72.).child(
                        CollabNotification::new(
                            "https://avatars.githubusercontent.com/u/1486634?v=4",
                            Button::new("accept", "接受"),
                            Button::new("decline", "拒绝"),
                        )
                        .child(
                            v_flex()
                                .overflow_hidden()
                                .child(Label::new("maxdeviant 在 Zed 中分享一个项目")),
                        ),
                    ),
                )),
            )
            .child(
                StorySection::new().child(StoryItem::new(
                    "Project Shared Notification",
                    window_container(400., 72.).child(
                        CollabNotification::new(
                            "https://avatars.githubusercontent.com/u/1714999?v=4",
                            Button::new("open", "打开"),
                            Button::new("dismiss", "关闭"),
                        )
                        .child(Label::new("iamnbutler"))
                        .child(Label::new("正在在 Zed 中分享一个项目："))
                        .child(Label::new("zed")),
                    ),
                )),
            )
    }
}
