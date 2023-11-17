
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: Version {
                        major: 1,
                        minor: 72,
                        patch: 0,
                        pre: vec![],
                        build: vec![],
                    },
                    host: "x86_64-apple-darwin".to_owned(),
                    short_version_string: "rustc 1.72.0 (5680fa18f 2023-08-23)".to_owned(),
                    commit_hash: Some("5680fa18feaa87f3ff04063800aec256c3d4b4be".to_owned()),
                    commit_date: Some("2023-08-23".to_owned()),
                    build_date: None,
                    channel: Channel::Stable,
                }
            }
            