#pragma once

#include <memory>
#include <string>

#include <speechapi_cxx.h>

#include "rust/cxx.h"

using Microsoft::CognitiveServices::Speech::Translation::TranslationRecognizer;

inline std::shared_ptr<TranslationRecognizer> form_config(
    std::shared_ptr<SpeechTranslationConfig> speech_config,
    std::shared_ptr<Audio::AudioConfig> audio_input
){
    return TranslationRecognizer::FromConfig(
        speech_config,
        audio_input
    );
}