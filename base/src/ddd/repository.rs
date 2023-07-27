use super::aggregate::IAggregate;

pub trait IRepository {
    type AG: IAggregate;
    type ID;
    fn delete(id: Self::ID);
    fn by_id(id: Self::ID) -> Self::AG;
    fn save(s: Self::AG) -> Self::AG;
    fn save_and_flush(s: Self::AG) -> Self::AG;
}
