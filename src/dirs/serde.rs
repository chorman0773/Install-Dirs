use super::InstallDirs;

macro_rules! impl_serde{
    {
        $tyname:ident = $defaults:ident {
            $($field:ident),*
            $(,)?
        }
    } => {
        const _: () = {
            use ::serde::ser::SerializeStruct as _;
            const __FIELD_COUNT: usize = (0 $(+(1,::core::stringify!($field)).0)*);

            const __FIELDS: [&'static str; __FIELD_COUNT] = [$(::core::stringify!($field)),*];

            impl ::serde::ser::Serialize for $tyname{
                fn serialize<__S>(&self, serializer: __S) -> Result<__S::Ok,__S::Error> where __S: ::serde::ser::Serializer{
                    let mut fields = serializer.serialize_struct(::core::stringify!($tyname),__FIELD_COUNT)?;

                    $(fields.serialize_field(::core::stringify!($field), &self.$field)?;)*

                    fields.end()
                }
            }

            #[allow(non_camel_case_types)]
            enum __Field{
                $($field),*
            }

            struct __FieldVisitor;

            impl<'de> ::serde::de::Visitor<'de> for __FieldVisitor{
                type Value = __Field;

                #[allow(unused_mut,unused_variables, unused_assignments)]
                fn expecting(&self, formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result{
                    formatter.write_str("one of: ")?;
                    let mut sep = "";

                    $({
                        formatter.write_str(sep)?;
                        sep = ", ";
                        formatter.write_str(::core::stringify!($field))?;
                    })*

                    Ok(())
                }

                fn visit_str<__E>(self, value: &str) -> Result<__Field,__E> where __E: ::serde::de::Error{
                    match value{
                        $(::core::stringify!($field) => Ok(__Field:: $field),)*
                        value => Err(::serde::de::Error::unknown_field(value, &__FIELDS))
                    }
                }
            }

            impl<'de> ::serde::de::Deserialize<'de> for __Field{
                fn deserialize<__D>(deserializer: __D) -> Result<__Field,__D::Error> where __D: ::serde::de::Deserializer<'de>{
                    deserializer.deserialize_identifier(__FieldVisitor)
                }
            }

            struct __Visitor;

            impl<'de> ::serde::de::Visitor<'de> for __Visitor{
                type Value = $tyname;

                fn expecting(&self, formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result{
                    formatter.write_str(::core::concat!("struct ", ::core::stringify!($tyname)))
                }

                fn visit_seq<__V>(self, mut seq: __V) -> Result<$tyname, __V::Error> where __V: ::serde::de::SeqAccess<'de>{
                    let mut __length = 0;
                    $(let $field = seq.next_element()?.ok_or_else(|| ::serde::de::Error::invalid_length({let __val = __length; __length += 1; __val},&self))?;)*

                    Ok($tyname { $($field),*})
                }

                fn visit_map<__V>(self, mut map: __V) -> Result<$tyname, __V::Error> where __V: ::serde::de::MapAccess<'de>{
                    $(let mut $field = None;)*

                    while let Some(key) = map.next_key()? {
                        match key{
                            $(__Field :: $field => {
                                if $field.is_some(){
                                    return Err(::serde::de::Error::duplicate_field(::core::stringify!($field)));
                                }

                                $field = Some(map.next_value()?);
                            })*
                        }
                    }

                    let defaults = $tyname::$defaults();

                    $(
                        let $field = $field.unwrap_or(defaults.$field);
                    )*

                    Ok($tyname {
                        $($field),*
                    })
                }
            }

            impl<'de> ::serde::de::Deserialize<'de> for $tyname{
                fn deserialize<__D>(deserializer: __D) -> Result<Self, __D::Error> where __D: ::serde::de::Deserializer<'de>{
                    deserializer.deserialize_struct(::core::stringify!($tyname), &__FIELDS, __Visitor)
                }
            }
        };

    }
}

impl_serde! {
    InstallDirs = defaults{
        prefix,
        exec_prefix,
        bindir,
        sbindir,
        libdir,
        libexecdir,
        includedir,
        datarootdir,
        datadir,
        mandir,
        docdir,
        infodir,
        localedir,
        localstatedir,
        runstatedir,
        sharedstatedir,
        sysconfdir,
    }
}
