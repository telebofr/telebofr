use crate::codegen::schema::Type;

pub enum Convert {
    #[allow(dead_code)]
    Id(Type),
    Into(Type),
    Collect(Type),
}

pub fn convert_for(ty: &Type) -> Convert {
    match ty {
        ty @ Type::True
        | ty @ Type::u8
        | ty @ Type::u16
        | ty @ Type::u32
        | ty @ Type::i32
        | ty @ Type::u64
        | ty @ Type::i64
        | ty @ Type::f64
        | ty @ Type::bool => Convert::Id(ty.clone()),
        ty @ Type::String => Convert::Into(ty.clone()),
        Type::Option(inner) => convert_for(inner),
        Type::ArrayOf(ty) => Convert::Collect((**ty).clone()),
        Type::RawTy(s) => match s.as_str() {
            raw @ "Recipient" | raw @ "ChatId" | raw @ "TargetMessage" | raw @ "ReplyMarkup" => {
                Convert::Into(Type::RawTy(raw.to_owned()))
            }
            raw => Convert::Id(Type::RawTy(raw.to_owned())),
        },
        ty @ Type::Url => Convert::Id(ty.clone()),
        ty @ Type::DateTime => Convert::Into(ty.clone()),
        ty @ Type::Box(_) => Convert::Into(ty.clone()),
    }
}
