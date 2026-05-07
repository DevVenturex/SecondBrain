pub mod ticket;

pub trait Model {
    type Insert: Insertable;
    type Find: Findable;
    type Update: Updatable;
    type Delete: Deletable;
}

pub trait Insertable {}
pub trait Findable {}
pub trait Updatable {}
pub trait Deletable {}