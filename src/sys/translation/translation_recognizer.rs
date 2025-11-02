use cxx::SharedPtr;

use super::speech_translation_config::{self, SpeechTranslationConfig};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("azure-speech-sdk-rs/src/sys/translation/translation_recognizer.h");

        type SpeechTranslationConfig = super::speech_translation_config::ffi::SpeechTranslationConfig;

        type TranslationRecognizer;

        fn from_config(
            config: SharedPtr<SpeechTranslationConfig>,
        ) -> SharedPtr<TranslationRecognizer>;
    }
}

pub struct TranslationRecognizer {
    ptr: SharedPtr<ffi::TranslationRecognizer>,
}

impl TranslationRecognizer {
    pub fn from_config(config: SpeechTranslationConfig) -> Self {
        let ptr = ffi::from_config(config.to_ptr());
        TranslationRecognizer { ptr }
    }

    pub fn as_ptr(&self) -> &SharedPtr<ffi::TranslationRecognizer> {
        &self.ptr
    }
}

impl Clone for TranslationRecognizer {
    fn clone(&self) -> Self {
        TranslationRecognizer {
            ptr: self.ptr.clone(),
        }
    }
}
