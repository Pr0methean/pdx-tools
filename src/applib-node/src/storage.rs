use napi::{Error, JsArrayBuffer, Task};

pub struct UniversalContainer {
    pub path: String,
}

impl UniversalContainer {
    fn transcode(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(applib::storage::to_universal_container(&self.path)?)
    }
}

impl Task for UniversalContainer {
    type Output = Vec<u8>;
    type JsValue = JsArrayBuffer;

    fn compute(&mut self) -> napi::Result<Self::Output> {
        self.transcode()
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    fn resolve(&mut self, env: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
        Ok(env.create_arraybuffer_with_data(output)?.into_raw())
    }
}
