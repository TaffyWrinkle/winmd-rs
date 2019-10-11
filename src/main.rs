// #![allow(unused_variables)]
// #![allow(dead_code)]

mod codes;
mod database;
mod error;
mod flags;
pub mod reader;
mod tables;
use database::*;
use tables::*;
use reader::*;

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}

fn run() -> std::io::Result<()> {

    let reader = Reader::new(&[r"c:\windows\system32\winmetadata\Windows.Foundation.winmd"])?;

    if let Some(t) = reader.find("Windows.Foundation", "IStringable")
    {
    println!(" {}.{}", t.namespace()?, t.name()?);
    }

    // let db = Database::new(r"c:\windows\system32\winmetadata\Windows.Foundation.winmd")?;

    // // for type_ref in db.type_ref()
    // // {
    // //     println!(" {}.{}", type_ref.namespace()?, type_ref.name()?);
    // // }

    // for type_def in db.type_def() {
    //     let flags = type_def.flags()?;

    //     if !flags.windows_runtime() {
    //         continue;
    //     }

    //     // if type_def.name()? != "IStringable" {
    //     //     continue;
    //     // }

    //     let category = type_def.category()?;

    //     match category {
    //         Category::Interface => print!("interface"),
    //         Category::Class => print!("class"),
    //         Category::Enum => print!("enum"),
    //         Category::Struct => print!("struct"),
    //         Category::Delegate => print!("delegate"),
    //         Category::Attribute => print!("attribute"),
    //         Category::Contract => print!("contract"),
    //     }

    //     println!(" {}.{}", type_def.namespace()?, type_def.name()?);

    //     // if category == Category::Interface {
    //     //     for method in type_def.methods()? {
    //     //         println!("  {}", method.name()?);
    //     //     }
    //     // }

    //     // let a = type_def.attributes()?;

    //     // for attribute in a {
    //     //     println!("at {} {}", attribute.first, attribute.last);
    //     //     if attribute.has_name("Windows.Foundation.Metadata", "GuidAttribute")?
    //     //     {
    //     //     println!("guid");
    //     //     }
    //     //     if attribute.has_name("Windows.Foundation.Metadata", "ContractVersionAttribute")?
    //     //     {
    //     //     println!("contract");
    //     //     }
    //     // }
    // }
    Ok(())
}
