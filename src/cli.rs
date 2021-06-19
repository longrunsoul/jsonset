use clap::{
    App,
    Arg,
    crate_authors,
    crate_description,
    crate_name,
    crate_version,
};

pub(crate) fn get_cli<'a, 'b>() -> App<'a, 'b> {
    let app =
        App::new(crate_name!())
            .version(crate_version!())
            .about(crate_description!())
            .author(crate_authors!())
            .arg(
                Arg::with_name("key")
                    .help("the JSONPath to select nodes")
                    .long("key")
                    .short("k")
                    .value_name("KEY")
                    .required(true)
            )
            .arg(
                Arg::with_name("value")
                    .help("the value to set to")
                    .long("value")
                    .short("v")
                    .value_name("VALUE")
                    .required(true)
            )
            .arg(
                Arg::with_name("as-null")
                    .help("set value as null")
                    .long("as-null")
                    .short("u")
            )
            .arg(
                Arg::with_name("as-bool")
                    .help("set value as bool")
                    .long("as-bool")
                    .short("b")
            )
            .arg(
                Arg::with_name("as-number")
                    .help("set value as number")
                    .long("as-number")
                    .short("n")
            )
            .arg(
                Arg::with_name("FILE")
                    .help("the file containing json text")
                    .required(true)
            );
    return app;
}