pub mod entity {
    pub mod task {
        use crate::domain::vo;
        use chrono::{DateTime, Utc};
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Task {
            id: vo::TaskId,
            title: String,
            category: vo::Category,
            content: String,
            created_at: DateTime<Utc>,
            updated_at: DateTime<Utc>,
        }

        #[derive(Deserialize, Debug)]
        pub struct CreateCommand {
            pub title: String,
            pub category: vo::Category,
            pub content: String,
        }

        impl Task {
            pub fn create(cmd: CreateCommand) -> Result<Self, anyhow::Error> {
                let now = Utc::now();
                Ok(Self {
                    id: vo::TaskId::new(),
                    title: cmd.title,
                    category: cmd.category,
                    content: cmd.content,
                    created_at: now,
                    updated_at: now,
                })
            }
            pub fn id(&self) -> vo::TaskId {
                self.id
            }
            pub fn title(&self) -> &str {
                self.title.as_ref()
            }
            pub fn created_at(&self) -> DateTime<Utc> {
                self.created_at
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

        pub fn to_string(&self) -> String {
            let mut buff = vec![0_u8; uuid::adapter::SimpleRef::LENGTH];
            self.0.to_simple_ref().encode_lower(buff.as_mut_slice());
            String::from_utf8(buff).expect("Task uuid to valid utf8")
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

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn task_id_to_string() {
            TaskId::new().to_string();
        }
    }
}
