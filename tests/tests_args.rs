#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_args() {
        let args = vec![
            (
                vec!["filerefine".to_owned(), "README.md".to_owned()],
                filerefine::OptionnalFields {
                    options: filerefine::OptionsFields { dry_run: true },
                    verbosity: filerefine::VerbosityFields {
                        verbose: true,
                        json: false,
                        json_pretty: false,
                        json_error: false,
                    },
                },
            ),
            (
                vec!["filerefine".to_owned(), "README.md".to_owned(), "-d".to_owned()],
                filerefine::OptionnalFields {
                    options: filerefine::OptionsFields { dry_run: false },
                    verbosity: filerefine::VerbosityFields {
                        verbose: true,
                        json: false,
                        json_pretty: false,
                        json_error: false,
                    },
                },
            ),
            (
                vec![
                    "filerefine".to_owned(),
                    "README.md".to_owned(),
                    "-d".to_owned(),
                    "-j".to_owned(),
                ],
                filerefine::OptionnalFields {
                    options: filerefine::OptionsFields { dry_run: false },
                    verbosity: filerefine::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: false,
                        json_error: false,
                    },
                },
            ),
            (
                vec![
                    "filerefine".to_owned(),
                    "README.md".to_owned(),
                    "-d".to_owned(),
                    "--json".to_owned(),
                ],
                filerefine::OptionnalFields {
                    options: filerefine::OptionsFields { dry_run: false },
                    verbosity: filerefine::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: false,
                        json_error: false,
                    },
                },
            ),
            (
                vec![
                    "filerefine".to_owned(),
                    "README.md".to_owned(),
                    "-d".to_owned(),
                    "-e".to_owned(),
                ],
                filerefine::OptionnalFields {
                    options: filerefine::OptionsFields { dry_run: false },
                    verbosity: filerefine::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: false,
                        json_error: true,
                    },
                },
            ),
            (
                vec![
                    "filerefine".to_owned(),
                    "README.md".to_owned(),
                    "-d".to_owned(),
                    "--json-error".to_owned(),
                ],
                filerefine::OptionnalFields {
                    options: filerefine::OptionsFields { dry_run: false },
                    verbosity: filerefine::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: false,
                        json_error: true,
                    },
                },
            ),
            (
                vec![
                    "filerefine".to_owned(),
                    "README.md".to_owned(),
                    "-d".to_owned(),
                    "-p".to_owned(),
                ],
                filerefine::OptionnalFields {
                    options: filerefine::OptionsFields { dry_run: false },
                    verbosity: filerefine::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: true,
                        json_error: false,
                    },
                },
            ),
            (
                vec![
                    "filerefine".to_owned(),
                    "README.md".to_owned(),
                    "-d".to_owned(),
                    "--json-pretty".to_owned(),
                ],
                filerefine::OptionnalFields {
                    options: filerefine::OptionsFields { dry_run: false },
                    verbosity: filerefine::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: true,
                        json_error: false,
                    },
                },
            ),
            (
                vec!["filerefine".to_owned(), "-v".to_owned()],
                filerefine::OptionnalFields {
                    options: filerefine::OptionsFields { dry_run: true },
                    verbosity: filerefine::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: true,
                        json_error: false,
                    },
                },
            ),
            (
                vec!["filerefine".to_owned(), "--version".to_owned()],
                filerefine::OptionnalFields {
                    options: filerefine::OptionsFields { dry_run: true },
                    verbosity: filerefine::VerbosityFields {
                        verbose: false,
                        json: true,
                        json_pretty: true,
                        json_error: false,
                    },
                },
            ),
            (
                vec!["filerefine".to_owned(), "-q".to_owned()],
                filerefine::OptionnalFields {
                    options: filerefine::OptionsFields { dry_run: true },
                    verbosity: filerefine::VerbosityFields {
                        verbose: false,
                        json: false,
                        json_pretty: false,
                        json_error: false,
                    },
                },
            ),
            (
                vec!["filerefine".to_owned(), "--quiet".to_owned()],
                filerefine::OptionnalFields {
                    options: filerefine::OptionsFields { dry_run: true },
                    verbosity: filerefine::VerbosityFields {
                        verbose: false,
                        json: false,
                        json_pretty: false,
                        json_error: false,
                    },
                },
            ),
        ];
        print!("ARGS: {:?}\n", args);
        for one_test in args.iter() {
            let res = filerefine::parse_args(one_test.0.clone());
            if let Ok(ok_res) = res {
                assert_eq!(ok_res.0, one_test.1);
            } else {
                assert_eq!(res.err().unwrap(), 1)
            }
        }
    }

    #[test]
    fn test_parse_args_no_file_found() {
        let vec_args = vec![
            "filerefine".to_owned(),
            "README.md".to_owned(),
            "README".to_owned(),
        ];
        let res = filerefine::parse_args(vec_args);
        let (options, vect) = res.ok().unwrap();
        assert_eq!(
            options,
            filerefine::OptionnalFields {
                options: filerefine::OptionsFields { dry_run: true },
                verbosity: filerefine::VerbosityFields {
                    verbose: true,
                    json: false,
                    json_pretty: false,
                    json_error: false,
                },
            }
        );
        assert_eq!(vect.len(), 1);
    }

    #[test]
    fn test_parse_args_star() {
        let vec_args = vec!["filerefine".to_owned(), "*".to_owned()];
        let res = filerefine::parse_args(vec_args);
        let (options, vect) = res.ok().unwrap();
        assert_eq!(
            options,
            filerefine::OptionnalFields {
                options: filerefine::OptionsFields { dry_run: true },
                verbosity: filerefine::VerbosityFields {
                    verbose: true,
                    json: false,
                    json_pretty: false,
                    json_error: false,
                },
            }
        );
        let number = std::fs::read_dir(".")
            .ok()
            .unwrap()
            .filter(|entry| entry.is_ok())
            .count();
        assert_eq!(vect.len(), number);
    }
}
