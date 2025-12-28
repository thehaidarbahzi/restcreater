pub mod commands {
    pub mod new;
    pub mod update;
}

pub mod cli;

pub mod templates {
    pub mod lang {
        pub mod golang {
            pub mod blank;
        }
        pub mod rust {}
    }
}

pub mod helper {
    pub mod validation;
    pub mod generation;
    pub mod installation;
}
