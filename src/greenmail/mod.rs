use std::{borrow::Cow, collections::HashMap};

use testcontainers::{
    core::{ContainerPort, WaitFor},
    Image,
};

const NAME: &str = "greenmail/standalone";
const TAG: &str = "2.1.3";

const SMTP_PORT: ContainerPort = ContainerPort::Tcp(3025);
const POP3_PORT: ContainerPort = ContainerPort::Tcp(3110);
const IMAP_PORT: ContainerPort = ContainerPort::Tcp(3134);
const SMTPS_PORT: ContainerPort = ContainerPort::Tcp(3465);
const POP3S_PORT: ContainerPort = ContainerPort::Tcp(3993);
const IMAPS_PORT: ContainerPort = ContainerPort::Tcp(3995);
const API_PORT: ContainerPort = ContainerPort::Tcp(8080);

/// Module to work with [`Greemail`] inside of tests.
///
/// Starts an instance of Greenmail.
/// This module is based on the official [`Greenmail docker image`].
///
/// # Example
/// ```
/// use testcontainers_modules::{greenmail, testcontainers::runners::SyncRunner};
///
/// let greenmail_instance = greenmail::Postgres::default().start().unwrap();
///
/// // TODO:
/// ```
///
/// [`Greenmail`]: https://greenmail-mail-test.github.io/greenmail/
/// [`Greenmail docker image`]: https://hub.docker.com/r/greenmail/standalone

#[derive(Debug, Clone)]
pub struct Greenmail {
    env_vars: HashMap<String, String>,
}

impl Greenmail {
    /// Overrides the greenmail options.
    pub fn with_greenmail_opts(mut self, opts: String) -> Self {
        self.env_vars.insert("GREENMAIL_OPTS".to_owned(), opts);
        self
    }

    /// Overrides the java options.
    pub fn with_java_opts(mut self, opts: String) -> Self {
        self.env_vars.insert("JAVA_OPTS".to_owned(), opts);
        self
    }
}
impl Default for Greenmail {
    fn default() -> Self {
        let mut env_vars = HashMap::new();
        env_vars.insert(
            "GREENMAIL_OPTS".to_owned(),
            "-Dgreenmail.setup.test.all -Dgreenmail.hostname=0.0.0.0 -Dgreenmail.auth.disabled"
                .to_owned(),
        );
        env_vars.insert(
            "JAVA_OPTS".to_owned(),
            "-Djava.net.preferIPv4Stack=true".to_owned(),
        );

        Self { env_vars }
    }
}

impl Image for Greenmail {
    fn name(&self) -> &str {
        NAME
    }

    fn tag(&self) -> &str {
        TAG
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stdout(
            "Starting GreenMail API server at",
        )]
    }

    fn env_vars(
        &self,
    ) -> impl IntoIterator<Item = (impl Into<Cow<'_, str>>, impl Into<Cow<'_, str>>)> {
        &self.env_vars
    }

    fn expose_ports(&self) -> &[ContainerPort] {
        &[
            SMTP_PORT, POP3_PORT, IMAP_PORT, SMTPS_PORT, POP3S_PORT, IMAPS_PORT, API_PORT,
        ]
    }
}
