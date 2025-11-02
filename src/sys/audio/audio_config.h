#pragma once

#include <memory>
#include <string>

#include <speechapi_cxx.h>

#include "rust/cxx.h"

using Microsoft::CognitiveServices::Speech::Audio::AudioConfig;

inline std::shared_ptr<AudioConfig> from_wav_file_input(const std::string& file_name){
    return AudioConfig::FromWavFileInput(file_name);
}