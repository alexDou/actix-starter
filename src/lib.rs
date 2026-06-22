#[path = "api/v1"]
pub mod api {
    pub mod auth {
        pub mod login;
        pub mod register;
    }
    pub mod items {
        pub mod fetch;
        pub mod create;
    }
    pub mod monitoring {
        pub mod internal;
    }
    pub mod routes;
}

pub mod domain {
    pub mod auth {
        pub mod lib {
            pub mod common;
            pub mod keys;
            pub mod validator;
        }
        pub mod model;
    }
    pub mod item {
        pub mod entity;
        pub mod model;
    }
    pub mod user {
        pub mod entity;
        pub mod model;
    }
}

pub mod libs {
    pub mod db;
    pub mod errors;
}
