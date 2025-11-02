use cxx::SharedPtr;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("azure-speech-sdk-rs/src/sys/translation/speech_translation_config.h");

        type AutoDetectSourceLanguageConfig;

        fn from_open_range() -> SharedPtr<AutoDetectSourceLanguageConfig>;
    }
}

pub struct AutoDetectSourceLanguageConfig {
    ptr: SharedPtr<ffi::AutoDetectSourceLanguageConfig>,
}

impl AutoDetectSourceLanguageConfig {
    pub fn from_open_range() -> Self {
        let ptr = ffi::from_open_range();
        AutoDetectSourceLanguageConfig { ptr }
    }

    pub fn as_ptr(&self) -> &SharedPtr<ffi::AutoDetectSourceLanguageConfig> {
        &self.ptr
    }
}

impl Clone for AutoDetectSourceLanguageConfig {
    fn clone(&self) -> Self {
        AutoDetectSourceLanguageConfig {
            ptr: self.ptr.clone(),
        }
    }
}
