use facet::{EnumType, Facet, StructType, Type, UserType, Variant};
use facet_reflect::{HasFields, Peek, PeekEnum};

struct Out {}

fn get_stuffs<M: Facet<'static>>(msg: &M) -> Result<Out, Box<dyn std::error::Error>> {
    // goal: get nested type, and full variant name

    let peek = Peek::new(msg);
    let peek_enum = peek.into_enum()?;
    let (variant, variant_data) = peek_enum.fields().next().expect("TODO");

    let Type::User(UserType::Enum(EnumType { variants, .. })) = M::SHAPE.ty else {
        panic!("expected enum");
    };

    let Type::User(UserType::Struct(StructType { fields, .. })) =
        variants[0].data.fields[0].shape().ty
    else {
        panic!("expected struct");
    };

    let field = &fields[0];

    let &sens_id = field
        .get_attr(Some("datapoint"), "sensor_id")
        .expect("attribute should exist")
        .get_as::<&'static str>()
        .expect("attribute should decode");
    assert_eq!(sens_id, "lox-vent");

    todo!()
}
