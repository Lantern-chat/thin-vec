use postgres_protocol::types;
use postgres_types::{to_sql_checked, FromSql, IsNull, Kind, ToSql, Type};

use fallible_iterator::FallibleIterator;

use crate::ThinVec;

impl<'a, T: FromSql<'a>> FromSql<'a> for ThinVec<T> {
    fn from_sql(
        ty: &postgres_types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let member_type = match *ty.kind() {
            Kind::Array(ref member) => member,
            _ => panic!("expected array type"),
        };

        let array = types::array_from_sql(raw)?;
        if array.dimensions().count()? > 1 {
            return Err("array contains too many dimensions".into());
        }

        array
            .values()
            .map(|v| T::from_sql_nullable(member_type, v))
            .collect()
    }

    fn accepts(ty: &postgres_types::Type) -> bool {
        match *ty.kind() {
            Kind::Array(ref inner) => T::accepts(inner),
            _ => false,
        }
    }
}

impl<T: ToSql> ToSql for ThinVec<T> {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        out: &mut bytes::BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        <&[T] as ToSql>::to_sql(&&**self, ty, out)
    }

    fn accepts(ty: &Type) -> bool {
        <&[T] as ToSql>::accepts(ty)
    }

    to_sql_checked!();
}
