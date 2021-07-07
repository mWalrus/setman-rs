// Copyright (c) 2021 Oskar Hellkvist <hellkvistoskar@protonmail.com>

// SPDX-License-Identifier: BSD-2-Clause

use colored::*;

pub fn print_header() {
    println!(
        "{}\n\n{}\n",
        "      ::::::::  :::::::::: ::::::::::: :::   :::       :::     ::::    :::
    :+:    :+: :+:            :+:    :+:+: :+:+:    :+: :+:   :+:+:   :+:
   +:+        +:+            +:+   +:+ +:+:+ +:+  +:+   +:+  :+:+:+  +:+
  +#++:++#++ +#++:++#       +#+   +#+  +:+  +#+ +#++:++#++: +#+ +:+ +#+
        +#+ +#+            +#+   +#+       +#+ +#+     +#+ +#+  +#+#+#
#+#    #+# #+#            #+#   #+#       #+# #+#     #+# #+#   #+#+#
########  ##########     ###   ###       ### ###     ### ###    ####"
            .bold()
            .blue(),
        "Application settings manager".bright_cyan().bold()
    );
}

//https://doc.rust-lang.org/reference/macros-by-example.html
macro_rules! __log {
    ($pre:expr, $($arg:tt)*) => {{
        use ::colored::*;
        let pre = $pre;
        let pre = match pre {
            $crate::LogLevel::Job => {"[~]".bold().yellow()},
            $crate::LogLevel::Info => {"[*]".bold().blue()},
            $crate::LogLevel::Warning => {"[!]".bold().red()},
        };
        println!("{} {}", pre, format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! job {
    ($($arg:tt)*) => {{
        __log!($crate::LogLevel::Job, $($arg)*);
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        __log!($crate::LogLevel::Info, $($arg)*);
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        __log!($crate::LogLevel::Warning, $($arg)*);
    }};
}

#[macro_export]
macro_rules! print_app {
    ($app:expr, $verbose:tt) => {{
        use ::colored::*;
        let app = $app;
        info!("{}", app.name);
        if $verbose {
            println!("{}{} {:?}", " ".repeat(4), "Config path =>".bold().cyan(), app.config_path);
            if &app.file_names.len() > &0 {
                println!("{}{}", " ".repeat(4), "File names:".bold().green());
                for file in &app.file_names {
                    println!("{}{} {}", " ".repeat(9), "=>".bold().green(), file);
                }
            }

        }
    }};
}
