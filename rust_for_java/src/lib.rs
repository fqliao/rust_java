extern crate jni;

use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jint, jobject};
use jni::JNIEnv;
//use std::{thread, time};

#[no_mangle]
pub extern "system" fn Java_org_com_fisco_RustLib_add(_env: JNIEnv, _class: JClass, a: jint, b: jint) -> jint {
    a + b
}

#[no_mangle]
pub extern "system" fn Java_org_com_fisco_RustLib_getPerson(
    _env: JNIEnv,
    _class: JClass,
    name: JString, // java string parameter must be JString
    age: jint,
) -> jobject {
    //1. find java class
    let person_class = _env.find_class("Lorg/com/fisco/Person;").expect("could not find class");

    //2. allocate java object
    let person = _env
        .alloc_object(person_class)
        .expect("Could not allocate object");
    //3 set field
    _env.set_field(
        person,
        "name",
        "Ljava/lang/String;",
        JValue::from(JObject::from(name)),
    )
    .expect("Could not set name field");
    _env.set_field(person, "age", "I", JValue::from(age))
        .expect("Could not set age field");

    //4 return object
    person.into_inner()
}
