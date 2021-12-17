use arma_rs::{arma, Extension, Group};

mod system_info;
mod timer;

#[arma]
fn init() -> Extension {
    Extension::build()
        .command("log", log)
        .group(
            "hello",
            Group::new()
                .command("english", hello::english)
                .command("french", hello::french)
                .command("spanish", hello::spanish),
        )
        .group(
            "welcome",
            Group::new()
                .command("english", welcome::english)
                .command("french", welcome::french)
                .command("spanish", welcome::spanish),
        )
        .group("system", system_info::group())
        .group("timer", timer::group())
        .finish()
}

pub fn log(s: String) {
    println!("{}", s);
}

mod hello {
    pub fn english() -> &'static str {
        "hello"
    }

    pub fn french() -> &'static str {
        "bonjour"
    }

    pub fn spanish() -> &'static str {
        "hola"
    }
}

mod welcome {
    pub fn english(name: String) -> String {
        format!("Welcome {}", name)
    }

    pub fn french(name: String) -> String {
        format!("Bienvenue {}", name)
    }

    pub fn spanish(name: String) -> String {
        format!("Bienvenido {}", name)
    }
}
