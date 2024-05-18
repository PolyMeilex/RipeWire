use std::io::Cursor;

use libspa_consts::{SpaFraction, SpaRectangle, SpaType};

fn main() {
    let mut builder = pod_simple::Builder::new(Cursor::new(vec![]));

    builder.push_struct_with(|b| {
        b.push(())
            .push(false)
            .push(true)
            .push(u32::MAX as i32)
            .push(u64::MAX as i64)
            .push(SpaRectangle {
                width: 1,
                height: 2,
            })
            .push(SpaFraction { num: 1, denom: 2 })
            .push_struct_with(|b| {
                b.push(())
                    .push(false)
                    .push(true)
                    .push(u32::MAX as i32)
                    .push(u64::MAX as i64);
            })
            .push("abc")
            .push([1, 2].as_slice())
            .push_array_with(4, SpaType::Int, |b| {
                b.push(1).push(2).push(3);
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

    let _builder_out = vec![
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
    pod_simple::dbg_print::dbg_print(&builder_out);

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
