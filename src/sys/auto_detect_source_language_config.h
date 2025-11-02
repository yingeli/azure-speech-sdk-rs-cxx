#pragma once

#include <memory>
#include <string>

#include <speechapi_cxx.h>

#include "rust/cxx.h"

using Microsoft::CognitiveServices::Speech::AutoDetectSourceLanguageConfig;

inline std::shared_ptr<AutoDetectSourceLanguageConfig> from_open_range(){
    return AutoDetectSourceLanguageConfig::FromOpenRange();
}