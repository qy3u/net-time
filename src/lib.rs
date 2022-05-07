use anyhow::{Context, Result};
use serde_json::Value;

pub enum Source {
    Taobao,
}

impl Source {
    fn to_url(&self) -> &'static str {
        match self {
            Source::Taobao => "http://api.m.taobao.com/rest/api3.do?api=mtop.common.getTimestamp",
        }
    }

    fn parse(&self, body: &str) -> Result<u64> {
        match self {
            Source::Taobao => {
                let v: Value = serde_json::from_str(body).context("parse taobao msg into json")?;

                let t = v
                    .get("data")
                    .and_then(|v| v.get("t"))
                    .context("get `t` from json taotao returned")?;

                t.as_str()
                    .context("'t' returned by taobao is not a String")?
                    .parse()
                    .context("par `t` into u64")
            }
        }
    }

    pub fn get_timestamp(&self) -> Result<u64> {
        let data = ureq::get(self.to_url())
            .call()
            .with_context(|| format!("call {} failed", self.to_url()))?
            .into_string()
            .with_context(|| {
                format!(
                    "parse data returned by {} into string failed",
                    self.to_url()
                )
            })?;

        self.parse(&data)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ts_from_taobao() {
        println!("{}", Source::Taobao.get_timestamp().unwrap());
    }
}
