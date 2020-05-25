pub mod entity {
    pub mod task {
        use crate::domain::vo;
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Task {
            id: vo::TaskId,
            title: String,
            category: vo::Category,
            content: String,
        }

        #[derive(Deserialize, Debug)]
        pub struct CreateCommand {
            pub title: String,
            pub category: vo::Category,
            pub content: String,
        }

        impl Task {
            pub fn create(cmd: CreateCommand) -> Result<Self, anyhow::Error> {
                Ok(Self {
                    id: vo::TaskId::new(),
                    title: cmd.title,
                    category: cmd.category,
                    content: cmd.content,
                })
            }
            pub fn id(&self) -> vo::TaskId {
                self.id
            }
        }
    }
}

pub mod vo {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(PartialEq, Eq, Hash, Debug, Copy, Clone, Serialize, Deserialize)]
    pub struct TaskId(Uuid);

    impl TaskId {
        pub fn new() -> Self {
            Self(Uuid::new_v4())
        }
    }
    // clippyに怒られたので定義しておく
    impl Default for TaskId {
        fn default() -> Self {
            TaskId::new()
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Category(String);

    impl Default for Category {
        fn default() -> Self {
            Category(String::from("default"))
        }
    }
}
