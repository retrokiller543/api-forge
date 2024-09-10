#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use serde::{Deserialize, Serialize};
use api_forge::ApiRequest;
use api_forge_macro::Request;
#[request(endpoint = "/posts", response_type = Posts)]
pub struct GetPosts;
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for GetPosts {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serializer::serialize_unit_struct(__serializer, "GetPosts")
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for GetPosts {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<GetPosts>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = GetPosts;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "unit struct GetPosts",
                    )
                }
                #[inline]
                fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    _serde::__private::Ok(GetPosts)
                }
            }
            _serde::Deserializer::deserialize_unit_struct(
                __deserializer,
                "GetPosts",
                __Visitor {
                    marker: _serde::__private::PhantomData::<GetPosts>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for GetPosts {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "GetPosts")
    }
}
#[automatically_derived]
impl ::core::clone::Clone for GetPosts {
    #[inline]
    fn clone(&self) -> GetPosts {
        GetPosts
    }
}
impl api_forge::ApiRequest<Posts> for GetPosts {
    const ENDPOINT: &'static str = "/posts";
    const METHOD: reqwest::Method = reqwest::Method::GET;
    const DATA_TRANSMISSION_METHOD: api_forge::DataTransmissionMethod = api_forge::DataTransmissionMethod::QueryParams;
    const AUTHENTICATION_METHOD: api_forge::AuthenticationMethod = api_forge::AuthenticationMethod::None;
    fn generate_request(
        &self,
        base_url: &str,
        headers: Option<reqwest::header::HeaderMap>,
        token: Option<(String, Option<String>)>,
    ) -> reqwest::RequestBuilder {
        let mut url = ::alloc::__export::must_use({
            let res = ::alloc::fmt::format(
                format_args!("{0}{1}", base_url, Self::ENDPOINT),
            );
            res
        });
        let client = reqwest::Client::new();
        let mut builder = match Self::METHOD {
            reqwest::Method::GET => client.get(&url),
            reqwest::Method::POST => client.post(&url),
            reqwest::Method::PUT => client.put(&url),
            reqwest::Method::DELETE => client.delete(&url),
            reqwest::Method::PATCH => client.patch(&url),
            reqwest::Method::HEAD => client.head(&url),
            _ => client.get(&url),
        };
        builder = match Self::DATA_TRANSMISSION_METHOD {
            api_forge::DataTransmissionMethod::QueryParams => builder.query(self),
            api_forge::DataTransmissionMethod::Json => builder.json(self),
            _ => builder.form(self),
        };
        if let Some((token, password)) = token {
            builder = match Self::AUTHENTICATION_METHOD {
                api_forge::AuthenticationMethod::Basic => {
                    builder.basic_auth(token, password)
                }
                api_forge::AuthenticationMethod::Bearer => builder.bearer_auth(token),
                api_forge::AuthenticationMethod::None => builder,
            };
        }
        let mut all_headers = reqwest::header::HeaderMap::new();
        if let Some(headers) = headers {
            all_headers.extend(headers);
        }
        builder.headers(all_headers)
    }
}
pub struct Post {
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub id: i32,
    pub title: String,
    pub body: String,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Post {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "Post",
                false as usize + 1 + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "userId",
                &self.user_id,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "id",
                &self.id,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "title",
                &self.title,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "body",
                &self.body,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Post {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "userId" => _serde::__private::Ok(__Field::__field0),
                        "id" => _serde::__private::Ok(__Field::__field1),
                        "title" => _serde::__private::Ok(__Field::__field2),
                        "body" => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"userId" => _serde::__private::Ok(__Field::__field0),
                        b"id" => _serde::__private::Ok(__Field::__field1),
                        b"title" => _serde::__private::Ok(__Field::__field2),
                        b"body" => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Post>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Post;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct Post")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        i32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Post with 4 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        i32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Post with 4 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct Post with 4 elements",
                                ),
                            );
                        }
                    };
                    let __field3 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct Post with 4 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(Post {
                        user_id: __field0,
                        id: __field1,
                        title: __field2,
                        body: __field3,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                    let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("userId"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("title"),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field3 => {
                                if _serde::__private::Option::is_some(&__field3) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("body"),
                                    );
                                }
                                __field3 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("userId")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("id")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("title")?
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::__private::Some(__field3) => __field3,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("body")?
                        }
                    };
                    _serde::__private::Ok(Post {
                        user_id: __field0,
                        id: __field1,
                        title: __field2,
                        body: __field3,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["userId", "id", "title", "body"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Post",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Post>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for Post {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "Post",
            "user_id",
            &self.user_id,
            "id",
            &self.id,
            "title",
            &self.title,
            "body",
            &&self.body,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Post {
    #[inline]
    fn clone(&self) -> Post {
        Post {
            user_id: ::core::clone::Clone::clone(&self.user_id),
            id: ::core::clone::Clone::clone(&self.id),
            title: ::core::clone::Clone::clone(&self.title),
            body: ::core::clone::Clone::clone(&self.body),
        }
    }
}
#[automatically_derived]
impl ::core::default::Default for Post {
    #[inline]
    fn default() -> Post {
        Post {
            user_id: ::core::default::Default::default(),
            id: ::core::default::Default::default(),
            title: ::core::default::Default::default(),
            body: ::core::default::Default::default(),
        }
    }
}
#[request(endpoint = "/posts", response_type = Post, method = POST, transmission = Json)]
pub struct CreatePost {
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub title: String,
    pub body: String,
    #[request(header_name = "test")]
    #[serde(skip)]
    header: Option<String>,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for CreatePost {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "CreatePost",
                false as usize + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "userId",
                &self.user_id,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "title",
                &self.title,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "body",
                &self.body,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for CreatePost {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "userId" => _serde::__private::Ok(__Field::__field0),
                        "title" => _serde::__private::Ok(__Field::__field1),
                        "body" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"userId" => _serde::__private::Ok(__Field::__field0),
                        b"title" => _serde::__private::Ok(__Field::__field1),
                        b"body" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<CreatePost>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = CreatePost;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct CreatePost",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        i32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct CreatePost with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct CreatePost with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct CreatePost with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field3 = _serde::__private::Default::default();
                    _serde::__private::Ok(CreatePost {
                        user_id: __field0,
                        title: __field1,
                        body: __field2,
                        header: __field3,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("userId"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("title"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("body"),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("userId")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("title")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("body")?
                        }
                    };
                    _serde::__private::Ok(CreatePost {
                        user_id: __field0,
                        title: __field1,
                        body: __field2,
                        header: _serde::__private::Default::default(),
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["userId", "title", "body"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "CreatePost",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<CreatePost>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for CreatePost {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "CreatePost",
            "user_id",
            &self.user_id,
            "title",
            &self.title,
            "body",
            &self.body,
            "header",
            &&self.header,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for CreatePost {
    #[inline]
    fn clone(&self) -> CreatePost {
        CreatePost {
            user_id: ::core::clone::Clone::clone(&self.user_id),
            title: ::core::clone::Clone::clone(&self.title),
            body: ::core::clone::Clone::clone(&self.body),
            header: ::core::clone::Clone::clone(&self.header),
        }
    }
}
#[automatically_derived]
impl ::core::default::Default for CreatePost {
    #[inline]
    fn default() -> CreatePost {
        CreatePost {
            user_id: ::core::default::Default::default(),
            title: ::core::default::Default::default(),
            body: ::core::default::Default::default(),
            header: ::core::default::Default::default(),
        }
    }
}
impl api_forge::ApiRequest<Post> for CreatePost {
    const ENDPOINT: &'static str = "/posts";
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const DATA_TRANSMISSION_METHOD: api_forge::DataTransmissionMethod = api_forge::DataTransmissionMethod::Json;
    const AUTHENTICATION_METHOD: api_forge::AuthenticationMethod = api_forge::AuthenticationMethod::None;
    fn generate_request(
        &self,
        base_url: &str,
        headers: Option<reqwest::header::HeaderMap>,
        token: Option<(String, Option<String>)>,
    ) -> reqwest::RequestBuilder {
        let mut url = ::alloc::__export::must_use({
            let res = ::alloc::fmt::format(
                format_args!("{0}{1}", base_url, Self::ENDPOINT),
            );
            res
        });
        let client = reqwest::Client::new();
        let mut builder = match Self::METHOD {
            reqwest::Method::GET => client.get(&url),
            reqwest::Method::POST => client.post(&url),
            reqwest::Method::PUT => client.put(&url),
            reqwest::Method::DELETE => client.delete(&url),
            reqwest::Method::PATCH => client.patch(&url),
            reqwest::Method::HEAD => client.head(&url),
            _ => client.get(&url),
        };
        builder = match Self::DATA_TRANSMISSION_METHOD {
            api_forge::DataTransmissionMethod::QueryParams => builder.query(self),
            api_forge::DataTransmissionMethod::Json => builder.json(self),
            _ => builder.form(self),
        };
        if let Some((token, password)) = token {
            builder = match Self::AUTHENTICATION_METHOD {
                api_forge::AuthenticationMethod::Basic => {
                    builder.basic_auth(token, password)
                }
                api_forge::AuthenticationMethod::Bearer => builder.bearer_auth(token),
                api_forge::AuthenticationMethod::None => builder,
            };
        }
        let mut all_headers = reqwest::header::HeaderMap::new();
        if let Some(value) = self.header.as_ref() {
            builder = builder.header("test", value);
        }
        if let Some(headers) = headers {
            all_headers.extend(headers);
        }
        builder.headers(all_headers)
    }
}
#[request(endpoint = "/posts/{id}", method = DELETE, path_parameters = ["id"])]
pub struct DeletePost {
    pub id: i32,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for DeletePost {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "DeletePost",
                false as usize + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "id",
                &self.id,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for DeletePost {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "id" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"id" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<DeletePost>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = DeletePost;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct DeletePost",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        i32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct DeletePost with 1 element",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(DeletePost { id: __field0 })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("id")?
                        }
                    };
                    _serde::__private::Ok(DeletePost { id: __field0 })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["id"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "DeletePost",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<DeletePost>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for DeletePost {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "DeletePost",
            "id",
            &&self.id,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for DeletePost {
    #[inline]
    fn clone(&self) -> DeletePost {
        DeletePost {
            id: ::core::clone::Clone::clone(&self.id),
        }
    }
}
#[automatically_derived]
impl ::core::default::Default for DeletePost {
    #[inline]
    fn default() -> DeletePost {
        DeletePost {
            id: ::core::default::Default::default(),
        }
    }
}
impl api_forge::ApiRequest<EmptyResponse> for DeletePost {
    const ENDPOINT: &'static str = "/posts/{id}";
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    const DATA_TRANSMISSION_METHOD: api_forge::DataTransmissionMethod = api_forge::DataTransmissionMethod::QueryParams;
    const AUTHENTICATION_METHOD: api_forge::AuthenticationMethod = api_forge::AuthenticationMethod::None;
    fn generate_request(
        &self,
        base_url: &str,
        headers: Option<reqwest::header::HeaderMap>,
        token: Option<(String, Option<String>)>,
    ) -> reqwest::RequestBuilder {
        let mut url = ::alloc::__export::must_use({
            let res = ::alloc::fmt::format(
                format_args!("{0}{1}", base_url, Self::ENDPOINT),
            );
            res
        });
        url = url
            .replace(
                &::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(format_args!("{{{0}}}", "id"));
                    res
                }),
                &self.id.to_string(),
            );
        let client = reqwest::Client::new();
        let mut builder = match Self::METHOD {
            reqwest::Method::GET => client.get(&url),
            reqwest::Method::POST => client.post(&url),
            reqwest::Method::PUT => client.put(&url),
            reqwest::Method::DELETE => client.delete(&url),
            reqwest::Method::PATCH => client.patch(&url),
            reqwest::Method::HEAD => client.head(&url),
            _ => client.get(&url),
        };
        builder = match Self::DATA_TRANSMISSION_METHOD {
            api_forge::DataTransmissionMethod::QueryParams => builder.query(self),
            api_forge::DataTransmissionMethod::Json => builder.json(self),
            _ => builder.form(self),
        };
        if let Some((token, password)) = token {
            builder = match Self::AUTHENTICATION_METHOD {
                api_forge::AuthenticationMethod::Basic => {
                    builder.basic_auth(token, password)
                }
                api_forge::AuthenticationMethod::Bearer => builder.bearer_auth(token),
                api_forge::AuthenticationMethod::None => builder,
            };
        }
        let mut all_headers = reqwest::header::HeaderMap::new();
        if let Some(headers) = headers {
            all_headers.extend(headers);
        }
        builder.headers(all_headers)
    }
}
pub struct Posts(Vec<Post>);
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Posts {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serializer::serialize_newtype_struct(__serializer, "Posts", &self.0)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Posts {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Posts>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Posts;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "tuple struct Posts",
                    )
                }
                #[inline]
                fn visit_newtype_struct<__E>(
                    self,
                    __e: __E,
                ) -> _serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: Vec<Post> = <Vec<
                        Post,
                    > as _serde::Deserialize>::deserialize(__e)?;
                    _serde::__private::Ok(Posts(__field0))
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        Vec<Post>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct Posts with 1 element",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(Posts(__field0))
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "Posts",
                __Visitor {
                    marker: _serde::__private::PhantomData::<Posts>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for Posts {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Posts", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Posts {
    #[inline]
    fn clone(&self) -> Posts {
        Posts(::core::clone::Clone::clone(&self.0))
    }
}
#[automatically_derived]
impl ::core::default::Default for Posts {
    #[inline]
    fn default() -> Posts {
        Posts(::core::default::Default::default())
    }
}
fn main() {
    let body = async {
        let request = GetPosts;
        let base_url = "https://jsonplaceholder.typicode.com";
        let result = request.send_and_parse(base_url, None, None).await;
        match result {
            Ok(post) => {
                ::std::io::_print(
                    format_args!("Successfully fetched post: {0:?}\n", post),
                );
            }
            Err(e) => {
                ::std::io::_eprint(format_args!("Error occurred: {0:?}\n", e));
            }
        }
        let request = CreatePost {
            user_id: 1,
            title: "Test".to_string(),
            body: "Test".to_string(),
            header: Some("test-header".to_string()),
        };
        let result = request.send_and_parse(base_url, None, None).await;
        match result {
            Ok(post) => {
                ::std::io::_print(
                    format_args!("Successfully created post: {0:?}\n", post),
                );
            }
            Err(e) => {
                ::std::io::_eprint(format_args!("Error occurred: {0:?}\n", e));
            }
        }
        let request = DeletePost { id: 100 };
        let result = request.send_and_parse(base_url, None, None).await;
        match result {
            Ok(post) => {
                ::std::io::_print(
                    format_args!("Successfully deleted post: {0:?}\n", post),
                );
            }
            Err(e) => {
                ::std::io::_eprint(format_args!("Error occurred: {0:?}\n", e));
            }
        }
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
