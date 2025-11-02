#pragma once

#include <memory>
#include <string>

#include <speechapi_cxx.h>

#include "rust/cxx.h"

using Microsoft::CognitiveServices::Speech::Translation::SpeechTranslationConfig;

inline std::shared_ptr<SpeechTranslationConfig> from_subscription(
    const rust::Str subscription,
    const rust::Str region
){
    return SpeechTranslationConfig::FromSubscription(
        static_cast<std::string>(subscription),
        static_cast<std::string>(region)
    );
}

inline void add_target_language(
    std::shared_ptr<SpeechTranslationConfig>& config,
    const rust::Str language
){
    config->AddTargetLanguage(static_cast<std::string>(language));
}

inline void set_voice_name(
    std::shared_ptr<SpeechTranslationConfig>& config,
    const rust::Str voice
){
    config->SetVoiceName(static_cast<std::string>(voice));
}