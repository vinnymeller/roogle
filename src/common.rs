use std::{fmt::Display, path::PathBuf};

pub struct CallableInfo {
    pub file: PathBuf,
    pub struct_name: Option<String>,
    pub sig: syn::Signature,
    // pub generic_params: Vec<String>,
    // pub identifier: String,
    // pub args: Vec<String>,
    // pub return_type: String,
}

// impl Display for CallableInfo {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut s = String::new();
//         s.push_str("( ");
//         s.push_str(
//             &self
//                 .args
//                 .iter()
//                 .map(|a| a.as_ref())
//                 .collect::<Vec<_>>()
//                 .join(", "),
//         );
//
//         s.push_str(") ");
//         s.push_str(self.return_type.as_ref());
//         write!(f, "{}", s)
//     }
// }

pub struct UserQuery {
    args: Option<Vec<String>>,
    return_type: Option<String>,
}

impl Display for UserQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str("( ");
        s.push_str(
            &self
                .args
                .as_ref()
                .unwrap_or(&Vec::new())
                .iter()
                .map(|a| a.as_ref())
                .collect::<Vec<_>>()
                .join(", "),
        );

        s.push_str(") ");
        s.push_str(self.return_type.as_ref().unwrap_or(&String::new()));
        write!(f, "{}", s)
    }
}

impl UserQuery {
    pub fn builder() -> UserQueryBuilder {
        UserQueryBuilder::new()
    }

    /// Takes a `Function` as other and returns a string formatted in the
    // same way as our `UserFunction`.to_string() method. This is what
    // gets passed to our distance metrics for calculating most similar
    // functions to our query.
    pub fn comparable_func_str(&self, other: &CallableInfo) -> String {
        let mut user_fn_builder = UserQuery::builder();

        // if other has a struct_name, we want to use that as our name. otherwise, fill in with
        // something. I'm not actually sure what makes the most sense, so for now I'll just say ""
        if self.args.is_some() {
            user_fn_builder = user_fn_builder.args(Some(other.args.clone()));
        }
        if self.return_type.is_some() {
            user_fn_builder = user_fn_builder.return_type(Some(other.return_type.clone()));
        }
        user_fn_builder.build().to_string()
    }
}

pub struct UserQueryBuilder {
    args: Option<Vec<String>>,
    return_type: Option<String>,
}

impl UserQueryBuilder {
    pub fn new() -> Self {
        UserQueryBuilder {
            args: None,
            return_type: None,
        }
    }

    pub fn args(mut self, args: Option<Vec<String>>) -> Self {
        self.args = args;
        self
    }

    pub fn return_type(mut self, return_type: Option<String>) -> Self {
        self.return_type = return_type;
        self
    }

    pub fn build(self) -> UserQuery {
        UserQuery {
            args: if let Some(mut args) = self.args {
                args.sort_unstable();
                Some(args)
            } else {
                None
            },
            return_type: self.return_type,
        }
    }
}
