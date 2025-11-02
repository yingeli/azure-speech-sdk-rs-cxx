use cxx::SharedPtr;

#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("azure-speech-sdk-rs/src/sys/translation/speech_translation_config.h");

        type SpeechTranslationConfig;

        fn from_subscription(
            subscription: &str,
            region: &str,
        ) -> SharedPtr<SpeechTranslationConfig>;

        fn add_target_language(config: &mut SharedPtr<SpeechTranslationConfig>, language: &str);

        fn set_voice_name(config: &mut SharedPtr<SpeechTranslationConfig>, voice: &str);
    }
}

pub struct SpeechTranslationConfig {
    ptr: SharedPtr<ffi::SpeechTranslationConfig>,
}

impl SpeechTranslationConfig {
    pub fn from_subscription(subscription: &str, region: &str) -> Self {
        let ptr = ffi::from_subscription(subscription, region);
        SpeechTranslationConfig { ptr }
    }

    pub(crate) fn to_ptr(self) -> SharedPtr<ffi::SpeechTranslationConfig> {
        self.ptr
    }

    pub fn add_target_language(&mut self, language: &str) {
        ffi::add_target_language(&mut self.ptr, language);
    }

    pub fn set_voice_name(&mut self, voice: &str) {
        ffi::set_voice_name(&mut self.ptr, voice);
    }
}

impl Clone for SpeechTranslationConfig {
    fn clone(&self) -> Self {
        SpeechTranslationConfig {
            ptr: self.ptr.clone(),
        }
    }
}
