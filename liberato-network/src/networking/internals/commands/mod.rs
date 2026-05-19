pub enum Commands {
    lsp, // list peers (-dom | domains)
    lsfs, // list filesystem
    lsu, // list users
    lsdom, // list domains

    ping, // test ping
    user, // user information (--key | public key) (--id | peer_id) (--is_ephermal | checks whether ephermal key)

    gix, // git like repo

}