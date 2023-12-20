use crate::imports::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub enum CoreOps {
    TestBg,
    Shutdown,
    TerminalReady,
    KashdCtl,
    KashdStatus,
    KashdVersion,
    CpuMinerCtl,
    CpuMinerStatus,
    CpuMinerVersion,
    MetricsOpen,
    MetricsClose,
    MetricsReady,
    MetricsCtl,
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct TestReq {
    pub req: String,
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct TestResp {
    pub resp: String,
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub enum DaemonCtl {
    Start,
    Stop,
    Join,
    Restart,
    Kill,
    Mute(bool),
    ToggleMute,
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub enum KashdOps {
    Configure(KashdConfig),
    DaemonCtl(DaemonCtl),
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub enum CpuMinerOps {
    Configure(CpuMinerConfig),
    DaemonCtl(DaemonCtl),
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub enum MetricsCtl {
    Retention(u64),
}

#[derive(Debug, Clone)]
pub struct CoreIpc {
    target: IpcTarget,
}

impl CoreIpc {
    pub fn new(target: IpcTarget) -> CoreIpc {
        CoreIpc { target }
    }

    pub async fn shutdown(&self) -> Result<()> {
        self.target.call(CoreOps::Shutdown, ()).await?;
        Ok(())
    }

    pub async fn metrics_open(&self) -> Result<()> {
        self.target.call(CoreOps::MetricsOpen, ()).await?;
        Ok(())
    }

    pub async fn metrics_close(&self) -> Result<()> {
        self.target.call(CoreOps::MetricsClose, ()).await?;
        Ok(())
    }

    pub async fn metrics_ctl(&self, ctl: MetricsCtl) -> Result<()> {
        self.target.call(CoreOps::MetricsCtl, ctl).await?;
        Ok(())
    }

    pub async fn metrics_ready(&self) -> Result<()> {
        self.target.call(CoreOps::MetricsReady, ()).await?;
        Ok(())
    }

    pub async fn terminal_ready(&self) -> Result<()> {
        self.target.call(CoreOps::TerminalReady, ()).await?;
        Ok(())
    }
}

#[async_trait]
impl KashdCtl for CoreIpc {
    async fn configure(&self, config: KashdConfig) -> DaemonResult<()> {
        // self.target.call::<_, _, ()>(CoreOps::KashdCtl, KashdOps::Configure(config)).await?;
        self.target.call(CoreOps::KashdCtl, KashdOps::Configure(config)).await?;

        Ok(())
    }

    async fn start(&self) -> DaemonResult<()> {
        self.target.call(CoreOps::KashdCtl, KashdOps::DaemonCtl(DaemonCtl::Start)).await?;
        Ok(())
    }

    async fn stop(&self) -> DaemonResult<()> {
        self.target.call(CoreOps::KashdCtl, KashdOps::DaemonCtl(DaemonCtl::Stop)).await?;
        Ok(())
    }

    async fn join(&self) -> DaemonResult<()> {
        self.target.call(CoreOps::KashdCtl, KashdOps::DaemonCtl(DaemonCtl::Join)).await?;
        Ok(())
    }

    async fn restart(&self) -> DaemonResult<()> {
        self.target.call(CoreOps::KashdCtl, KashdOps::DaemonCtl(DaemonCtl::Restart)).await?;
        Ok(())
    }

    async fn kill(&self) -> DaemonResult<()> {
        self.target.call(CoreOps::KashdCtl, KashdOps::DaemonCtl(DaemonCtl::Kill)).await?;
        Ok(())
    }

    async fn status(&self) -> DaemonResult<DaemonStatus> {
        Ok(self.target.call(CoreOps::KashdStatus, ()).await?)
    }

    async fn version(&self) -> DaemonResult<String> {
        Ok(self.target.call(CoreOps::KashdVersion, ()).await?)
    }

    async fn mute(&self, mute: bool) -> DaemonResult<()> {
        self.target.call(CoreOps::KashdCtl, KashdOps::DaemonCtl(DaemonCtl::Mute(mute))).await?;
        Ok(())
    }

    async fn toggle_mute(&self) -> DaemonResult<()> {
        self.target.call(CoreOps::KashdCtl, KashdOps::DaemonCtl(DaemonCtl::ToggleMute)).await?;
        Ok(())
    }
}

#[async_trait]
impl CpuMinerCtl for CoreIpc {
    async fn configure(&self, config: CpuMinerConfig) -> DaemonResult<()> {
        self.target.call(CoreOps::CpuMinerCtl, CpuMinerOps::Configure(config)).await?;

        Ok(())
    }

    async fn start(&self) -> DaemonResult<()> {
        self.target.call(CoreOps::CpuMinerCtl, KashdOps::DaemonCtl(DaemonCtl::Start)).await?;
        Ok(())
    }

    async fn stop(&self) -> DaemonResult<()> {
        self.target.call(CoreOps::CpuMinerCtl, KashdOps::DaemonCtl(DaemonCtl::Stop)).await?;
        Ok(())
    }

    async fn join(&self) -> DaemonResult<()> {
        self.target.call(CoreOps::CpuMinerCtl, KashdOps::DaemonCtl(DaemonCtl::Join)).await?;
        Ok(())
    }

    async fn restart(&self) -> DaemonResult<()> {
        self.target.call(CoreOps::CpuMinerCtl, KashdOps::DaemonCtl(DaemonCtl::Restart)).await?;
        Ok(())
    }

    async fn kill(&self) -> DaemonResult<()> {
        self.target.call(CoreOps::CpuMinerCtl, KashdOps::DaemonCtl(DaemonCtl::Kill)).await?;
        Ok(())
    }

    async fn status(&self) -> DaemonResult<DaemonStatus> {
        Ok(self.target.call(CoreOps::CpuMinerStatus, ()).await?)
    }

    async fn version(&self) -> DaemonResult<String> {
        Ok(self.target.call(CoreOps::CpuMinerVersion, ()).await?)
    }

    async fn mute(&self, mute: bool) -> DaemonResult<()> {
        self.target.call(CoreOps::CpuMinerCtl, KashdOps::DaemonCtl(DaemonCtl::Mute(mute))).await?;
        Ok(())
    }

    async fn toggle_mute(&self) -> DaemonResult<()> {
        self.target.call(CoreOps::CpuMinerCtl, KashdOps::DaemonCtl(DaemonCtl::ToggleMute)).await?;
        Ok(())
    }
}
