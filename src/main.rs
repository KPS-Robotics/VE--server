mod application;
mod core;
mod infrastructure;
use std::process::Command;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = Command::new("clear").status();
    print!("\x1b[32m");
    println!(
        r#"
              ┌┐                ┌┐
              ├┴─┐             ┌┴┴┐
              │  └─┐          ┌┘  │ 
              │    └──────────┘   │
              │                   │
      ┌┐     │                    │
    ┌─┴┴─┐   │    ───       #==   │     Created by Mikhail Void Akhsakov
   ┌┘    └─┐ │                    │
  ┌┘       │ │           ___      │
┌─┘        └┬┘       ___ \  \     │
│           │         \-____|     │
│         ┌─                     ┌┘
│         │                      │
│         │                      │
│        ┌┘                      │
└┐       │                      │
 │      ┌┘       │  │        │  │
 └┐    ┌┘  ────┐ │  │        │  │
  └┐   │       │ │  │    │   │  │
   └┐  │       │ │  │    │   │  │
    └─┬┤       ├─┤  │    │   │  │
      └┤       │┼│  ├─┬──┤   │  ├─┐
       └───────┴─┤  │┼│  └───┤  │┼│
                 └──┴─┘      └──┴─┘"#
    );

    infrastructure::run_server::run_server().await
}
