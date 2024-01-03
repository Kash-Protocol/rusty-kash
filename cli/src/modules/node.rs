use crate::imports::*;
use kash_daemon::KashdConfig;
use workflow_core::task::sleep;
use workflow_node::process;
pub use workflow_node::process::Event;
use workflow_store::fs;

#[derive(Describe, Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum KashdSettings {
    #[describe("Binary location")]
    Location,
    #[describe("Mute logs")]
    Mute,
}

#[async_trait]
impl DefaultSettings for KashdSettings {
    async fn defaults() -> Vec<(Self, Value)> {
        let mut settings = vec![(Self::Mute, to_value(true).unwrap())];

        let root = nw_sys::app::folder();
        if let Ok(binaries) = kash_daemon::locate_binaries(&root, "kashd").await {
            if let Some(path) = binaries.first() {
                settings.push((Self::Location, to_value(path.to_string_lossy().to_string()).unwrap()));
            }
        }

        settings
    }
}

pub struct Node {
    settings: SettingsStore<KashdSettings>,
    mute: Arc<AtomicBool>,
    is_running: Arc<AtomicBool>,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            settings: SettingsStore::try_new("kashd").expect("Failed to create node settings store"),
            mute: Arc::new(AtomicBool::new(true)),
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }
}

#[async_trait]
impl Handler for Node {
    fn verb(&self, ctx: &Arc<dyn Context>) -> Option<&'static str> {
        if let Ok(ctx) = ctx.clone().downcast_arc::<KashCli>() {
            ctx.daemons().clone().kashd.as_ref().map(|_| "node")
        } else {
            None
        }
    }

    fn help(&self, _ctx: &Arc<dyn Context>) -> &'static str {
        "Manage the local Kash node instance"
    }

    async fn start(self: Arc<Self>, _ctx: &Arc<dyn Context>) -> cli::Result<()> {
        self.settings.try_load().await.ok();
        if let Some(mute) = self.settings.get(KashdSettings::Mute) {
            self.mute.store(mute, Ordering::Relaxed);
        }
        Ok(())
    }

    async fn handle(self: Arc<Self>, ctx: &Arc<dyn Context>, argv: Vec<String>, cmd: &str) -> cli::Result<()> {
        let ctx = ctx.clone().downcast_arc::<KashCli>()?;
        self.main(ctx, argv, cmd).await.map_err(|e| e.into())
    }
}

impl Node {
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    async fn create_config(&self, ctx: &Arc<KashCli>) -> Result<KashdConfig> {
        let location: String = self
            .settings
            .get(KashdSettings::Location)
            .ok_or_else(|| Error::Custom("No miner binary specified, please use `miner select` to select a binary.".into()))?;
        let network_id = ctx.wallet().network_id()?;
        // disabled for prompt update (until progress events are implemented)
        // let mute = self.mute.load(Ordering::SeqCst);
        let mute = false;
        let config = KashdConfig::new(location.as_str(), network_id, mute);
        Ok(config)
    }

    async fn main(self: Arc<Self>, ctx: Arc<KashCli>, mut argv: Vec<String>, cmd: &str) -> Result<()> {
        if argv.is_empty() {
            return self.display_help(ctx, argv).await;
        }
        let kashd = ctx.daemons().kashd();
        match argv.remove(0).as_str() {
            "start" => {
                let mute = self.mute.load(Ordering::SeqCst);
                if mute {
                    tprintln!(ctx, "starting kash node... {}", style("(logs are muted, use 'node mute' to toggle)").dim());
                } else {
                    tprintln!(ctx, "starting kash node... {}", style("(use 'node mute' to mute logging)").dim());
                }

                let wrpc_client = ctx.wallet().wrpc_client().ok_or(Error::custom("Unable to start node with non-wRPC client"))?;

                kashd.configure(self.create_config(&ctx).await?).await?;
                kashd.start().await?;

                // temporary setup for auto-connect
                let url = ctx.wallet().settings().get(WalletSettings::Server);
                let network_type = ctx.wallet().network_id()?;
                if let Some(url) = url
                    .map(|url| wrpc_client.parse_url_with_network_type(url, network_type.into()).map_err(|e| e.to_string()))
                    .transpose()?
                {
                    // log_info!("connecting to url: {}", url);
                    if url.contains("127.0.0.1") || url.contains("localhost") {
                        spawn(async move {
                            let options = ConnectOptions {
                                block_async_connect: true,
                                strategy: ConnectStrategy::Fallback,
                                url: Some(url),
                                ..Default::default()
                            };
                            for _ in 0..5 {
                                sleep(Duration::from_millis(1000)).await;
                                if wrpc_client.connect(options.clone()).await.is_ok() {
                                    break;
                                }
                            }
                        });
                    }
                }
            }
            "stop" => {
                kashd.stop().await?;
            }
            "restart" => {
                kashd.configure(self.create_config(&ctx).await?).await?;
                kashd.restart().await?;
            }
            "kill" => {
                kashd.kill().await?;
            }
            "mute" | "logs" => {
                let mute = !self.mute.load(Ordering::SeqCst);
                self.mute.store(mute, Ordering::SeqCst);
                if mute {
                    tprintln!(ctx, "{}", style("node is muted").dim());
                } else {
                    tprintln!(ctx, "{}", style("node is unmuted").dim());
                }
                // kashd.mute(mute).await?;
                self.settings.set(KashdSettings::Mute, mute).await?;
            }
            "status" => {
                let status = kashd.status().await?;
                tprintln!(ctx, "{}", status);
            }
            "select" => {
                let regex = Regex::new(r"(?i)^\s*node\s+select\s+").unwrap();
                let path = regex.replace(cmd, "").trim().to_string();
                self.select(ctx, path.is_not_empty().then_some(path)).await?;
            }
            "version" => {
                kashd.configure(self.create_config(&ctx).await?).await?;
                let version = kashd.version().await?;
                tprintln!(ctx, "{}", version);
            }
            v => {
                tprintln!(ctx, "unknown command: '{v}'\r\n");

                return self.display_help(ctx, argv).await;
            }
        }

        Ok(())
    }

    async fn display_help(self: Arc<Self>, ctx: Arc<KashCli>, _argv: Vec<String>) -> Result<()> {
        ctx.term().help(
            &[
                ("select", "Select Kashd executable (binary) location"),
                ("version", "Display Kashd executable version"),
                ("start", "Start the local Kash node instance"),
                ("stop", "Stop the local Kash node instance"),
                ("restart", "Restart the local Kash node instance"),
                ("kill", "Kill the local Kash node instance"),
                ("status", "Get the status of the local Kash node instance"),
                ("mute", "Toggle log output"),
            ],
            None,
        )?;

        Ok(())
    }

    async fn select(self: Arc<Self>, ctx: Arc<KashCli>, path: Option<String>) -> Result<()> {
        let root = nw_sys::app::folder();

        match path {
            None => {
                let binaries = kash_daemon::locate_binaries(root.as_str(), "kashd").await?;

                if binaries.is_empty() {
                    tprintln!(ctx, "No kashd binaries found");
                } else {
                    let binaries = binaries.iter().map(|p| p.display().to_string()).collect::<Vec<_>>();
                    if let Some(selection) = ctx.term().select("Please select a kashd binary", &binaries).await? {
                        tprintln!(ctx, "selecting: {}", selection);
                        self.settings.set(KashdSettings::Location, selection.as_str()).await?;
                    } else {
                        tprintln!(ctx, "no selection is made");
                    }
                }
            }
            Some(path) => {
                if fs::exists(&path).await? {
                    let version = process::version(&path).await?;
                    tprintln!(ctx, "detected binary version: {}", version);
                    tprintln!(ctx, "selecting: {path}");
                    self.settings.set(KashdSettings::Location, path.as_str()).await?;
                } else {
                    twarnln!(ctx, "destination binary not found, please specify full path including the binary name");
                    twarnln!(ctx, "example: 'node select /home/user/testnet/kashd'");
                    tprintln!(ctx, "no selection is made");
                }
            }
        }

        Ok(())
    }

    pub async fn handle_event(&self, ctx: &Arc<KashCli>, event: Event) -> Result<()> {
        let term = ctx.term();

        match event {
            Event::Start => {
                self.is_running.store(true, Ordering::SeqCst);
                term.refresh_prompt();
            }
            Event::Exit(_code) => {
                tprintln!(ctx, "Kashd has exited");
                self.is_running.store(false, Ordering::SeqCst);
                term.refresh_prompt();
            }
            Event::Error(error) => {
                tprintln!(ctx, "{}", style(format!("Kashd error: {error}")).red());
                self.is_running.store(false, Ordering::SeqCst);
                term.refresh_prompt();
            }
            Event::Stdout(text) | Event::Stderr(text) => {
                if !ctx.wallet().utxo_processor().is_synced() {
                    ctx.wallet().utxo_processor().sync_proc().handle_stdout(&text).await?;
                }

                if !self.mute.load(Ordering::SeqCst) {
                    let sanitize = true;
                    if sanitize {
                        let lines = text.split('\n').collect::<Vec<_>>();
                        lines.into_iter().for_each(|line| {
                            let line = line.trim();
                            if !line.is_empty() {
                                if line.len() < 38 || &line[30..31] != "[" {
                                    term.writeln(line);
                                } else {
                                    let time = &line[11..23];
                                    let kind = &line[31..36];
                                    let text = &line[38..];

                                    match kind {
                                        "WARN " => {
                                            term.writeln(format!("{time} {}", style(text).yellow()));
                                        }
                                        "ERROR" => {
                                            term.writeln(format!("{time} {}", style(text).red()));
                                        }
                                        _ => {
                                            if text.starts_with("Processed") {
                                                term.writeln(format!("{time} {}", style(text).blue()));
                                            } else {
                                                term.writeln(format!("{time} {text}"));
                                            }
                                        }
                                    }
                                }
                            }
                        });
                    } else {
                        term.writeln(text.trim().crlf());
                    }
                }
            }
        }
        Ok(())
    }
}
