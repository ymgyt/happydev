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
    use std::{fmt, str};
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

    impl fmt::Display for TaskId {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl str::FromStr for TaskId {
        type Err = anyhow::Error;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Uuid::from_str(s).map(TaskId).map_err(anyhow::Error::from)
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
