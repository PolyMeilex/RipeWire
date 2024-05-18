use libspa_consts::{SpaFormat, SpaType};

use crate::{deserialize::PodDeserializerKind, PodDeserializer};

pub fn dbg_print(bytes: &[u8]) {
    let (pod, buff) = PodDeserializer::new(bytes);
    assert!(buff.is_empty());
    print_item(0, pod);
}

fn pad(nest: usize) {
    for _ in 0..nest {
        print!("  ");
    }
}

fn print_item(nest: usize, pod: PodDeserializer) {
    pad(nest);
    // print!("Size: {}, Ty: {:?}", pod.size(), pod.ty());
    // print!("{:?}", pod.ty());

    match pod.kind() {
        PodDeserializerKind::None => {
            println!("None");
        }
        PodDeserializerKind::Bool(value) => {
            println!("Bool({value:?})");
        }
        PodDeserializerKind::Id(value) => {
            println!("Id({value:?})");
        }
        PodDeserializerKind::Int(value) => {
            println!("Int({value:?})");
        }
        PodDeserializerKind::Long(value) => {
            println!("Long({value:?})");
        }
        PodDeserializerKind::Float(value) => {
            println!("Float({value:?})");
        }
        PodDeserializerKind::Double(value) => {
            println!("Double({value:?})");
        }
        PodDeserializerKind::Rectangle(value) => {
            println!("{value:?}");
        }
        PodDeserializerKind::Fraction(value) => {
            println!("{value:?}");
        }
        PodDeserializerKind::Bitmap(bytes) => {
            println!("Bitmap {bytes:?}");
        }
        PodDeserializerKind::Array(pod) => {
            println!("Array [");
            for pod in pod {
                print_item(nest + 1, pod);
            }
            pad(nest);
            println!("]");
        }
        PodDeserializerKind::Struct(pod) => {
            println!("Struct {{");
            for pod in pod {
                print_item(nest + 1, pod);
            }
            pad(nest);
            println!("}}");
        }
        PodDeserializerKind::Object(pod) => {
            let object_ty = pod.object_ty();
            println!("{object_ty:?} {{");
            for property in pod {
                pad(nest + 1);
                if object_ty == SpaType::ObjectFormat {
                    print!("{:?}:", SpaFormat::from_raw(property.key).unwrap());
                } else {
                    print!("key: {:?}", property.key);
                }
                print_item(nest + 1, property.pod);
            }
            pad(nest);
            println!("}}");
        }
        PodDeserializerKind::Fd(value) => {
            println!("Fn({value})");
        }
        PodDeserializerKind::Choice(pod) => {
            println!("Choice::{:?} [", pod.choice_ty());
            for pod in pod {
                print_item(nest + 1, pod);
            }
            pad(nest);
            println!("]");
        }
        PodDeserializerKind::Bytes(bytes) => {
            println!("Bytes {bytes:?}");
        }
        PodDeserializerKind::String(s) => {
            println!("String({s:?})");
        }
        PodDeserializerKind::Unknown(_pod) => {
            println!("{:?}", pod.ty());
        }
    }
}
