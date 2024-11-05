pub(crate) enum Cmd {
    Set { tag: String, dir: String },
    Get { tag: String },
    Delete { tag: String },
    List,
    Init { filename: String },
}
