use std::io::Cursor;

use libspa_consts::{SpaEnum, SpaFormat, SpaMediaType, SpaType};

fn main() {
    {
        let mut builder = pod_v2::Builder::new(Cursor::new(vec![]));

        builder.write_object_with(SpaEnum::Value(SpaType::ObjectFormat), 0, |b| {
            b.write_property(SpaFormat::MediaType as u32, 0, |b| {
                b.write_u32(SpaMediaType::Audio as u32);
            });
            b.write_property(SpaFormat::AudioFlags as u32, 0, |b| {
                b.write_u32(5);
            });
        });

        let builder = builder.into_inner().into_inner();
        pod_v2::dbg_print::dbg_print(&builder);
    }

    let mut builder = pod_v2::Builder::new(Cursor::new(vec![]));

    builder.push_struct_with(|b| {
        b.write_none()
            .write_bool(false)
            .write_bool(true)
            .write_int(u32::MAX as i32)
            .write_long(u64::MAX as i64)
            .write_rectangle(1, 2)
            .write_fraction(1, 2)
            .push_struct_with(|b| {
                b.write_none()
                    .write_bool(false)
                    .write_bool(true)
                    .write_int(u32::MAX as i32)
                    .write_long(u64::MAX as i64);
            })
            .write_str("abc")
            .write_bytes([1, 2])
            .write_bitmap([1, 2])
            .write_array_with(|b| {
                b.write_int(1).write_int(2).write_int(3);
            });
    });

    let builder_out = builder.into_inner().into_inner();
    // let mut pos = 0;
    // for b in builder_out.chunks(4) {
    //     print!("{pos:4} |");
    //
    //     for b in b {
    //         print!("{b:<3}, ");
    //     }
    //
    //     pos += b.len();
    //     print!(" | {pos:4}");
    //     println!();
    // }
    // println!();
    //
    // let value: pod::Value = pod::deserialize::PodDeserializer::deserialize_from(&builder_out)
    //     .unwrap()
    //     .1;
    //
    // dbg!(&value);

    dbg!(builder_out.len());
    pod_v2::dbg_print::dbg_print(&builder_out);

    let builder_out = vec![
        72, 1, 0, 0, 14, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 76, 1, 0, 64, 0, 0, 0, 0, 4, 0, 0, 0, 3,
        0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0,
        0, 4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 15, 0, 0, 0, 3, 0, 4, 0, 4, 0, 0, 0, 1,
        0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 3, 0, 0,
        0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 4, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 24, 0, 0,
        0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 3, 0, 0, 0, 27, 1, 0, 0, 27, 1, 0, 0,
        3, 0, 1, 0, 0, 0, 0, 0, 24, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 4, 0,
        0, 0, 128, 187, 0, 0, 128, 187, 0, 0, 4, 0, 1, 0, 0, 0, 0, 0, 24, 0, 0, 0, 19, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 5, 0, 1, 0, 0, 0, 0,
        0, 32, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 13, 0, 0, 0, 4, 0, 0, 0,
        3, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0,
    ];

    dbg!(builder_out.len());
    pod_v2::dbg_print::dbg_print(&builder_out);

    // for _ in 0..1000 {
    //     println!("{:?} => {}", deserializer.ty(), deserializer.size());
    //     // if deserializer.ty() == SpaType::Struct {
    //     //     let mut deserializer = deserializer.as_struct();
    //     //
    //     //     while deserializer.next() {
    //     //         println!("\t{:?} => {}", deserializer.ty(), deserializer.size());
    //     //
    //     //         deserializer.next();
    //     //     }
    //     // } else {
    //     deserializer.next();
    //     // }
    // }

    //
    // let mut out = Cursor::new(Vec::new());
    // pod::serialize::PodSerializer::serialize(&mut out, &value).unwrap();
    // let out = out.into_inner();
    //
    // assert_eq!(out, builder_out);
}
