// Copyright 2018 Stafi Protocol, Inc.
// This file is part of Stafi.

// Stafi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Stafi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

#![warn(missing_docs)]

extern crate ctrlc;
extern crate futures;
extern crate serde_json as json;
extern crate stafi_cli as cli;

use cli::VersionInfo;
use futures::sync::oneshot;
use futures::{future, Future};
use json::Value;
use std::cell::RefCell;

// handles ctrl-c
struct Exit;
impl stafi_cli::IntoExit for Exit {
    type Exit = future::MapErr<oneshot::Receiver<()>, fn(oneshot::Canceled) -> ()>;
    fn into_exit(self) -> Self::Exit {
        // can't use signal directly here because CtrlC takes only `Fn`.
        let (exit_send, exit) = oneshot::channel();

        let exit_send_cell = RefCell::new(Some(exit_send));
        ctrlc::set_handler(move || {
            if let Some(exit_send) = exit_send_cell
                .try_borrow_mut()
                .expect("signal handler not reentrant; qed")
                .take()
            {
                exit_send.send(()).expect("Error sending exit notification");
            }
        })
        .expect("Error setting Ctrl-C handler");

        exit.map_err(drop)
    }
}

fn merge_args_with_config() -> Box<dyn Iterator<Item = String>> {
    let args: Vec<String> = std::env::args().collect();
    let mut merged_args: Vec<String> = Vec::new();
    let mut config_paths: Vec<&str> = Vec::new();
    let mut config_index: usize = args.len();
    for (i, arg) in args.iter().enumerate() {
        if i == config_index + 1 {
            config_paths.push(arg);
            continue;
        }
        if arg == "-c" || arg == "--config" {
            config_index = i;
            continue;
        }
        merged_args.push(arg.to_owned());
    }

    for config_path in config_paths {
        let contents =
            std::fs::read_to_string(config_path).expect("Something went wrong reading the file");

        if let Ok(parsed) = json::from_str(&contents) {
            if let json::Value::Object(a) = parsed {
                for (name, value) in a {
                    let result: Result<Option<String>, String> = match value {
                        Value::String(s) => Ok(Some(s.to_owned())),
                        Value::Bool(b) => Ok(None),
                        Value::Number(n) => Ok(Some(n.to_string())),
                        _ => Err(format!("Unexpected config for {}", name).to_owned()),
                    };
                    match result {
                        Ok(s) => {
                            if name.starts_with("--") {
                                merged_args.push(name.to_owned());
                            } else {
                                merged_args.push(("--".to_owned() + &name).to_owned());
                            }
                            if let Some(s) = s {
                                merged_args.push(s);
                            }
                        }
                        Err(s) => {
                            println!("{}", s);
                        }
                    }
                }
            }
        }
    }

    Box::new(merged_args.into_iter())
}

fn main() {
    let version = VersionInfo {
        name: "Stafi",
        commit: env!("VERGEN_SHA_SHORT"),
        version: env!("CARGO_PKG_VERSION"),
        executable_name: "stafi",
        author: "Stafi Protocol",
        description: "Stafi Client Node",
        support_url: "https://github.com/stafiprotocol/stafi-node/issues/new",
    };

    let args = merge_args_with_config();
    if let Err(e) = stafi_cli::run(args, Exit, version) {
        eprintln!("Fatal error: {}\n\n{:?}", e, e);
        std::process::exit(1)
    }
}
