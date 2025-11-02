use cxx::SharedPtr;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("azure-speech-sdk-rs/src/sys/translation/speech_translation_config.h");

        type AudioConfig;

        fn from_wav_file_input(file_name: &str) -> SharedPtr<AudioConfig>;
    }
}

pub struct AudioConfig {
    ptr: SharedPtr<ffi::AudioConfig>,
}

impl AudioConfig {
    pub fn from_wav_file_input(file_name: &str) -> Self {
        let ptr = ffi::from_wav_file_input(file_name);
        AudioConfig { ptr }
    }

    pub fn as_ptr(&self) -> &SharedPtr<ffi::AudioConfig> {
        &self.ptr
    }
}

impl Clone for AudioConfig {
    fn clone(&self) -> Self {
        AudioConfig {
            ptr: self.ptr.clone(),
        }
    }
}
