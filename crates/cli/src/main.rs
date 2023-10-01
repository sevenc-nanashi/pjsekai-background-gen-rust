mod console;

use getopts::Options;
use pjsekai_background_gen_core as core;
use std::env;

use crate::console::show_title;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let ansi = enable_ansi_support::enable_ansi_support().is_ok();
    console::ANSI.store(ansi, std::sync::atomic::Ordering::SeqCst);
    show_title();
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "", "出力先を指定します。", "NAME");
    opts.optflag("h", "help", "ヘルプを表示します。");
    opts.optopt(
        "v",
        "version",
        "バージョンを指定します。（1、3）",
        "VERSION",
    );
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            console::error(format!("{}", f).as_str());
            std::process::exit(1);
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let output_path = matches.opt_str("o").unwrap_or_else(|| {
        let path = std::path::PathBuf::from(&input);
        path.with_extension("output.png")
            .to_str()
            .unwrap()
            .to_string()
    });
    let version = if let Some(version) = matches.opt_str("v") {
        let version = version.parse::<u32>();
        if version.is_err() {
            console::error("バージョンは1か3を指定してください。");
            std::process::exit(1);
        }
        version.unwrap()
    } else {
        3
    };
    console::info("背景を生成中...");
    let input_image = match image::open(input.as_str()) {
        Ok(image) => image.into_rgba8(),
        Err(e) => {
            console::error(format!("ジャケット読み込みに失敗しました：{}", e).as_str());
            std::process::exit(1);
        }
    };
    let output = match version {
        1 => core::render_v1(&input_image),
        3 => core::render_v3(&input_image),
        _ => {
            console::error("バージョンは1か3を指定してください。");
            std::process::exit(1);
        }
    };
    match output.save(&output_path) {
        Ok(_) => {
            console::info(format!("背景を生成しました：{}", &output_path).as_str());
        }
        Err(e) => {
            console::error(format!("背景保存に失敗しました：{}", e).as_str());
            std::process::exit(1);
        }
    }
}
